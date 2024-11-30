default: run

release: fmt clippy run

build:
	cargo build

run:
	cargo run

fmt:
	cargo fmt --all

clippy:
	cargo clippy --fix --allow-staged --allow-dirty