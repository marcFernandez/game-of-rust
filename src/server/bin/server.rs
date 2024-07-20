use std::io::{Result, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{poll, read, Event, KeyCode};
use gol_multi::game::{create_state, next_grid, State, GRID, GRID_WIDTH, MS_PER_FRAME, PREV_GRID};
use gol_multi::term::{end_terminal, render, render_debug_data, reset_terminal, start_terminal};

static mut ACTIVE_CONNECTIONS: u64 = 0;

fn main() -> Result<()> {
    println!("Hello, server!");

    let listener = TcpListener::bind("0.0.0.0:42069").unwrap();
    let streams: Arc<Mutex<Vec<Mutex<TcpStream>>>> = Arc::new(Mutex::new(Vec::with_capacity(10)));

    unsafe {
        let streams_clone = Arc::clone(&streams);
        thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                eprintln!("Connection established from {}", stream.peer_addr().unwrap());
                ACTIVE_CONNECTIONS = ACTIVE_CONNECTIONS + 1;
                streams_clone.lock().unwrap().push(Mutex::new(stream));
            }
        });

        let streams_clone2 = Arc::clone(&streams);
        let state: State = create_state();
        run(state, streams_clone2)?;
    }

    Ok(())
}

unsafe fn run(mut state: State, streams: Arc<Mutex<Vec<Mutex<TcpStream>>>>) -> Result<()> {
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

        render()?;
        render_debug_data(true, &state, &ACTIVE_CONNECTIONS)?;
        let grid_str: &str = &GRID.iter().map(|val| val.to_string()).collect::<String>();
        //eprintln!("Grid: {:?}", grid_str);
        streams.lock().unwrap().iter().for_each(|stream| {
            let mut stream_lock = stream.lock().unwrap();
            let _ = stream_lock.write(grid_str.as_bytes());
            let _ = stream_lock.flush();
        });
        next_grid(&state);
        let _ = send_grid(&state);
        state.frame = state.frame + 1;
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
