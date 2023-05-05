.PHONY: build run setup all api watch build-yew build-tauri watch-yew watch-tauri web clean-yew clean-tauri clean cli build-cli

all: build

# Install required build tools and dependencies
setup:
	cargo install tauri-cli
	cargo install trunk
	rustup target add wasm32-unknown-unknown

# Install Bolt CLI
install-cli:
	cd bolt_cli && cargo install --path .

# Build Bolt Desktop App
build: build-yew-tauri build-tauri
	cp -r ./bolt_tauri/target/release/bundle ./target

# Build Bolt CLI
build-cli: build-yew-cli
	cd bolt_cli && cargo build --release

# Run Bolt Desktop App in debug mode
run: build-yew-tauri watch-tauri

# Run Bolt CLI in debug mode
run-cli: build-yew-cli
	cd bolt_cli && BOLT_DEV=1 cargo run

build-yew: build-yew-cli build-yew-tauri

build-yew-tauri:
	cd bolt_yew && trunk build -d ../bolt_tauri/dist --filehash false
	cd bolt_yew && cp ./script.js ../bolt_tauri/dist
	mkdir ./bolt_tauri/dist/icon/
	cp -r ./icon/* ./bolt_tauri/dist/icon/ 
	
build-yew-cli:
	cd bolt_yew && trunk build -d ../bolt_tauri/dist --filehash false
	cd bolt_yew && cp ./script.js ../bolt_tauri/dist
	mkdir ./bolt_tauri/dist/icon/
	cp -r ./icon/* ./bolt_tauri/dist/icon/

build-tauri:
	cd bolt_tauri && cargo tauri build

watch-tauri:
	cargo tauri dev

publish:
	cd bolt_server && cargo publish
	cd bolt_cli && cargo publish

# Clean temporary build files
clean: clean_yew clean-tauri clean-cli clean-lib

clean-yew:
	cd bolt_yew && cargo clean

clean-tauri:
	cd bolt_tauri && cargo clean

clean-cli:
	cd bolt_cli && cargo clean

clean-lib:
	cd bolt_server && cargo clean
