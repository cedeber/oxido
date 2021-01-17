# Simple Web

The simplest example of WebAssembly.
Run on the server and in the browser.

## Build WebAssembly


```sh
rustup target add wasm32-wasi
cargo build --release --target wasm32-wasi
wasmer run target/wasm32-wasi/release/simple_wasm.wasm -i add 3 2
```
