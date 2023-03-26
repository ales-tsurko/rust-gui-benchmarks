const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

const container = document.getElementById('container');

function addDiv(number) {
  const item = document.createElement('div');
  item.setAttribute("class", "item");
  item.innerText = number;

  container.appendChild(item);
}

function addResult(result) {
  const header = document.createElement('h1');
  header.innerText = `The result in nanoseconds is: ${result.time}`;
  container.insertAdjacentElement("afterBegin", header);
}

invoke('start');

for (let i = 0; i < 10000; i++) {
  addDiv(i);
}

invoke('stop').then(addResult);