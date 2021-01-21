# Simple Web

The simplest example of WebAssembly.
Run on the server and in the browser.

## Build WebAssembly

```sh
rustup target add wasm32-wasi
cargo build --release --target wasm32-wasi
```

## Run on the Terminal (CLI)

```sh
wasmer run target/wasm32-wasi/release/simple_wasm.wasm -i add 3 2
```

## Javascript

You must run a server form the root with, for instance:
```sh
python3 -m http.server
```

And then access the page on `http://localhost:8000/javascript/`
