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
    pub width: u16,
    pub height: u16,
    pub frame: usize,
}

pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 10;

pub const FPS: usize = 2;
pub const MS_PER_FRAME: usize = 1000 / FPS;
pub const CELL: &str = "  ";

pub static mut GRID: [u8; GRID_WIDTH * GRID_HEIGHT] = [0; GRID_WIDTH * GRID_HEIGHT];
pub static mut PREV_GRID: [u8; GRID_WIDTH * GRID_HEIGHT] = [0; GRID_WIDTH * GRID_HEIGHT];

pub fn create_state() -> State {
    let mut args = args().skip(1);
    let mut width = 10;
    let mut height = 10;
    while let Some(next) = args.next() {
        match next.as_str() {
            "--help" => {
                print_usage();
                exit(0);
            }
            "-w" => {
                width = match args.next().expect("Width to be provided").parse::<u16>() {
                    Ok(w) => w,
                    Err(err) => {
                        eprintln!("ERROR - Cannot parse provided width as u16: {:?}", err);
                        exit(1);
                    }
                }
            }
            "-h" => {
                height = match args.next().expect("Height to be provided").parse::<u16>() {
                    Ok(h) => h,
                    Err(err) => {
                        eprintln!("ERROR - Cannot parse provided height as u16: {:?}", err);
                        exit(1);
                    }
                }
            }
            _ => {
                eprintln!("Unrecognized arg: {}", next);
                print_usage();
                exit(1)
            }
        }
    }

    return State {
        width,
        height,
        frame: 0,
    };
}

pub unsafe fn next_grid(state: &State) {
    //let mut log = String::new();
    for y in 0..(state.height as usize) {
        for x in 0..(state.width as usize) {
            let prev_val = GRID[x + GRID_WIDTH * y];
            let new_val = compute_neighbors(GRID[x + GRID_WIDTH * y], x, y);
            if prev_val != new_val || prev_val > 0 {
                //log.push_str(format!("[{}, {}] {} -> {}\n", x, y, prev_val, new_val).as_str());
            }
            GRID[x + GRID_WIDTH * y] = new_val;
        }
    }
    //let mut file = OpenOptions::new().write(true).append(true).open("exec.log").unwrap();

    //if let Err(e) = writeln!(file, "{}", log) {
    //eprintln!("Couldn't write to file: {}", e);
    //}
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
