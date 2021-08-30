import init, { add } from "./pkg/simple_wasm.js";

let ready = init();
let ctx = self;

ctx.wasm_cb = (str) => {
  console.log("(worker thread)", str);
};

ctx.addEventListener("message", (event) => {
  let { a, b } = event.data;

  ready.then(() => {
    ctx.postMessage(add(a, b));
  });
});
