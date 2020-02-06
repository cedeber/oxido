import init, { add } from "../pkg/simple_web.js";

// Path is mandatory for worker because rollup replace "import.meta.url"
// with document-related APIs, which is not available in worker.
let ready = init("../pkg/simple_web_bg.wasm");
let ctx = self;

ctx.addEventListener("message", event => {
    let { a, b } = event.data;

    ready.then(() => {
        ctx.postMessage(add(3, 2));
    });
});
