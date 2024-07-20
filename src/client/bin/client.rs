use gol_multi::{
    game::{GRID, GRID_HEIGHT, GRID_WIDTH},
    term::{clear_terminal, render},
};
use std::{
    io::{Read, Result},
    net::TcpStream,
};

fn main() -> Result<()> {
    unsafe {
        let stream = TcpStream::connect("0.0.0.0:42069").unwrap();
        println!("Connection established from {}", stream.peer_addr().unwrap());
        handle_connection(stream)?;
    }

    Ok(())
}

unsafe fn handle_connection(mut stream: TcpStream) -> Result<()> {
    const BUFF_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;
    let mut buffer: [u8; BUFF_SIZE] = [0; BUFF_SIZE];
    clear_terminal()?;
    println!("Starting the loop");
    loop {
        stream.read_exact(&mut buffer)?;
        let grid_data = String::from_utf8(buffer.to_vec()).unwrap();
        eprintln!("Grid: {}", grid_data);

        grid_data.chars().enumerate().for_each(|(i, val)| {
            GRID[i] = val.to_string().parse().unwrap();
        });
        render()?;
    }
}
