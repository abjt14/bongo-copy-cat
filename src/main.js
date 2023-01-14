const { invoke } = window.__TAURI__.tauri;

async function listener() {
  await invoke("listener");
}

window.addEventListener("DOMContentLoaded", () => {
  listener();

  document.querySelector("button").addEventListener("click", (e) => {
    setImageIndex();
  });
});

const IMAGE_ELEMENT = document.querySelector("img");
const IMAGES = [
  "./assets/bongo-cat/none.png",
  "./assets/bongo-cat/left.png",
  "./assets/bongo-cat/right.png",
];
let active_index = 0;
let last_index = 2;
let timeout = setTimeout(() => {}, 0);

function setImage() {
  IMAGE_ELEMENT.setAttribute("src", IMAGES[active_index]);
}

function setImageIdle() {
  clearTimeout(timeout);
  timeout =  setTimeout(() => {
    active_index = 0;
    setImage();
  }, 100);
}

function setImageIndex() {
  console.log(last_index);
  if (last_index == 1) {
    active_index = 2;
    last_index = 2;
  } else if (last_index == 2) {
    active_index = 1;
    last_index = 1;
  }

  setImageIdle();
  setImage();
}
