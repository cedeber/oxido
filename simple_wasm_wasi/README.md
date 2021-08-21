# Simple Web

The simplest example of WebAssembly.
Run on the server and in the browser.

## Build WebAssembly

```sh
rustup target add wasm32-wasi
cargo build --release --target wasm32-wasi
```

or

```sh
cargo install cargo-wasi
cargo wasi build --release
```

## CLI

```sh
wasmer target/wasm32-wasi/release/simple_wasm.wasm -i add 3 2
wasmtime target/wasm32-wasi/release/simple_wasm.wasm --invoke add 3 2
```

## Javascript

The browsers have all the needed tools. But supports only intergers and floats.
For more glue, you need to use wasm_bindgen.

## Python

```sh
pip install wasmer
pip install wasmer-compiler-cranelift
pip install wasmtime
```
