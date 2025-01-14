use std::ffi::c_void;

use cxx::{type_id, ExternType};

pub struct ScaleformInterface {}

unsafe impl ExternType for ScaleformInterface {
    type Id = type_id!("ScaleformInterface");
    type Kind = cxx::kind::Trivial;
}

pub struct PapyrusInterface {}

unsafe impl ExternType for PapyrusInterface {
    type Id = type_id!("PapyrusInterface");
    type Kind = cxx::kind::Trivial;
}

pub struct SKSEInterface {
    pub skseVersion: u32,
    pub runtimeVersion: u32,
    pub editorVersion: u32,
    pub isEditor: u32,
    pub QueryInterface: fn(a: u32) -> *mut c_void,
    pub GetPluginHandle: fn() -> u32,
    pub GetReleaseIndex: fn() -> u32,
    pub GetPluginInfo: fn(name: &str) -> *const c_void,
}

pub struct SerializationInterface {
    pub interfaceVersion: u32,
}

unsafe impl ExternType for SerializationInterface {
    type Id = type_id!("SerializationInterface");
    type Kind = cxx::kind::Trivial;
}

pub struct TaskInterface {
    pub interfaceVersion: u32,
}

unsafe impl ExternType for TaskInterface {
    type Id = type_id!("TaskInterface");
    type Kind = cxx::kind::Trivial;
}

pub struct TrampolineInterface {
    pub interfaceVersion: u32,
}

unsafe impl ExternType for TrampolineInterface {
    type Id = type_id!("TrampolineInterface");
    type Kind = cxx::kind::Trivial;
}

pub struct MessagingInterface {
    pub interfaceVersion: u32,
}

unsafe impl ExternType for MessagingInterface {
    type Id = type_id!("MessagingInterface");
    type Kind = cxx::kind::Trivial;
}

pub struct ObjectInterface {
    pub interfaceVersion: u32,
}

unsafe impl ExternType for ObjectInterface {
    type Id = type_id!("ObjectInterface");
    type Kind = cxx::kind::Trivial;
}

pub struct SKSEDelayFunctorManager {}

unsafe impl ExternType for SKSEDelayFunctorManager {
    type Id = type_id!("SKSEDelayFunctorManager");
    type Kind = cxx::kind::Trivial;
}

pub struct SKSEObjectRegistry {}

unsafe impl ExternType for SKSEObjectRegistry {
    type Id = type_id!("SKSEObjectRegistry");
    type Kind = cxx::kind::Trivial;
}

pub struct SKSEPersistentObjectStorage {}

unsafe impl ExternType for SKSEPersistentObjectStorage {
    type Id = type_id!("SKSEPersistentObjectStorage");
    type Kind = cxx::kind::Trivial;
}

pub struct PluginInfo {
    pub infoVersion: u32,
    pub name: *const u8,
    pub version: u32,
}

unsafe impl ExternType for PluginInfo {
    type Id = type_id!("PluginInfo");
    type Kind = cxx::kind::Trivial;
}

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

pub fn get_scaleform_interface() -> *const ScaleformInterface {
    bindings::GetScaleformInterface() as *const ScaleformInterface
}

pub fn get_papyrus_interface() -> *const PapyrusInterface {
    bindings::GetPapyrusInterface() as *const PapyrusInterface
}

pub fn get_serialization_interface() -> *const SerializationInterface {
    bindings::GetSerializationInterface() as *const SerializationInterface
}

pub fn get_task_interface() -> *const TaskInterface {
    bindings::GetTaskInterface() as *const TaskInterface
}

pub fn get_trampoline_interface() -> *const TrampolineInterface {
    bindings::GetTrampolineInterface() as *const TrampolineInterface
}

pub fn messaging_interface() -> *const MessagingInterface {
    bindings::GetMessagingInterface() as *const MessagingInterface
}

pub fn get_object_interface() -> *const ObjectInterface {
    bindings::GetObjectInterface() as *const ObjectInterface
}

pub fn get_delay_functor_manager() -> *const SKSEDelayFunctorManager {
    bindings::GetDelayFunctorManager() as *const SKSEDelayFunctorManager
}

pub fn get_object_registry() -> *const SKSEObjectRegistry {
    bindings::GetObjectRegistry() as *const SKSEObjectRegistry
}

pub fn get_persistent_object_storage() -> *const SKSEPersistentObjectStorage {
    bindings::GetPersistentObjectStorage() as *const SKSEPersistentObjectStorage
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

#[cxx::bridge]
pub mod bindings {
    #[namespace = "SKSE"]
    unsafe extern "C++" {
        include!("SKSE/API.h");

        type ScaleformInterface;
        pub fn GetScaleformInterface() -> *const ScaleformInterface;
        type PapyrusInterface;
        pub fn GetPapyrusInterface() -> *const PapyrusInterface;
        type SerializationInterface;
        pub fn GetSerializationInterface() -> *const SerializationInterface;
        type TaskInterface;
        pub fn GetTaskInterface() -> *const TaskInterface;
        type TrampolineInterface;
        pub fn GetTrampolineInterface() -> *const TrampolineInterface;
        type MessagingInterface;
        pub fn GetMessagingInterface() -> *const MessagingInterface;
        type ObjectInterface;
        pub fn GetObjectInterface() -> *const ObjectInterface;
        type SKSEDelayFunctorManager;
        pub fn GetDelayFunctorManager() -> *const SKSEDelayFunctorManager;
        type SKSEObjectRegistry;
        pub fn GetObjectRegistry() -> *const SKSEObjectRegistry;
        type SKSEPersistentObjectStorage;
        pub fn GetPersistentObjectStorage() -> *const SKSEPersistentObjectStorage;

        pub fn AllocTrampoline(size: u64, try_skse_reserve: bool);

        type PluginInfo;
    }
}
