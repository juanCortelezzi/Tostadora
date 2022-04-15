build:
	cargo build

buildrel:
	cargo build --release

install: buildrel
	# you can change the name of the bin by changing the name on the destination
	# of the copy command, or by changing the name attribute on the cargo.toml
	# file
	cp ./target/release/tost /usr/bin/tost
