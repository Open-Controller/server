extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/")
        .inputs(&["proto/OpenControllerLib.proto"])
        .include("proto")
        .run()
        .expect("protoc");
}