use gol_multi::{
    game::{create_state, GRID_HEIGHT, GRID_WIDTH},
    term::render,
};

use core::panic;
use std::{
    io::{BufRead, BufReader, Read, Result, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<()> {
    let state = create_state();
    unsafe {
        let listener = TcpListener::bind("0.0.0.0:42069").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established from {}", stream.peer_addr().unwrap());
            handle_connection(stream)?;
        }
        render(&state)?;
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut tmp: String = String::new();
    let mut content_length: Option<u64> = None;
    loop {
        let bytes = buf_reader.read_line(&mut tmp).unwrap();
        if tmp.starts_with("Content-Length") {}
        println!("read {bytes} bytes");
        if bytes < 3 {
            break;
        }
        if tmp.starts_with("Content-Length") {
            let cl = tmp.split(": ").last().unwrap();
            content_length = Some(cl.lines().next().unwrap().parse::<u64>().unwrap());
        }
        tmp.clear();
    }

    let content_length = if let Some(c) = content_length {
        c
    } else {
        panic!("Expected Content-Length header")
    };

    println!("Content-Length: {content_length}");
    let mut buffer = vec![0; (content_length - 0).try_into().unwrap()];
    buf_reader.read_exact(&mut buffer)?;

    let grid_data = String::from_utf8(buffer).unwrap();

    println!("Grid: {}", grid_data);
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}
