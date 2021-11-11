build:
	cargo build

buildrel:
	cargo build --release

install: buildrel
	doas cp ./target/release/tostadora /usr/bin/tostadora
