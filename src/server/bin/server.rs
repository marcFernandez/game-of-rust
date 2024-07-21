#![allow(static_mut_refs)]

use std::io::{Result, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{poll, read, Event, KeyCode};
use gol_multi::game::{create_state, next_grid, State, GRID, GRID_HEIGHT, GRID_WIDTH, MS_PER_FRAME, PREV_GRID};
use gol_multi::net::{CMD_HEADER_SIZE, CMD_LOG_MSG, CMD_NEW_GRID, SIZE_HEADER_SIZE};
use gol_multi::term::{end_terminal, render, render_debug_data, reset_terminal, start_terminal};

static mut ACTIVE_CONNECTIONS: u64 = 0;

fn main() -> Result<()> {
    println!("Hello, server!");

    let streams: Arc<Mutex<Vec<Mutex<TcpStream>>>> = Arc::new(Mutex::new(Vec::with_capacity(10)));

    unsafe {
        let streams_clone = Arc::clone(&streams);
        thread::spawn(move || {
            let listener = TcpListener::bind("0.0.0.0:42069").unwrap();
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

    let mut cmd_msg: [u8; CMD_HEADER_SIZE];
    let mut size_msg: [u8; SIZE_HEADER_SIZE];

    let mut send_msg = false;
    let log_msg = "This is a test log message";
    let log_msg_size: u16 = log_msg.len().try_into().expect("Size must fit in 16bits");

    loop {
        clock = Instant::now();
        if poll(Duration::from_millis((MS_PER_FRAME as f64 * 0.5) as u64))? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => {
                        exit = true;
                    }
                    KeyCode::Char('m') => {
                        send_msg = true;
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

        //grid_str = &GRID.iter().map(|val| val.to_string()).collect::<String>();
        if send_msg {
            cmd_msg = CMD_LOG_MSG.to_be_bytes();
            size_msg = [(log_msg_size >> 8 & 0xFF) as u8, (log_msg_size & 0xFF) as u8];
        } else {
            cmd_msg = CMD_NEW_GRID.to_be_bytes();
            size_msg = ((GRID_WIDTH * GRID_HEIGHT) as u16).to_be_bytes();
        }
        eprintln!("Sending cmd_msg: {:?}", cmd_msg);
        eprintln!("Sending size_msg: {:?}", size_msg);
        eprintln!("Sending grid_msg: {:?}", &GRID);

        //eprintln!("Grid: {:?}", grid_str);
        streams.lock().unwrap().iter().for_each(|stream| {
            let mut stream_lock = stream.lock().unwrap();
            let peer_addr = stream_lock.peer_addr().expect("Peer addr to be available");
            eprintln!("Sending to {peer_addr}");
            let _ = stream_lock.write(&cmd_msg);
            let _ = stream_lock.write(&size_msg);
            if send_msg {
                let _ = stream_lock.write(&log_msg.as_bytes());
            } else {
                // TODO: Send grid as bit per cell instead of byte per cell
                let _ = stream_lock.write(&GRID);
            }
            let _ = stream_lock.flush();
            eprintln!("Sent to {peer_addr}")
        });
        send_msg = false;
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
