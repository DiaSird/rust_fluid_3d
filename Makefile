default: run

release: fmt clippy run

build:
	cargo build

dev:
	cd gui && npm run tauri dev

ui:
	cd gui && npm run tauri build

run:
	cargo run

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --fix --allow-staged --allow-dirty
