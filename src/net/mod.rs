use crate::game::{GRID, GRID_HEIGHT, GRID_WIDTH};

// PACKET
// [8bit]  [16bit ]                [       ]
// [header][content_size max=65535][content]
//
// 8bit header allows 4 commands
//   - 0000: New grid
//   - 0001: Log msg
//   - 0010: Dimensions
//   - 0011: Unused
//   - ...
//   - 1111: Unused
pub const CMD_NEW_GRID: u8 = 0;
pub const CMD_LOG_MSG: u8 = 1;
pub const CMD_GRID_DIMENSIONS: u8 = 2;

// sizes are represented in Bytes
pub const MAX_CONTENT_SIZE: usize = 65536;
pub const CMD_HEADER_SIZE: usize = 1;
pub const SIZE_HEADER_SIZE: usize = 2;

// CMD       | SIZE                | CONTENT
// 0000 0000 | 0000 0000 0000 0000 | 0000 ... 0000
//
// New grid of 10x10 cells
// CMD(0)    | SIZE = 100          | CONTENT = grid serialized (?)
// 0000 0000 | 0000 0000 0110 0100 | 0000 ... 0000

pub unsafe fn compress_grid() -> [u8; (GRID_WIDTH * GRID_HEIGHT) / 8] {
    //  GRID = ["0", "0", "1", "0", "0", "0", "1", "0", ...] -> 80 elems (10x8)
    // bytes = [x30, x30, x31, x30, x30, x30, x31, x30, ...] -> 80 bytes (10x8)
    // CGRID = [b00100010, ...] -> 80bits -> 10 bytes (10x8)

    let mut current_byte: u8;
    let mut compressed_grid: [u8; (GRID_WIDTH * GRID_HEIGHT) / 8] = [0; (GRID_WIDTH * GRID_HEIGHT) / 8];
    let mut g = 0;
    for byte in 0..((GRID_WIDTH * GRID_HEIGHT) / 8) {
        current_byte = 0;
        for i in 0..8 {
            current_byte |= GRID[g] << i;
            g += 1;
        }
        compressed_grid[byte] = current_byte;
    }
    return compressed_grid;
}

pub unsafe fn uncompress_grid(cgrid: &[u8]) {
    for (i, byte) in cgrid.iter().enumerate() {
        for bit in 0..8 {
            GRID[(i * 8) + bit] = (byte >> bit) & 0x01;
        }
    }
    for i in 0..GRID_HEIGHT {
        eprintln!("{:?}", &GRID[(i * GRID_WIDTH)..((i * GRID_WIDTH) + GRID_WIDTH)]);
    }
}

/***** WS STUFF ******/

use core::panic;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
};

use base64::{engine::general_purpose, Engine};
use sha1::{Digest, Sha1};

const MAGIC: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
const RESP: &str = "HTTP/1.1 101 Switching Protocols
Upgrade: websocket
Connection: Upgrade
Sec-WebSocket-Accept: ";

pub fn handle_ws_connection(mut stream: TcpStream) -> TcpStream {
    let buf_reader = BufReader::new(&mut stream);
    let mut key = String::new();
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| {
            let l = result.unwrap();
            if l.starts_with("Sec-WebSocket-Key") {
                key = l.split(": ").last().unwrap().to_string();
            }
            return l;
        })
        .take_while(|line| !line.is_empty())
        .collect();

    eprintln!("Request: {http_request:#?}");
    eprintln!("Key: {key}");
    let mut hasher = Sha1::new();
    hasher.update(format!("{key}{MAGIC}"));
    let result = hasher.finalize();
    eprintln!("sha1(key+MAGIC): {:?}", result);
    let bkey = general_purpose::STANDARD.encode(result);

    let response = format!("{}{}\n\n", RESP, bkey).replace("\n", "\r\n");
    eprintln!("Sending:\n{response}");

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    eprintln!("Sent handshake response");

    let mut one_byte: [u8; 1] = [0; 1];
    let mut two_bytes: [u8; 2] = [0; 2];
    let mut four_bytes: [u8; 4] = [0; 4];

    let mut fin: u8 = 0;
    let mut rsv1: u8 = 0;
    let mut rsv2: u8 = 0;
    let mut rsv3: u8 = 0;
    let mut opcode: u8 = 0;
    let mut masked: u8 = 0;
    let mut content_length: u64 = 0;
    let mut mask: [u8; 4] = [0; 4];

    let mut data: [u8; 128] = [0; 128];

    stream.read_exact(&mut one_byte).unwrap();

    fin = (one_byte[0] & 0b10000000) >> 7;
    rsv1 = one_byte[0] & 0b01000000;
    rsv2 = one_byte[0] & 0b00100000;
    rsv3 = one_byte[0] & 0b00010000;
    opcode = one_byte[0] & 0b00001111;

    stream.read_exact(&mut one_byte).unwrap();

    masked = (one_byte[0] & 0b10000000) >> 7;
    content_length = (one_byte[0] & 0b01111111) as u64;

    content_length = if content_length < 126 {
        eprintln!("Content length is full");
        content_length
    } else if content_length == 126 {
        stream.read_exact(&mut two_bytes).unwrap();
        todo!("Parse more than 7bit length properly");
    } else if content_length == 127 {
        todo!("Parse more than 7bit length properly");
    } else {
        panic!("This is impossible so idk");
    };

    eprintln!("fin: {}", fin);
    eprintln!("rsv1: {rsv1}");
    eprintln!("rsv2: {rsv2}");
    eprintln!("rsv3: {rsv3}");
    eprintln!("opcode: {opcode}");
    eprintln!("masked: {masked}");
    eprintln!("content_length: {content_length}");

    if masked > 0 {
        stream.read_exact(&mut mask).unwrap();
        eprintln!("mask: {:?}", mask);
    }

    eprintln!("Reading data");
    stream.read_exact(&mut data[0..(content_length as usize)]).unwrap();
    eprintln!("raw_data: {:?}", data);

    for i in 0..content_length {
        data[i as usize] ^= mask[(i % 4) as usize];
    }

    let string = String::from_utf8(data[0..(content_length as usize)].to_vec()).unwrap();
    eprintln!("data: {string}");

    if string.len() > 128 - "echo: ".len() {
        todo!("Handle echo of larger chunks of data");
    }

    eprintln!("Sending dimensions");
    send_dimensions(&mut stream);
    eprintln!("Sent dimensions");
    return stream;
}

pub fn send_ws_msg(stream: &mut TcpStream, cmd: &[u8], size: &[u8], data: &[u8]) {
    let header: u8 = 0b10000010;
    // Server must send unmasked (mask=0) messages
    let masked_and_content_length: u8 = (cmd.len() + size.len() + data.len()) as u8;

    let mut response: Vec<u8> = Vec::new();

    response.push(header);
    response.push(masked_and_content_length);
    cmd.iter().for_each(|u| {
        response.push(*u);
    });
    size.iter().for_each(|u| {
        response.push(*u);
    });
    data.iter().for_each(|u| {
        response.push(*u);
    });

    stream.write_all(&response).expect("Data to be sent");
}

const DIMENSIONS_MSG_LEN: u8 = 7;
pub fn send_dimensions(stream: &mut TcpStream) {
    let header: u8 = 0b10000010;

    // Server must send unmasked (mask=0) messages, which leaves 7bits for
    // size
    let masked_and_content_length: u8 = DIMENSIONS_MSG_LEN;

    // Sending two u16 (assuming 16bit dimensions are enough). That totals
    // CMD + SIZE  + DATA
    // u8  + 2*u8 + 2*u16 = [u8; 7]
    let data: [u8; DIMENSIONS_MSG_LEN as usize] = [
        CMD_GRID_DIMENSIONS,
        0,
        4,
        (GRID_WIDTH >> 8) as u8,
        (GRID_WIDTH & 0xFF) as u8,
        (GRID_HEIGHT >> 8) as u8,
        (GRID_HEIGHT & 0xFF) as u8,
    ];

    let msg: [u8; DIMENSIONS_MSG_LEN as usize + 2] = [
        header,
        masked_and_content_length,
        data[0],
        data[1],
        data[2],
        data[3],
        data[4],
        data[5],
        data[6],
    ];

    stream.write_all(&msg).expect("Data to be sent");
}
