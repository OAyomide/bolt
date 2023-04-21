#!/usr/bin/env just --justfile

_default: build

alias s := setup
alias b := build
alias r := run
alias w := watch
alias c := clean
alias a := api

# Install the dependencies
setup:
	cargo install tauri-cli
	cargo install trunk
	rustup target add wasm32-unknown-unknown


# Build the project
build: build-yew build-tauri
	cp -r ./tauri/target/release/bundle ./target

# Run the project
run: build-yew watch-tauri

cli: build-yew
	cd cli && cargo run

# Run the project in development mode (with hot reload)
watch:
	just watch-yew &
	just watch-tauri

# Build the frontend
build-yew:
	cd yew && trunk build -d ../tauri/dist --filehash false
	cd yew && cp ./script.js ../tauri/dist

# Build the backend
build-tauri:
	cd tauri && cargo tauri build

build-cli:
	cd cli && cargo build

# Run the tauri app in development mode
watch-tauri:
	cargo tauri dev

# Run the yew app in development mode
watch-yew:
	cd yew && trunk watch -d ../tauri/dist

# Run the frontend
web: build-yew
	cd ./tauri/dist && http-server -p 3000

# Clean the frontend project
clean-yew:
	cd yew && cargo clean

# Clean the backend project
clean-tauri:
	cd tauri && cargo clean

# Clean the project
clean: clean-yew clean-tauri

# Run the api
api:
	cd api && cargo run
