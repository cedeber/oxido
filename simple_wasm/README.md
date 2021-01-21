# Simple Web

The simplest example of WebAssembly.
Run on the server and in the browser.

## Build WebAssembly

```sh
rustup target add wasm32-wasi
cargo build --release --target wasm32-wasi
```

## CLI

```sh
wasmer target/wasm32-wasi/release/simple_wasm.wasm -i add 3 2
wasmtime target/wasm32-wasi/release/simple_wasm.wasm --invoke add 3 2
```

## Javascript

The browsers have all the needed tools.

## Python

```sh
pip install wasmer
pip install wasmer-compiler-cranelift
pip install wasmtime
```
