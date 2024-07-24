const ws = new WebSocket("ws://0.0.0.0:42069");

ws.addEventListener("open", (e) => {
  console.log(`Connected`);
  let utf8Encode = new TextEncoder();
  let data = utf8Encode.encode("zz");
  console.log(data.byteLength);
  console.log(data);
  ws.send(data);
});

ws.addEventListener("message", (e) => {
  console.log(e);
  console.log(e.data);
});

ws.addEventListener("error", (e) => {
  console.log(e);
});

ws.addEventListener("close", (e) => {
  console.log(`close: code[${e.code}] reason[${e.reason}]`);
});
