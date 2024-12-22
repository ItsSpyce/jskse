fn main() {
    let _bridge = cxx_build::bridge("lib.rs");
    println!("cargo:rerun-if-changed=lib.rs");
}
