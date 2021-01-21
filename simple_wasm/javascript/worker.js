let ctx = self;

ctx.addEventListener("message", event => {
    let { a, b } = event.data;

    /*
    // Not compatible with Webkit
    WebAssembly.instantiateStreaming(fetch('./simple_wasm.wasm'))
        .then(obj => {
            let { add } = obj.instance.exports;
            ctx.postMessage(add(a, b));
        });

    WebAssembly.compileStreaming(fetch('./.wasm'))
        .then(module => WebAssembly.instantiate(module))
        .then(instance => {
            let { add } = instance.exports;
            ctx.postMessage(add(a, b));
        });
    */

    let importObject = {};

    fetch('../target/wasm32-wasi/release/simple_wasm.wasm').then(response =>
        response.arrayBuffer()
    ).then(bytes =>
        WebAssembly.instantiate(bytes, importObject)
    ).then(obj => {
        let { add } = obj.instance.exports;
        ctx.postMessage(add(a, b));
    });
});
