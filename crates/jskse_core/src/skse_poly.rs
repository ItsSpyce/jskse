use std::path::PathBuf;

pub fn log_directory() -> Result<PathBuf, &'static str> {
    // get My Documents path
    let my_documents = match dirs::document_dir() {
        Some(path) => path,
        None => return Err("Could not find My Documents directory"),
    };
    Ok(my_documents
        .join("My Games")
        .join("Skyrim Special Edition")
        .join("SKSE"))
}

const fn pack_version(major: u32, minor: u32, build: u32, sub: u32) -> u32 {
    (((major) & 0xFF) << 24) | (((minor) & 0xFF) << 16) | (((build) & 0xFFF) << 4) | ((sub) & 0xF)
}

pub static RUNTIME_SSE_1_1_47: u32 = pack_version(1, 1, 47, 0);
pub static RUNTIME_SSE_1_1_51: u32 = pack_version(1, 1, 51, 0);
pub static RUNTIME_SSE_1_2_36: u32 = pack_version(1, 2, 36, 0);
pub static RUNTIME_SSE_1_2_39: u32 = pack_version(1, 2, 39, 0);
pub static RUNTIME_SSE_1_3_5: u32 = pack_version(1, 3, 5, 0);
pub static RUNTIME_SSE_1_3_9: u32 = pack_version(1, 3, 9, 0);
pub static RUNTIME_SSE_1_4_2: u32 = pack_version(1, 4, 2, 0);
pub static RUNTIME_SSE_1_5_3: u32 = pack_version(1, 5, 3, 0);
pub static RUNTIME_SSE_1_5_16: u32 = pack_version(1, 5, 16, 0);
pub static RUNTIME_SSE_1_5_23: u32 = pack_version(1, 5, 23, 0);
pub static RUNTIME_SSE_1_5_50: u32 = pack_version(1, 5, 50, 0);
pub static RUNTIME_SSE_1_5_53: u32 = pack_version(1, 5, 53, 0);
pub static RUNTIME_SSE_1_5_62: u32 = pack_version(1, 5, 62, 0);
pub static RUNTIME_SSE_1_5_73: u32 = pack_version(1, 5, 73, 0);
pub static RUNTIME_SSE_1_5_80: u32 = pack_version(1, 5, 80, 0);
pub static RUNTIME_SSE_1_5_97: u32 = pack_version(1, 5, 97, 0);
pub static RUNTIME_SSE_1_6_317: u32 = pack_version(1, 6, 317, 0);
pub static RUNTIME_SSE_1_6_318: u32 = pack_version(1, 6, 318, 0);
pub static RUNTIME_SSE_1_6_323: u32 = pack_version(1, 6, 323, 0);
pub static RUNTIME_SSE_1_6_342: u32 = pack_version(1, 6, 342, 0);
pub static RUNTIME_SSE_1_6_353: u32 = pack_version(1, 6, 353, 0);
pub static RUNTIME_SSE_1_6_629: u32 = pack_version(1, 6, 629, 0);
pub static RUNTIME_SSE_1_6_640: u32 = pack_version(1, 6, 640, 0);
pub static RUNTIME_SSE_1_6_659: u32 = pack_version(1, 6, 659, 0);
pub static RUNTIME_SSE_1_6_678: u32 = pack_version(1, 6, 678, 0);
pub static RUNTIME_SSE_1_6_1330: u32 = pack_version(1, 5, 1330, 0);
pub static RUNTIME_SSE_LATEST_AE: u32 = RUNTIME_SSE_1_6_1330;
pub static RUNTIME_SSE_LATEST_SE: u32 = RUNTIME_SSE_1_5_97;
pub static RUNTIME_SSE_LATEST: u32 = RUNTIME_SSE_LATEST_AE;
