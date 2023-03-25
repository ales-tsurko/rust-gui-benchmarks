const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

const container = document.getElementById('container');

await appWindow.listen('ping', (event) => {
  const item = document.createElement('div');
  item.setAttribute("class", "item");
  item.innerText = event.payload.count;

  container.appendChild(item);

  appWindow.emit('pong', event.payload);
})

await appWindow.listen('result', (event) => {
  const header = document.createElement('h1');
  header.innerText = `The result in nanoseconds is: ${event.payload.time}`;
  container.insertAdjacentElement("afterBegin", header);
})

appWindow.emit('pong', {
  count: 0,
});