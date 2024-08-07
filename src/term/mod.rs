#![allow(dead_code)]

use std::io::{stdout, Result, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    QueueableCommand,
};

use crate::game::{State, CELL, FPS, GRID, GRID_HEIGHT, GRID_WIDTH};
// Status
pub const RST: &str = "\x1b[0m";

// Foreground color
pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";
pub const DEFAULT: &str = "\x1b[39m";

pub const BRIGHT_BLACK: &str = "\x1b[90m";
pub const BRIGHT_RED: &str = "\x1b[91m";
pub const BRIGHT_GREEN: &str = "\x1b[92m";
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
pub const BRIGHT_BLUE: &str = "\x1b[94m";
pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
pub const BRIGHT_CYAN: &str = "\x1b[96m";
pub const BRIGHT_WHITE: &str = "\x1b[97m";

// Background color
pub const BLACK_BG: &str = "\x1b[40m";
pub const RED_BG: &str = "\x1b[41m";
pub const GREEN_BG: &str = "\x1b[42m";
pub const YELLOW_BG: &str = "\x1b[43m";
pub const BLUE_BG: &str = "\x1b[44m";
pub const MAGENTA_BG: &str = "\x1b[45m";
pub const CYAN_BG: &str = "\x1b[46m";
pub const WHITE_BG: &str = "\x1b[47m";
pub const DEFAULT_BG: &str = "\x1b[49m";

pub const BRIGHT_BLACK_BG: &str = "\x1b[100m";
pub const BRIGHT_RED_BG: &str = "\x1b[101m";
pub const BRIGHT_GREEN_BG: &str = "\x1b[102m";
pub const BRIGHT_YELLOW_BG: &str = "\x1b[103m";
pub const BRIGHT_BLUE_BG: &str = "\x1b[104m";
pub const BRIGHT_MAGENTA_BG: &str = "\x1b[105m";
pub const BRIGHT_CYAN_BG: &str = "\x1b[106m";
pub const BRIGHT_WHITE_BG: &str = "\x1b[107m";

pub fn reset_terminal() -> Result<()> {
    let mut stdout = stdout();
    disable_raw_mode()?;
    stdout.queue(Show)?;
    stdout.queue(Clear(ClearType::All))?;
    stdout.flush()
}

pub fn start_terminal() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.queue(Clear(ClearType::All))?;
    stdout.queue(Hide)?;
    stdout.flush()?;
    Ok(())
}

pub fn clear_terminal() -> Result<()> {
    let mut stdout = stdout();
    stdout.queue(Clear(ClearType::All))?;
    stdout.queue(Hide)?;
    stdout.flush()?;
    Ok(())
}

pub fn end_terminal() -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = stdout();
    stdout.queue(Show)?;
    stdout.flush()?;
    Ok(())
}

pub unsafe fn render() -> Result<()> {
    let mut stdout = stdout();
    for y in 0..(GRID_HEIGHT as usize) {
        for x in 0..(GRID_WIDTH as usize) {
            stdout.queue(MoveTo((x * 2) as u16, y as u16))?;
            if GRID[x + GRID_WIDTH * y] == 1 {
                stdout.write(WHITE_BG.as_bytes())?;
                stdout.write(CELL.as_bytes())?;
                stdout.write(RST.as_bytes())?;
            } else {
                stdout.write(RST.as_bytes())?;
                stdout.write(CELL.as_bytes())?;
                stdout.write(RST.as_bytes())?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}

#[derive(Debug)]
pub struct DebugData {
    pub active_connections: u64,
}

pub unsafe fn render_debug_data(active: bool, state: &State, active_connections: &u64) -> Result<()> {
    if !active {
        return Ok(());
    }
    let mut stdout = stdout();
    stdout.queue(MoveTo(1 as u16, (GRID_HEIGHT + 3) as u16))?;
    stdout.write(format!("active_connections: {active_connections}\n").as_bytes())?;
    stdout.queue(MoveTo(1 as u16, (GRID_HEIGHT + 4) as u16))?;
    stdout.write(format!("time_running: {}s", state.frames / FPS).as_bytes())?;
    stdout.queue(MoveTo(1 as u16, (GRID_HEIGHT + 5) as u16))?;
    stdout.write(format!("frames: {}", state.frames).as_bytes())?;
    stdout.queue(MoveTo(1 as u16, (GRID_HEIGHT + 6) as u16))?;
    stdout.write(format!("total_bytes_sent: {} B", state.total_bytes_sent).as_bytes())?;
    stdout.queue(MoveTo(1 as u16, (GRID_HEIGHT + 7) as u16))?;
    stdout.write(format!("total_messages_sent: {}", state.total_messages_sent).as_bytes())?;
    if state.total_messages_sent > 0 {
        stdout.queue(MoveTo(1 as u16, (GRID_HEIGHT + 8) as u16))?;
        stdout.write(
            format!(
                "avg_message_bytes: {} B",
                state.total_bytes_sent / state.total_messages_sent
            )
            .as_bytes(),
        )?;
    }
    if state.encoded_grid_lengths.len() > 0 {
        stdout.queue(MoveTo(1 as u16, (GRID_HEIGHT + 9) as u16))?;
        stdout.write(
            format!(
                "avg_grid_msg_bytes: {} B",
                state.encoded_grid_lengths.iter().sum::<usize>() / state.encoded_grid_lengths.len()
            )
            .as_bytes(),
        )?;
    }
    eprintln!("encoded_grid_lengths: {:?}", state.encoded_grid_lengths);
    Ok(())
}

pub unsafe fn render_txt() -> Result<()> {
    let mut stdout = stdout();
    for y in 0..GRID_HEIGHT as u16 {
        stdout.queue(MoveTo(0, y))?;
        for x in 0..(GRID_WIDTH as usize) {
            stdout.write(format!("{} ", GRID[x as usize + GRID_WIDTH * y as usize]).as_bytes())?;
        }
    }
    stdout.flush()?;
    Ok(())
}
