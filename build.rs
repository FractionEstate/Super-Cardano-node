fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_protos(&["proto/chaindb.proto"], &["proto"])
        .expect("Failed to compile proto");
}
