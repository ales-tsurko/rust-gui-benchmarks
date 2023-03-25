const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

const container = document.getElementById('container');
var DONE = false;

invoke("start").then((addr) => {
  const socket = new WebSocket(`ws://${addr}`);

  socket.addEventListener('open', (event) => {
    console.log('Connected to WebSocket');
    // Send a message to the WebSocket server
    socket.send(0);
  });

  socket.addEventListener('message', (event) => {
    const count = parseInt(event.data);
    addDiv(count);
    socket.send(count);
  });

  socket.addEventListener('close', (event) => {
    // console.log('Disconnected from WebSocket');
  });

  socket.addEventListener('error', (event) => {
    // console.log(`WebSocket error: ${event}`);
  });

});

function addDiv(number) {
  const item = document.createElement('div');
  item.setAttribute("class", "item");
  item.innerText = number;

  container.appendChild(item);
}

await appWindow.listen('result', (event) => {
  const header = document.createElement('h1');
  header.innerText = `The result in nanoseconds is: ${event.payload.time}`;
  container.insertAdjacentElement("afterBegin", header);
});