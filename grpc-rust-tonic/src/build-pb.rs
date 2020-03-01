extern crate tonic_build;

fn main() {
    tonic_build::compile_protos("../proto/helloworld.proto").unwrap();
    // or:
    // tonic_build::configure()
    //     .build_server(false)
    //     .compile(&["./proto/helloworld.proto"], &["./proto"])
    //     .unwrap();
    //
    // https://github.com/hyperium/tonic/blob/master/tonic-build/README.md
}
