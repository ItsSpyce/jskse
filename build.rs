fn main() {
    let _bridge = cxx_build::bridge("crates/jskse_core/lib.rs");
    println!("cargo:rerun-if-changed=crates/jskse_core//lib.rs");
}
