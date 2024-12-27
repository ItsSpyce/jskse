extern crate cbindgen;
extern crate cxx_build;

use deno_core::{
    extension,
    snapshot::{create_snapshot, CreateSnapshotOptions},
};
use std::{env, path::PathBuf};
use std::{fs, io};

static BRIDGE_FILES: &[&str] = &[
    "lib.rs",
    "bridge/cosave.rs",
    "bridge/logs.rs",
    "bridge/strings.rs",
    "bridge/wrappers.rs",
];

fn main() {
    // build JS snapshot
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

    // build cbindgen bindings
    let root_dir = find_nearest_cmake();
    if root_dir.is_err() {
        println!("cargo:warning=Could not find CMakeLists.txt in any parent directory");
        return;
    }
    let root_dir = root_dir.unwrap();
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let header_file = format!(
        "{root_dir}/include/{}.h",
        env::var("CARGO_PKG_NAME").unwrap()
    );
    println!("Generating bindings for {}", header_file);
    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_pragma_once(true)
        .with_cpp_compat(true)
        .with_language(cbindgen::Language::Cxx)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(header_file.clone());

    // build cxx bindings
    let _bridge = cxx_build::bridges(BRIDGE_FILES);
    let rerun_if_changed = BRIDGE_FILES.join(",");
    println!("cargo:rerun-if-changed={}", rerun_if_changed);
}

fn find_nearest_cmake() -> Result<String, io::Error> {
    let mut dir = env::current_dir().unwrap();
    loop {
        let cmake = dir.join("CMakeLists.txt");
        if cmake.exists() {
            return Ok(dir.to_str().unwrap().to_string());
        }
        if dir == dir.parent().unwrap() {
            return Err(io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find CMakeLists.txt in any parent directory",
            ));
        }
        dir = dir.parent().unwrap().to_path_buf();
    }
}
