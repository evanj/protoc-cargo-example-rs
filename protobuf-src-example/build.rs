use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("PROTOC", protobuf_src::protoc());
    prost_build::compile_protos(&["src/example.proto"], &["src/"])?;
    Ok(())
}
