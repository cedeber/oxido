dev : src/lib.rs
	wasm-pack build --dev --target web

release : src/lib.rs
	wasm-pack build --target web
	wasm-opt -Os -o pkg/simple_web.wasm pkg/simple_web_bg.wasm
	terser pkg/simple_web.js --output pkg/simple_web.min.js --compress --module --ecma 7
	rm pkg/simple_web_bg.wasm
	rm pkg/simple_web.js
	mv pkg/simple_web.wasm pkg/simple_web_bg.wasm
	mv pkg/simple_web.min.js pkg/simple_web.js

clean :
	rm -rf target/
	rm -rf pkg/