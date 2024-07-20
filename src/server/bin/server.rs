use std::io::{BufRead, BufReader, Result};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{poll, read, Event, KeyCode};
use gol_multi::game::{create_state, next_grid, State, GRID, GRID_WIDTH, MS_PER_FRAME, PREV_GRID};
use gol_multi::term::{end_terminal, render, reset_terminal, start_terminal};

fn main() -> Result<()> {
    println!("Hello, server!");

    thread::spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:42069").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established from {}", stream.peer_addr().unwrap());
            handle_connection(stream);
        }
    });

    let state: State = create_state();

    unsafe {
        run(state)?;
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}

unsafe fn run(mut state: State) -> Result<()> {
    start_terminal()?;
    /* GLIDER */
    GRID[1 + GRID_WIDTH * 8] = 1;
    GRID[2 + GRID_WIDTH * 8] = 1;
    GRID[3 + GRID_WIDTH * 8] = 1;
    GRID[2 + GRID_WIDTH * 6] = 1;
    GRID[3 + GRID_WIDTH * 7] = 1;

    PREV_GRID[1 + GRID_WIDTH * 8] = 1;
    PREV_GRID[2 + GRID_WIDTH * 8] = 1;
    PREV_GRID[3 + GRID_WIDTH * 8] = 1;
    PREV_GRID[2 + GRID_WIDTH * 6] = 1;
    PREV_GRID[3 + GRID_WIDTH * 7] = 1;

    /* STICK
    GRID[1][3] = 1;
    GRID[2][3] = 1;
    GRID[3][3] = 1;

    PREV_GRID[1][3] = 1;
    PREV_GRID[2][3] = 1;
    PREV_GRID[3][3] = 1;
    */

    let mut clock;
    let mut exit = false;
    loop {
        clock = Instant::now();
        if poll(Duration::from_millis((MS_PER_FRAME as f64 * 0.5) as u64))? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => {
                        exit = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        if exit {
            reset_terminal()?;
            break;
        }

        render(&state)?;
        next_grid(&state);
        let _ = send_grid(&state);
        state.frame = state.frame + 1;
        if state.frame == 2000 {
            break;
        }
        let diff = Duration::from_millis(MS_PER_FRAME as u64) - Instant::now().duration_since(clock);
        if diff.as_millis() > 0 {
            thread::sleep(diff);
        }
    }
    end_terminal()?;
    return Ok(());
}

fn send_grid(_state: &State) -> Result<()> {
    Ok(())
}
