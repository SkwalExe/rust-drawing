all: build

build:
	cargo build --release

install:
	sudo cp target/release/rust-drawing /usr/bin/

uninstall:
	sudo rm -f /usr/bin/rust-drawing