.PHONY: all clean

all:
	cd ./disease && wasm-pack build --target web

clean:
	cd ./disease && cargo clean