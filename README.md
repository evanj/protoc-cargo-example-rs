# protoc Cargo Examples

This repository contains examples of downloading protoc as part of a Rust Cargo build script (`build.rs`):

* `dlprotoc-example`: Using the [dlprotoc crate](https://github.com/evanj/dlprotoc-rs).
* `protobuf-src-example`: Using the [protobuf-src crate](https://crates.io/crates/protobuf-src)

The `dlprotoc` crate downloads pre-compiled binary releases, so is generally faster. Otherwise, they basically work the same way. They both support the ["well-known types" defined in the `google.protobuf.*` namespace](https://protobuf.dev/reference/protobuf/google.protobuf/).


## Demo

```
$ cargo run --bin=dlprotoc-example
message = Message {
    message: "protobuf compiled by dlprotoc",
    example_duration: Some(
        Duration {
            seconds: 7,
            nanos: 0,
        },
    ),
}

$ cargo run --bin=protobuf-src-example
message = Message {
    message: "protobuf compiled by protobuf-src",
    example_duration: Some(
        Duration {
            seconds: 7,
            nanos: 0,
        },
    ),
}
```
