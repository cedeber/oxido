import init, { add, async_add } from "./pkg/simple_wasm.js";

/* --- Extern --- */
// These functions will be called from Rust/Wasm
window.wasm_cb = (str) => {
  console.log("(main thread)", str);
};

// Could also be async/await
window.async_wasm_cb = (str) =>
  new Promise((resolve) => {
    window.setTimeout(() => {
      console.log("(main thread + async)", str);
      resolve(4); // will add 4 to the next async_add() call
    }, 2000);
  });

/* --- Wasm in the main thread --- */
async function main() {
  // init is main();
  await init(); // Accept a path as param if the .wasm file is not package_bg.wasm

  // -- Synchronous --
  const syncEl = document.getElementById("sync");
  syncEl.innerHTML = String(add(3, 2));

  // -- Asynchronous: Promise <-> Futures --
  const asyncEl = document.getElementById("async");
  asyncEl.innerHTML = String(await async_add(3, 2));
}

/* --- Wasm in the worker thread --- */
function worker() {
  const workerEl = document.getElementById("worker");
  const worker = new Worker("./worker.js", { type: "module" });

  worker.addEventListener("message", (event) => {
    workerEl.innerHTML = event.data;
  });

  worker.postMessage({ a: 3, b: 2 });
}

// -> Launch
main();
worker();
