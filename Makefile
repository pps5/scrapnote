build-wasm:
	cd scrapnote \
		&& cargo build \
		&& wasm-pack build --target web --out-name scrapnote \
		&& rollup ./main.js --format iife --file ./pkg/scrapnote.js
	cp scrapnote/pkg/scrapnote.js static/
	cp scrapnote/pkg/scrapnote_bg.wasm static/
	cp scrapnote/static/index.html static/
	cp scrapnote/static/styles.css static/

run-release: build-wasm
	cargo run --release

run: build-wasm
	cargo run --bin webview

