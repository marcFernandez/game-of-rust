use crate::game::{GRID, GRID_HEIGHT, GRID_WIDTH};

// PACKET
// [8bit]  [16bit ]                [       ]
// [header][content_size max=65535][content]
//
// 8bit header allows 4 commands
//   - 0000: New grid
//   - 0001: Log msg
//   - 0010: Unused
//   - ...
//   - 1111: Unused
pub const CMD_NEW_GRID: u8 = 0;
pub const CMD_LOG_MSG: u8 = 1;

// sizes are represented in Bytes
pub const MAX_CONTENT_SIZE: usize = 65536;
pub const CMD_HEADER_SIZE: usize = 1;
pub const SIZE_HEADER_SIZE: usize = 2;

// CMD       | SIZE                | CONTENT
// 0000 0000 | 0000 0000 0000 0000 | 0000 ... 0000
//
// New grid of 10x10 cells
// CMD(0)    | SIZE = 100          | CONTENT = grid serialized (?)
// 0000 0000 | 0000 0000 0110 0100 | 0000 ... 0000

pub unsafe fn compress_grid() -> [u8; (GRID_WIDTH * GRID_HEIGHT) / 8] {
    //  GRID = ["0", "0", "1", "0", "0", "0", "1", "0", ...] -> 80 elems
    // bytes = [x30, x30, x31, x30, x30, x30, x31, x30, ...] -> 80 bytes
    // CGRID = [b00100010, ...] -> 10 bytes

    let mut current_byte: u8;
    let mut compressed_grid: [u8; (GRID_WIDTH * GRID_HEIGHT) / 8] = [0; (GRID_WIDTH * GRID_HEIGHT) / 8];
    let mut g = 0;
    for byte in 0..((GRID_WIDTH * GRID_HEIGHT) / 8) {
        current_byte = 0;
        for i in 0..8 {
            current_byte |= GRID[g] << i;
            g += 1;
        }
        compressed_grid[byte] = current_byte;
    }
    return compressed_grid;
}

pub unsafe fn uncompress_grid(cgrid: &[u8]) {
    for (i, byte) in cgrid.iter().enumerate() {
        for bit in 0..8 {
            GRID[(i * 8) + bit] = (byte >> bit) & 0x01;
        }
    }
    for i in 0..GRID_HEIGHT {
        eprintln!("{:?}", &GRID[(i * GRID_WIDTH)..((i * GRID_WIDTH) + GRID_WIDTH)]);
    }
}
