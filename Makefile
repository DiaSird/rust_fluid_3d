default: run

build:
	cargo build

run:
	cargo run

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings