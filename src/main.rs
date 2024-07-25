use core::panic;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use base64::{engine::general_purpose, Engine};
use sha1::{Digest, Sha1};

const MAGIC: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
const RESP: &str = "HTTP/1.1 101 Switching Protocols
Upgrade: websocket
Connection: Upgrade
Sec-WebSocket-Accept: ";

fn main() {
    let listener = TcpListener::bind("0.0.0.0:42069").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        eprintln!("Connection established from {}", stream.peer_addr().unwrap());
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
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

    println!("Request: {http_request:#?}");
    println!("Key: {key}");
    let mut hasher = Sha1::new();
    hasher.update(format!("{key}{MAGIC}"));
    let result = hasher.finalize();
    println!("sha1(key+MAGIC): {:?}", result);
    let bkey = general_purpose::STANDARD.encode(result);

    let response = format!("{}{}\n\n", RESP, bkey).replace("\n", "\r\n");
    println!("Sending:\n{response}");

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Sent handshake response");

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

    loop {
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
            println!("Content length is full");
            content_length
        } else if content_length == 126 {
            stream.read_exact(&mut two_bytes).unwrap();
            todo!("Parse more than 7bit length properly");
        } else if content_length == 127 {
            todo!("Parse more than 7bit length properly");
        } else {
            panic!("This is impossible so idk");
        };

        println!("fin: {}", fin);
        println!("rsv1: {rsv1}");
        println!("rsv2: {rsv2}");
        println!("rsv3: {rsv3}");
        println!("opcode: {opcode}");
        println!("masked: {masked}");
        println!("content_length: {content_length}");

        if masked > 0 {
            stream.read_exact(&mut mask).unwrap();
            println!("mask: {:?}", mask);
        }

        println!("Reading data");
        stream.read_exact(&mut data[0..(content_length as usize)]).unwrap();
        println!("raw_data: {:?}", data);

        for i in 0..content_length {
            data[i as usize] ^= mask[(i % 4) as usize];
        }

        let string = String::from_utf8(data[0..(content_length as usize)].to_vec()).unwrap();
        println!("data: {string}");

        if string.len() > 128 - "echo: ".len() {
            todo!("Handle echo of larger chunks of data");
        }

        send_msg(&mut stream, &string.as_bytes());
    }
}

fn send_msg(stream: &mut TcpStream, data: &[u8]) {
    let header: u8 = 0b10000010;
    // Server must send unmasked (mask=0) messages
    let masked_and_content_length: u8 = (data.len() + "echo: ".len()) as u8;

    let mut response: Vec<u8> = Vec::new();

    response.push(header);
    response.push(masked_and_content_length);
    "echo: ".as_bytes().iter().for_each(|c| {
        response.push(*c);
    });
    data.iter().for_each(|u| {
        response.push(*u);
    });

    stream.write_all(&response).expect("Data to be sent");
}
