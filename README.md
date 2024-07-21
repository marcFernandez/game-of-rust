# Online Game of Life

Implementation of [Game of Life]() in Rust. Server sends the grid to all connected clients each frame.

![Four terminal panes synchronized](assets/demo.gif)

### TODO

- [X] Implement protocol to send/recv data
- [ ] Log to file
- [ ] Update client pool when client disconnects
- [ ] Handle client errors
- [ ] Send grid as bit per cell instead of byte per cell
