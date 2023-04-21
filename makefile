.PHONY: build run setup all api watch build-yew build-tauri watch-yew watch-tauri web clean-yew clean-tauri clean cli build-cli

all: build

setup:
	cargo install tauri-cli
	cargo install trunk
	rustup target add wasm32-unknown-unknown


build: build-yew build-tauri
	cp -r ./tauri/target/release/bundle ./target

run: build-yew watch-tauri

cli: build-yew
	cd cli && cargo run

watch:
	make watch-yew &
	make watch-tauri

build-yew: build-yew-tauri build-yew-cli

build-yew-tauri:
	cd yew && trunk build -d ../tauri/dist --filehash false --features for-tauri
	cd yew && cp ./script.js ../tauri/dist
	
build-yew-cli:
	cd yew && trunk build -d ../cli/dist --filehash false --features cli
	cd yew && cp ./script.js ../cli/dist


build-tauri:
	cd tauri && cargo tauri build

build-cli:
	cd cli && cargo build

watch-tauri:
	cargo tauri dev

watch-yew:
	cd yew && trunk watch -d ../tauri/dist
	
web: build-yew
	cd ./tauri/dist && http-server -p 3000

clean-yew:
	cd yew && cargo clean

clean-tauri:
	cd tauri && cargo clean

clean: clean-yew clean-tauri

api:
	cd api && cargo run