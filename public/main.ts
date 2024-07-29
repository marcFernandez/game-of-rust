let ws = new WebSocket("ws://0.0.0.0:42069");

const LOGIN_KEY = "zz";

ws.addEventListener("open", (_e) => {
  console.log(`Connected`);
  let utf8Encode = new TextEncoder();
  let data = utf8Encode.encode(LOGIN_KEY);
  // console.log(data.byteLength);
  // console.log(data);
  ws.send(data);
});

ws.addEventListener("message", (e) => {
  if (e.data instanceof Blob) {
    e.data.arrayBuffer().then(parseData);
  } else {
    console.log(e.data);
  }
});

ws.addEventListener("error", (e) => {
  console.log(e);
});

ws.addEventListener("close", (e) => {
  console.log(`close: code[${e.code}] reason[${e.reason}]`);
});

const CELL_WIDTH = 100;
const CELL_HEIGHT = CELL_WIDTH;

const canvas = document.getElementById("canvas")! as HTMLCanvasElement;
const ctx = canvas.getContext("2d")!;

let GRID: Array<number> = new Array();

const CMD_IDX = 0;
const CMD_SIZE = 1;
const SIZE_IDX = 1;
const SIZE_SIZE = 2;
const MSG_IDX = 3;

function parseData(data: ArrayBuffer) {
  console.log("------------------");
  console.log(`Parsing ArrayBuffer: `, data);
  const view = new DataView(data);

  const cmd = view.getUint8(CMD_IDX);
  const size = view.getUint16(SIZE_IDX);
  const msg_view = new DataView(data, 3, size);

  console.log(`Received cmd[${cmd}]`);

  msgProcessor[cmd](msg_view);
  console.log("------------------");
}

const msgProcessor = {
  0: newGrid,
  1: logMsg,
  2: dimensions,
};

function logMsg(data: DataView) {}

function dimensions(data: DataView) {
  const width = data.getUint16(0);
  const height = data.getUint16(2);
  console.log(`Grid dimensions: [${width}, ${height}]`);
  GRID = new Array(width * height);
  canvas.width = width * CELL_WIDTH;
  canvas.height = height * CELL_HEIGHT;
  ctx.fillStyle = "black";
  ctx.fillRect(0, 0, canvas.width, canvas.height);
}

function newGrid(data: DataView) {
  console.log(`New grid received`);
  for (let i = 0; i < data.byteLength; i++) {
    let byte = data.getUint8(i);
    for (let bit = 0; bit < 8; bit++) {
      GRID[i * 8 + bit] = (byte >> bit) & 0x01;
    }
  }
  console.log(GRID);
  logGrid();
  drawGrid();
}

function logGrid() {
  let gridStr = "\n";
  for (let h = 0; h < canvas.height / CELL_HEIGHT; h++) {
    for (let w = 0; w < canvas.width / CELL_WIDTH; w++) {
      gridStr += `${GRID[w + (canvas.width / CELL_WIDTH) * h]} `;
    }
    gridStr += "\n";
  }
  console.log(gridStr);
}

function drawGrid() {
  let cell = 0;
  for (let h = 0; h < canvas.height / CELL_HEIGHT; h++) {
    for (let w = 0; w < canvas.width / CELL_WIDTH; w++) {
      cell = GRID[w + h * (canvas.width / CELL_WIDTH)];
      if (cell == 1) {
        ctx.fillStyle = "white";
        ctx.fillRect(w * CELL_WIDTH, h * CELL_HEIGHT, CELL_WIDTH, CELL_HEIGHT);
      } else {
        ctx.fillStyle = "black";
        ctx.fillRect(w * CELL_WIDTH, h * CELL_HEIGHT, CELL_WIDTH, CELL_HEIGHT);
      }
    }
  }
}
