use gol_multi::{
    game::{GRID, GRID_HEIGHT, GRID_WIDTH},
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
    const BUFF_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;
    let mut buffer: [u8; BUFF_SIZE] = [0; BUFF_SIZE];
    clear_terminal()?;
    loop {
        stream.read_exact(&mut buffer)?;
        let grid_data = String::from_utf8(buffer.to_vec()).unwrap();

        grid_data.chars().enumerate().for_each(|(i, val)| {
            GRID[i] = val.to_string().parse().unwrap();
        });
        render()?;
    }
}
