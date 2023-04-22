.PHONY: build run setup all api watch build-yew build-tauri watch-yew watch-tauri web clean-yew clean-tauri clean cli build-cli

all: build

setup:
	cargo install tauri-cli
	cargo install trunk
	rustup target add wasm32-unknown-unknown

install-cli:
	cd cli && cargo install --path .

build: build-yew-tauri build-tauri
	cp -r ./tauri/target/release/bundle ./target

build-cli: build-yew-cli
	cd cli && cargo build --release

run: build-yew-tauri watch-tauri

run-cli: build-yew-cli
	cd cli && cargo run

build-yew: build-yew-cli build-yew-tauri

build-yew-tauri:
	cd yew && trunk build -d ../tauri/dist --filehash false --features for-tauri
	cd yew && cp ./script.js ../tauri/dist
	
build-yew-cli:
	cd yew && trunk build -d ../cli/dist --filehash false --no-default-features --features for-cli
	cd yew && cp ./script.js ../cli/dist

build-tauri:
	cd tauri && cargo tauri build

watch-tauri:
	cargo tauri dev

clean-yew:
	cd yew && cargo clean

clean-tauri:
	cd tauri && cargo clean

clean: clean-yew clean-tauri

api:
	cd api && cargo run