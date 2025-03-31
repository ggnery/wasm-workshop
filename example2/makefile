hello.wasm: hello.wat
	wasm-tools parse hello.wat -o hello.wasm

js: hello.wasm main.js
	node main.js

python: hello.wat main.py
	python main.py

rust: hello.wat
	cargo run --bin main