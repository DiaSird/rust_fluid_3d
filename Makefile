default: run

release: fmt clippy run

build:
	cargo build

ui:
	cd gui && npm run tauri dev

run:
	cargo run

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --fix --allow-staged --allow-dirty
