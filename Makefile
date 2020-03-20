.PHONY: all clean

all:
	cd ./disease && wasm-pack build --target web
	cd ./inverse-square-law && wasm-pack build --target web

clean:
	cd ./disease && cargo clean
	cd ./inverse-square-law && cargo clean
