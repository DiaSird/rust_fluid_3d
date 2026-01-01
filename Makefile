default: ui

# release to upload
up: fmt clippy ui

dev:
	cd gui && npm run tauri dev

ui:
	cd gui && npm run tauri build --no-bundle

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --fix --allow-staged --allow-dirty
