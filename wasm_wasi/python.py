from wasmer import engine, Store, Module, Instance
from wasmer_compiler_cranelift import Compiler

# Let's define the store, that holds the engine, that holds the compiler.
store = Store(engine.JIT(Compiler))

# Let's compile the module to be able to execute it!
module = Module(store, open('./target/wasm32-wasi/release/simple_wasm.wasm', 'rb').read())

# Now the module is compiled, we can instantiate it.
instance = Instance(module)

# Call the exported `add` function.
add = instance.exports.add

print("wasmer", add(5, 37)) # 42!

# --- --- ---

from wasmtime import Store, Module, Instance, Func, FuncType

store = Store()
module = Module(store.engine, open('./target/wasm32-wasi/release/simple_wasm.wasm', 'rb').read())
instance = Instance(store, module, [])
add = instance.exports["add"]

print("wasmtime", add(5, 37)) # 42!
