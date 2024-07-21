// PACKET
// [8bit]  [16bit ]                [       ]
// [header][content_size max=65535][content]
//
// 8bit header allows 4 commands
//   - 0000: Unused
//   - 0001: New grid
//   - 0010: Log msg
//   - 0011: Unused
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
