all:
	cargo test
	cargo fmt
	cargo clippy \
		--all-targets \
		-- \
		-D warnings \
		-D clippy::nursery \
		-D clippy::pedantic \
		-D clippy::style \
		-D clippy::cargo \
		-A clippy::option-if-let-else \
		-A clippy::missing-panics-doc \
		-A clippy::missing-errors-doc
	cargo verify-project
	cargo audit

	# run the binaries to make sure they work
	cargo run --bin=dlprotoc-example
	cargo run --bin=protobuf-src-example
