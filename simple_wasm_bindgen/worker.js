import init, { add } from "../pkg/simple_web.js";

let ready = init();
let ctx = self;

ctx.addEventListener("message", event => {
    let { a, b } = event.data;

    ready.then(() => {
        ctx.postMessage(add(3, 2));
    });
});
