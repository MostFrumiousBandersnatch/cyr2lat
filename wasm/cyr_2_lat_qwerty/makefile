node:
	wasm-pack build --target nodejs --out-dir pkg/node --release

web:
	wasm-pack build --target bundler --out-dir pkg/web --release

clean: FRC
	echo 'Cleaning pkg dir'
	rm -rf ./pkg/*

FRC:

all: clean node web
