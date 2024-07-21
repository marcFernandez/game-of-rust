use gol_multi::{
    game::GRID,
    net::{CMD_HEADER_SIZE, CMD_LOG_MSG, CMD_NEW_GRID, MAX_CONTENT_SIZE, SIZE_HEADER_SIZE},
    term::{clear_terminal, render},
};
use std::{
    env::args,
    io::{Read, Result},
    net::TcpStream,
    process::exit,
};

fn main() -> Result<()> {
    let mut args = args().skip(1);
    let mut host = String::from("0.0.0.0");
    let mut port = String::from("42069");
    while let Some(next) = args.next() {
        match next.as_str() {
            "-p" => match args.next() {
                Some(p) => {
                    port = p;
                }
                None => {
                    eprintln!("ERROR - Port expected after flag -p");
                    exit(1);
                }
            },
            "-h" => match args.next() {
                Some(h) => {
                    host = h;
                }
                None => {
                    eprintln!("ERROR - Host expected after flag -h");
                    exit(1);
                }
            },
            _ => {
                eprintln!("Unrecognized arg: {}", next);
                exit(1)
            }
        }
    }
    unsafe {
        let stream = TcpStream::connect(format!("{host}:{port}")).unwrap();
        handle_connection(stream)?;
    }

    Ok(())
}

unsafe fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut header_buffer: [u8; CMD_HEADER_SIZE] = [0; CMD_HEADER_SIZE];
    let mut size_buffer: [u8; SIZE_HEADER_SIZE] = [0; SIZE_HEADER_SIZE];
    let mut content_buffer: [u8; MAX_CONTENT_SIZE] = [0; MAX_CONTENT_SIZE];
    let mut content_size: u16;
    let mut log: String;
    clear_terminal()?;
    loop {
        // Read cmd header (1 byte)
        println!("Reading header");
        stream.read_exact(&mut header_buffer)?;
        println!("Header: {:#04x}", header_buffer[0]);
        // Read size header (2 bytes) big-endian
        println!("Reading size");
        stream.read_exact(&mut size_buffer)?;
        content_size = (size_buffer[0] as u16) << 8 | size_buffer[1] as u16;
        println!("Size: {}B", content_size);

        // Read to fill slice of max-sized content buffer based on content_size
        println!("Reading contents");
        stream.read_exact(&mut content_buffer[0..content_size.into()])?;
        println!("Read content");

        match header_buffer[0] {
            CMD_NEW_GRID => {
                println!("Received new grid");
                GRID.copy_from_slice(&content_buffer[0..content_size.into()]);
                render()?;
            }
            CMD_LOG_MSG => {
                log = String::from_utf8(content_buffer[0..content_size.into()].to_vec())
                    .expect("Log message to be valid utf8");
                println!("Received log msg: {log}");
            }
            _ => {
                println!("header_buffer[0] = {} did not match anything", header_buffer[0]);
            }
        }
    }
}
