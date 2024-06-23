all:
	cargo test
	cargo fmt
	cargo clippy --all-targets
	cargo verify-project
	cargo audit

	# run the binaries to make sure they work
	cargo run --bin=dlprotoc-example
	cargo run --bin=protobuf-src-example
