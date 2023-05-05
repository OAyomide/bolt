.PHONY: build run setup all api watch build-yew build-tauri watch-yew watch-tauri web clean-yew clean-tauri clean cli build-cli

all: build

# Install required build tools and dependencies
setup:
	cargo install tauri-cli
	cargo install trunk
	rustup target add wasm32-unknown-unknown

# Install Bolt CLI
install-cli:
	cd bolt-cli && cargo install --path .

# Build Bolt Desktop App
build: build-yew-tauri build-tauri
	cp -r ./bolt-tauri/target/release/bundle ./target

# Build Bolt CLI
build-cli: build-yew-cli
	cd bolt-cli && cargo build --release

# Run Bolt Desktop App in debug mode
run: build-yew-tauri watch-tauri

# Run Bolt CLI in debug mode
run-cli: build-yew-cli
	cd bolt-cli && BOLT_DEV=1 cargo run

build-yew: build-yew-cli build-yew-tauri

build-yew-tauri:
	cd bolt-yew && trunk build -d ../bolt-tauri/dist --filehash false
	cd bolt-yew && cp ./script.js ../bolt-tauri/dist
	mkdir ./bolt-tauri/dist/icon/
	cp -r ./icon/* ./bolt-tauri/dist/icon/ 
	
build-yew-cli:
	cd bolt-yew && trunk build -d ../bolt-tauri/dist --filehash false
	cd bolt-yew && cp ./script.js ../bolt-tauri/dist
	mkdir ./bolt-tauri/dist/icon/
	cp -r ./icon/* ./bolt-tauri/dist/icon/

build-tauri:
	cd bolt-tauri && cargo tauri build

watch-tauri:
	cargo tauri dev

publish:
	cd bolt-server && cargo publish
	cd bolt-cli && cargo publish

# Clean temporary build files
clean: clean-yew clean-tauri clean-cli clean-lib

clean-yew:
	cd bolt-yew && cargo clean

clean-tauri:
	cd bolt-tauri && cargo clean

clean-cli:
	cd bolt-cli && cargo clean

clean-lib:
	cd bolt-server && cargo clean
