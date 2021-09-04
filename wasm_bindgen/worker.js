import init, { add, async_add } from "./pkg/simple_wasm.js";

/* --- Extern --- */
// These functions will be called from Rust/Wasm
self.wasm_cb = (str) => {
  console.log("(worker thread)", str);
};

// Could also be async/await
self.async_wasm_cb = (str) =>
  new Promise((resolve) => {
    self.setTimeout(() => {
      console.log("(worker thread + async)", str);
      resolve(4); // will add 4 to the next async_add() call
    }, 2000);
  });

/* --- Worker --- */
self.addEventListener("message", async (event) => {
  const { a, b } = event.data;

  await init();

  // Sync
  self.postMessage(add(a, b));

  // Async
  const result = await async_add(a, b);
  self.postMessage(result);
});
