# Online Game of Life

Implementation of [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in Rust. Server sends the grid
to all connected clients each frame.

![Four terminal panes synchronized](assets/demo.gif)

## Network protocol

Server and client use TcpStreams to communicate. The message format is:

- 1 byte - Command
- 2 bytes - Content size
- 0-65535 bytes - Content

```text
  [command][content_size][content]
```

### Network commands

- 0x00: New grid
- 0x01: Log message
- 0x02: Grid dimensions
- 0x02-0xFF: Unused

Example new grid message:

```text
  command   | size           | content
  0000 0000 | 0x00 1100 0100 | 0000 ... 0000
```

### Grid encoding

Initially, I was sending the grid as a String representation of the cells' state. A single cell was taking one entire
byte over the network ('0' was encoded as 0x30 [Ref](https://en.wikipedia.org/wiki/ASCII#Printable_characters)):

```rust
  //  GRID = ["0", "0", "1", "0", "0", "0", "1", "0", ...] -> 80 elems
  // bytes = [x30, x30, x31, x30, x30, x30, x31, x30, ...] -> 80 bytes
```

This was a waste bc there are only 2 states. Instead I represented each cell as a single bit, which reduced the amount
of data x8 times:

```rust
  //  GRID = ["0", "0", "1", "0", "0", "0", "1", "0", ...] -> 80 elems
  // CGRID = [b00100010, ...] -> 10 bytes
```

## WebSocket implementation

So I went down the rabbit hole of implementing the websocket protocol (partially to support web client). I'm following
the [wikipedia site](https://en.wikipedia.org/wiki/WebSocket).

**Update**: Server is able to speak websocket language now. Frontend is able to parse dimensions and new_grid commands
and draw the board on HTMLCanvas.

![Terminal and web frontend synchronized](assets/demo_frontend.gif)

### Client setup

1. Compile `main.ts`:

```bash
npx tsc main.ts
```

2. Start the server:

```bash
cargo run --bin server 2> server.log
```

3. Open frontend in browser: `file://<path_to_repo>/public/index.html`

### TODO

> Those are in order of priority, but it can always change ¯\_(ツ)_/¯

- [X] ~Implement protocol to send/recv data~
- [X] ~Send grid as bit per cell instead of byte per cell~
- [X] ~Update client pool when client disconnects~
- [X] ~Map clients to protocol for message sending~
- [ ] **wip**: Frontend client implementation
- [ ] **wip**: Raw websocket protocol implementation
  - [X] ~Basic implementation for small messages~
  - [ ] Write and read messages larger than 127 bytes
- [ ] Handle client errors
- [ ] Server to log a QR code for web clients to use (inspired by **tj_deev** [Writing a QR Code Generator in Go](https://www.youtube.com/watch?v=71SO8NB2ghU))
- [ ] Improve file logging and create `--debug` flag
- [ ] Allow multiple message sending from server per frame (?)
