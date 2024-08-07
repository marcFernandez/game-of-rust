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

pub unsafe fn compress_grid_rle() -> Vec<u8> {
    //  GRID = ["0", "0", "1", "0", "0", "0", "1", "0", ...] -> 80 elems (10x8)
    //
    //  For RLE, first byte tells how many bytes represent the "number".
    //  For instance, having RLE = "3o77z":
    //    - [1, 3, 1, 1, 77, 0]
    //       ^ next 1 bytes represent the number of values
    //          ^ 3 of the next value
    //             ^ the value
    //  RLE = [1, 3, 1, 1, 77, 0] -> 6 bytes

    let mut rle = Vec::new();

    // assuming data.len() will always be > 0
    let mut i = 1;
    let mut current_value = GRID[0];
    let mut current_count: usize = 1;

    // [0, 0, 0, 0, 1, 1, 0, 0];
    // [1, 4, 0, 1, 2, 1, 1, 2, 0];

    while i < GRID.len() {
        if GRID[i] == current_value {
            current_count += 1;
        } else {
            if current_count < 256 {
                rle.push(1);
                rle.push(current_count as u8);
            } else if current_count < 256 * 256 {
                rle.push(2);
                rle.push((current_count >> 8) as u8);
                rle.push(current_count as u8);
            }
            rle.push(current_value);
            current_count = 1;
            current_value = GRID[i];
        }
        i += 1;
    }
    if current_count < 256 {
        rle.push(1);
        rle.push(current_count as u8);
    } else if current_count < 256 * 256 {
        rle.push(2);
        rle.push((current_count >> 8) as u8);
        rle.push(current_count as u8);
    }
    rle.push(current_value);

    return rle;
}

pub fn compress_grid_rle_arg(data: &[u8]) -> Vec<u8> {
    //  GRID = ["0", "0", "1", "0", "0", "0", "1", "0", ...] -> 80 elems (10x8)
    //
    //  For RLE, first byte tells how many bytes represent the "number".
    //  For instance, having RLE = "3o77z":
    //    - [1, 3, 1, 1, 77, 0]
    //       ^ next 1 bytes represent the number of values
    //          ^ 3 of the next value
    //             ^ the value
    //  RLE = [1, 3, 1, 1, 77, 0] -> 6 bytes

    let mut rle = Vec::new();

    // assuming data.len() will always be > 0
    let mut i = 1;
    let mut current_value = data[0];
    let mut current_count: usize = 1;

    // [0, 0, 0, 0, 1, 1, 0, 0];
    // [1, 4, 0, 1, 2, 1, 1, 2, 0];

    while i < data.len() {
        if data[i] == current_value {
            current_count += 1;
        } else {
            if current_count < 256 {
                rle.push(1);
                rle.push(current_count as u8);
            } else if current_count < 256 * 256 {
                rle.push(2);
                rle.push((current_count >> 8) as u8);
                rle.push(current_count as u8);
            }
            rle.push(current_value);
            current_count = 1;
            current_value = data[i];
        }
        i += 1;
    }
    if current_count < 256 {
        rle.push(1);
        rle.push(current_count as u8);
    } else if current_count < 256 * 256 {
        rle.push(2);
        rle.push((current_count >> 8) as u8);
        rle.push(current_count as u8);
    }
    rle.push(current_value);

    return rle;
}

#[cfg(test)]
mod tests {
    use crate::net::compress_grid_rle_arg;

    #[test]
    fn test_rle() {
        let data = [0, 0, 0, 1, 1, 0, 0];
        let rle_data = [1, 3, 0, 1, 2, 1, 1, 2, 0];

        let result = compress_grid_rle_arg(&data);

        assert_eq!(result, rle_data);
    }

    #[test]
    fn test_rle_two_bytes() {
        let mut data: Vec<u8> = vec![0; 259];
        data[257] = 1;
        let rle_data = [2, (257 >> 8) as u8, (257 & 0xFF) as u8, 0, 1, 1, 1, 1, 1, 0];

        let result = compress_grid_rle_arg(&data);

        assert_eq!(result, rle_data);
    }
}

pub unsafe fn uncompress_grid_binary(cgrid: &[u8]) {
    for (i, byte) in cgrid.iter().enumerate() {
        for bit in 0..8 {
            GRID[(i * 8) + bit] = (byte >> bit) & 0x01;
        }
    }
    for i in 0..GRID_HEIGHT {
        eprintln!("{:?}", &GRID[(i * GRID_WIDTH)..((i * GRID_WIDTH) + GRID_WIDTH)]);
    }
}

pub unsafe fn uncompress_grid_rle(cgrid: &[u8]) {
    let mut i = 0;
    let mut grid_idx: usize = 0;
    let mut len_bytes;
    let mut value_count: u16;
    let mut value: u8;

    eprintln!("uncompress_grid_rle: {cgrid:?}");

    while i < cgrid.len() {
        len_bytes = cgrid[i];
        i += 1;
        value_count = cgrid[i] as u16;
        i += 1;

        if len_bytes == 1 {
        } else if len_bytes == 2 {
            value_count = (value_count << 8) | (cgrid[i] as u16);
            i += 1;
        } else {
            panic!("Unsupported length for count: {len_bytes}");
        }

        value = cgrid[i];
        i += 1;

        eprintln!(
            "Received {value_count} {value}s from [{grid_idx}, {}]",
            grid_idx + (value_count as usize)
        );

        for _ in 0..value_count {
            GRID[grid_idx] = value;
            grid_idx += 1;
        }
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
    let _four_bytes: [u8; 4] = [0; 4];

    let fin: u8;
    let rsv1: u8;
    let rsv2: u8;
    let rsv3: u8;
    let opcode: u8;
    let masked: u8;
    let mut content_length: u64;
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

pub fn send_ws_msg(stream: &mut TcpStream, cmd: &[u8], size: &[u8], data: &[u8]) -> Result<usize, std::io::Error> {
    let mut response: Vec<u8> = Vec::new();

    let header: u8 = 0b10000010;
    response.push(header);

    let content_length: u64 = (cmd.len() + size.len() + data.len()) as u64;

    // Server must send unmasked (mask=0) messages
    if content_length < 126 {
        response.push(content_length as u8);
    } else if content_length < 1_048_576 {
        response.push(126);
        response.push((content_length >> 8) as u8);
        response.push(content_length as u8);
    } else if content_length < u64::MAX {
        response.push(127);
        todo!("I don't think that'll be needed")
    } else {
        todo!("Split message into several frames")
    }

    cmd.iter().for_each(|u| {
        response.push(*u);
    });
    size.iter().for_each(|u| {
        response.push(*u);
    });
    data.iter().for_each(|u| {
        response.push(*u);
    });

    stream.write_all(&response)?;

    return Ok(response.len());
}

pub fn send_ws_msg_text(stream: &mut TcpStream, message: &str) -> Result<usize, std::io::Error> {
    let mut response: Vec<u8> = Vec::new();

    let header: u8 = 0b10000010;
    response.push(header);

    let content_length: u64 = (CMD_HEADER_SIZE + SIZE_HEADER_SIZE + message.len()) as u64;

    // Server must send unmasked (mask=0) messages
    if content_length < 126 {
        response.push(content_length as u8);
    } else if content_length < 1_048_576 {
        response.push(126);
        response.push((content_length >> 8) as u8);
        response.push(content_length as u8);
    } else if content_length < u64::MAX {
        response.push(127);
        todo!("I don't think that'll be needed")
    } else {
        todo!("Split message into several frames")
    }

    response.push(CMD_LOG_MSG);
    // TODO: probably it makes sense to error if len does not fit in 16 bits
    [(message.len() >> 8) as u8, message.len() as u8].iter().for_each(|u| {
        response.push(*u);
    });

    eprint!("\nSending: ");
    message.chars().for_each(|c| {
        eprint!("{c}");
        response.push(c as u8);
    });
    eprint!("\n");

    eprintln!("{:?}", response);
    stream.write_all(&response)?;

    return Ok(response.len());
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

pub fn write_data_to_stream(stream: &mut TcpStream, data: &[u8]) -> Result<usize, std::io::Error> {
    return match stream.write(&data) {
        Ok(n) if n == 0 => panic!("write call returned 0"),
        Ok(n) if n < data.len() => todo!("write call was unable to write all bytes"),
        Ok(n) if n == data.len() => Ok(n),
        Err(e) => Err(e),
        _ => unreachable!("Neither an error nor an expected num of bytes were returned in the write call"),
    };
}
