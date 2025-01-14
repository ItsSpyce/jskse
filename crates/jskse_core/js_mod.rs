use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsModDependency {
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct JsMod {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub main: String,
    pub dependencies: Vec<JsModDependency>,
}

pub fn find_mods() -> Vec<JsMod> {
    let mut result = vec![];
    let mods = std::fs::read_dir("Data/SKSE/Plugins/JSKSE").unwrap();
    for mod_dir in mods {
        if mod_dir.is_err() {
            continue;
        }
        // check if there's an skse.json file
        let mod_dir_path = mod_dir.unwrap().path();
        if !std::fs::exists(mod_dir_path.join("skse.json")).unwrap() {
            log::info!(
                "No skse.json file found in mod directory {}, skipping.",
                mod_dir_path.display()
            );
            continue;
        }
        let skse_json = std::fs::read_to_string(mod_dir_path.join("skse.json")).unwrap();
        let js_mod: JsMod = serde_json::from_str(&skse_json).unwrap();
        result.push(js_mod);
    }
    result
}
