extern crate cbindgen;
extern crate cxx_build;

use std::io;
use std::{env, path::Path};

static BRIDGE_FILES: &[&str] = &[
    "cxx.rs",
    "bridge/cosave.rs",
    "bridge/rimgui.rs",
    "bridge/logs.rs",
    "bridge/strings.rs",
    "bridge/wrappers.rs",
    "bridge/clib/re.rs",
    "bridge/clib/rel.rs",
    "bridge/clib/skse.rs",
];

struct BindgenDescription<'a> {
    pub filename: &'a str,
    pub includes: Vec<&'a str>,
}

static BINDGEN_FILES: &[BindgenDescription] = &[BindgenDescription {
    filename: "bindgen/clib.rs",
    includes: vec![""],
}];

fn main() {
    // build cbindgen bindings
    let root_dir = find_nearest_cmake().unwrap_or_else(|_| {
        panic!("cargo:warning=Could not find CMakeLists.txt in any parent directory");
    });
    let root_dir = Path::new(&root_dir);
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let crate_dir = Path::new(&crate_dir);
    let header_file = format!(
        "{}/include/{}.h",
        root_dir.to_str().unwrap(),
        env::var("CARGO_PKG_NAME").unwrap()
    );

    for bindgen in BINDGEN_FILES {
        cbindgen::Builder::new()
            .with_crate(crate_dir.to_str().unwrap())
            .with_pragma_once(true)
            .with_cpp_compat(true)
            .with_language(cbindgen::Language::Cxx)
            .with_namespace("bindgen")
            .include_item("cxx.rs")
            .generate()
            .expect(format!("Unable to generate bindings for {}", bindgen.filename).as_str())
            .write_to_file(root_dir.join(format!("include/{}.h", bindgen.filename)));
    }

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
