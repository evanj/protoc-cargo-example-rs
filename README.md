# protoc Cargo Examples

This repository contains examples of downloading protoc as part of a Rust Cargo build script (`build.rs`):

* `dlprotoc-example`: Using the [dlprotoc crate](https://github.com/evanj/dlprotoc-rs).
* `protobuf-src-example`: Using the [protobuf-src crate](https://crates.io/crates/protobuf-src)

The `dlprotoc` crate downloads pre-compiled binary releases, so is generally faster. Otherwise, they basically work the same way.

## Demo

```
$ cargo run --bin=dlprotoc-example
message compiled by dlprotoc = Message {
    message: "protobuf compiled by dlprotobuf",
}

$ cargo run --bin=protobuf-src-example
message = Message {
    message: "protobuf compiled by protobuf-src",
}
```
