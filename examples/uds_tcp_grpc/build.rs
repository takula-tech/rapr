fn main() {
    tonic_prost_build::configure()
        .compile_protos(&["proto/helloworld.proto"], &["proto"])
        .unwrap();
}
