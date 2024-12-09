all:
	cargo test --all-targets
	cargo fmt
	# disallow warnings so they fail CI
	cargo clippy --all-targets -- -D warnings
	# fail for rustdoc warnings
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
	cargo verify-project
	cargo audit

	# run the binaries to make sure they work
	cargo run --bin=dlprotoc-example
	cargo run --bin=protobuf-src-example
