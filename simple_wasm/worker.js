let ctx = self;

ctx.addEventListener("message", event => {
    let { a, b } = event.data;

    WebAssembly.instantiateStreaming(fetch('./target/wasm32-wasi/release/simple_wasm.wasm'))
        .then(obj => {
            let { add } = obj.instance.exports;
            ctx.postMessage(add(a, b));
        });
});
