mod examplepb {
    include!(concat!(env!("OUT_DIR"), "/examplepb.rs"));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let std_duration = std::time::Duration::from_secs(7);
    let proto_duration = prost_types::Duration::try_from(std_duration)?;
    let message = examplepb::Message {
        message: "protobuf compiled by protobuf-src".to_string(),
        example_duration: Some(proto_duration),
    };
    println!("message = {message:#?}");

    Ok(())
}
