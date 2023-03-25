const invoke = window.__TAURI__.invoke;
const { appWindow } = window.__TAURI__.window;

const container = document.getElementById('container');

async function pong(count) {
  const res = await invoke('ping', { count });

  if (res === count) {
    return;
  }

  const item = document.createElement('div');
  item.setAttribute("class", "item");
  item.innerText = res;

  container.appendChild(item);

  await pong(res);
}

await appWindow.listen('result', (event) => {
  const header = document.createElement('h1');
  header.innerText = `The result in nanoseconds is: ${event.payload.time}`;
  container.insertAdjacentElement("afterBegin", header);
});

await pong(0);