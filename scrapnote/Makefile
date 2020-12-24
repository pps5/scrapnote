clean:
	rm -r pkg
	find static/* | grep -v index.html | xargs rm

build:
	cargo build \
		&& wasm-pack build --target web --out-name scrapnote \
		&& rollup ./main.js --format iife --file ./pkg/scrapnote.js
	cp pkg/scrapnote.js static/
	cp pkg/scrapnote_bg.wasm static/


