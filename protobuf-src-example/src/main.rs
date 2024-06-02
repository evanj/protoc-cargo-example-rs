mod examplepb {
    include!(concat!(env!("OUT_DIR"), "/examplepb.rs"));
}

fn main() {
    let message = examplepb::Message {
        message: "protobuf compiled by protobuf-src".to_string(),
    };
    println!("message = {message:#?}");
}
