use std::{env::args, process::exit};

use crate::term::reset_terminal;

pub fn print_usage() {
    reset_terminal().ok();
    println!("");
    println!("    --help  print this help");
    println!("    -w  width of the board");
    println!("    -h  height of the board");
}

pub struct State {
    pub total_bytes_sent: usize,
    pub total_messages_sent: usize,
    pub encoded_grid_lengths: Vec<usize>,
    pub frames: usize,
}

//pub const GRID_WIDTH: usize = 48;
//pub const GRID_HEIGHT: usize = 10;
pub const GRID_WIDTH: usize = 48;
pub const GRID_HEIGHT: usize = 31;

pub const FPS: usize = 5;
pub const MS_PER_FRAME: usize = 1000 / FPS;
pub const CELL: &str = "  ";

pub static mut GRID: [u8; GRID_WIDTH * GRID_HEIGHT] = [0; GRID_WIDTH * GRID_HEIGHT];
pub static mut PREV_GRID: [u8; GRID_WIDTH * GRID_HEIGHT] = [0; GRID_WIDTH * GRID_HEIGHT];

pub fn create_state() -> State {
    let mut args = args().skip(1);
    while let Some(next) = args.next() {
        match next.as_str() {
            "--help" => {
                print_usage();
                exit(0);
            }
            _ => {
                eprintln!("Unrecognized arg: {}", next);
                print_usage();
                exit(1)
            }
        }
    }

    return State {
        total_bytes_sent: 0,
        encoded_grid_lengths: Vec::new(),
        total_messages_sent: 0,
        frames: 0,
    };
}

pub unsafe fn next_grid() {
    for y in 0..(GRID_HEIGHT as usize) {
        for x in 0..(GRID_WIDTH as usize) {
            let prev_val = GRID[x + GRID_WIDTH * y];
            let new_val = compute_neighbors(prev_val, x, y);
            GRID[x + GRID_WIDTH * y] = new_val;
        }
    }
    for i in 0..(GRID_WIDTH * GRID_HEIGHT) {
        PREV_GRID[i] = GRID[i];
    }
}

//              previous cell state     dead                      alive
static CELL_STATE_MAP: [[u8; 8]; 2] = [[0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 1, 1, 0, 0, 0, 0]];
static IT_VALUES: [isize; 3] = [-1, 0, 1];

unsafe fn compute_neighbors(curr_cell: u8, x: usize, y: usize) -> u8 {
    let mut sum = 0;
    for j in IT_VALUES {
        for i in IT_VALUES {
            let actual_x = match x.checked_add_signed(i) {
                Some(new_val) if new_val >= GRID_WIDTH => 0,
                Some(new_val) => new_val,
                None => GRID_WIDTH - 1,
            };
            let actual_y = match y.checked_add_signed(j) {
                Some(new_val) if new_val >= GRID_HEIGHT => 0,
                Some(new_val) => new_val,
                None => GRID_HEIGHT - 1,
            };
            if actual_x == x && actual_y == y {
                continue;
            }
            sum = sum + PREV_GRID[actual_x + GRID_WIDTH * actual_y];
        }
    }

    return CELL_STATE_MAP[curr_cell as usize][sum as usize];
}
