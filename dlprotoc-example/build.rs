fn main() -> Result<(), Box<dyn std::error::Error>> {
    dlprotoc::download_protoc()?;
    prost_build::compile_protos(&["src/example.proto"], &["src/"])?;
    Ok(())
}
