use std::env;
use std::path::PathBuf;



fn main() {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR env var is not defined"));

    let out_dir = PathBuf::from(env::var("OUT_DIR")
        .expect("OUT_DIR env var is not defined"));

    let config = cbindgen::Config::from_file("cbindgen.toml")
        .expect("Unable to find cbindgen.toml configuration file");

    // OUT_DIR doesn't appear to be configurable, so prolly not a good destination
    // cargo +nightly build --out-dir `pwd` -Z unstable-options
    // added question to this issue: https://github.com/rust-lang/cargo/issues/6790
    // for now, CARGO_MANIFEST_DIR (crate_dir) seems reasonable
    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file(out_dir.join("libnyx.h"));
    
}
