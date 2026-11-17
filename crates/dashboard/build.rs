fn main() {
    println!("cargo:rerun-if-changed=schemas");
    cynic_codegen::register_schema("server")
        .from_sdl_file("schemas/server.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
