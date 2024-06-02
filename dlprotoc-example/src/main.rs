mod examplepb {
    include!(concat!(env!("OUT_DIR"), "/examplepb.rs"));
}

fn main() {
    let message = examplepb::Message {
        message: "protobuf compiled by dlprotobuf".to_string(),
    };
    println!("message compiled by dlprotoc = {message:#?}");
}
