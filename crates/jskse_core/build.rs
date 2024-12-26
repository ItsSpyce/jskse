use deno_core::{
    extension,
    snapshot::{create_snapshot, CreateSnapshotOptions},
};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    extension!(
        jskse_extension,
        esm_entry_point = "jskse:runtime",
        esm = [
            dir "lib",
            "jskse:runtime" = "runtime.js"
        ],
    );

    let options = CreateSnapshotOptions {
        cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
        startup_snapshot: None,
        extensions: vec![jskse_extension::init_ops_and_esm()],
        with_runtime_cb: None,
        skip_op_registration: false,
        extension_transpiler: None,
    };
    let warmup_script = None;
    let snapshot = create_snapshot(options, warmup_script).expect("Error creating snapshot");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let file_path = out_dir.join("jskse.bin");
    fs::write(file_path, snapshot.output).expect("Failed to write snapshot");

    for path in snapshot.files_loaded_during_snapshot {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}
