build:
	cargo build

buildrel:
	cargo build --release

install: buildrel
	sudo cp ./target/release/tostadora /usr/bin/tostadora
