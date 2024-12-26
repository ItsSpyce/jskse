#![deny(future_incompatible, clippy::unwrap_used)]
#![warn(rust_2018_idioms, trivial_casts)]

#[cxx::bridge]
pub mod skse_bindings {

    #[namespace = "REL"]
    unsafe extern "C++" {
        include!("PCH.h");

        type Version;

        pub fn compare(self: &Version, other: &Version) -> i32;
        pub fn pack(self: &Version) -> u32;
        pub fn major(self: &Version) -> u16;
        pub fn minor(self: &Version) -> u16;
        pub fn patch(self: &Version) -> u16;
        pub fn build(self: &Version) -> u16;
        pub fn string(self: &Version, separator: &CxxString) -> String;
    }

    #[namespace = "RE"]
    unsafe extern "C++" {
        include!("PCH.h");
        type GFxMovieView;
        type GFxValue;
        type InventoryEntryData;

        type TESForm;
        pub fn GetFormID(self: &TESForm) -> u32;

        type BGSEquipSlot;
        type ButtonEvent;
        pub fn IsDown(self: &ButtonEvent) -> bool;
        pub fn IsUp(self: &ButtonEvent) -> bool;
        pub fn IsPressed(self: &ButtonEvent) -> bool;
    }

    #[namespace = "RE::BSScript"]
    unsafe extern "C++" {
        include!("PCH.h");
        type IVirtualMachine;
    }

    #[namespace = "RE::BSScript::Internal"]
    unsafe extern "C++" {
        include!("PCH.h");
        type VirtualMachine;
    }

    #[namespace = "SKSE"]
    unsafe extern "C++" {
        include!("PCH.h");
        type PluginInfo;
        type QueryInterface;

        pub fn EditorVersion(self: &QueryInterface) -> u32;
        pub fn IsEditor(self: &QueryInterface) -> bool;
        pub fn RuntimeVersion(self: &QueryInterface) -> &Version;
        pub fn SKSEVersion(self: &QueryInterface) -> u32;

        type LoadInterface;

        pub fn EditorVersion(self: &LoadInterface) -> u32;
        pub fn IsEditor(self: &LoadInterface) -> bool;
        pub fn RuntimeVersion(self: &LoadInterface) -> &Version;
        pub fn SKSEVersion(self: &LoadInterface) -> u32;
        pub fn GetPluginHandle(self: &LoadInterface) -> u32;
        pub fn GetPluginInfo(self: &LoadInterface, name: &CxxString) -> *const PluginInfo;
        pub fn GetReleaseIndex() -> u32;
        pub fn QueryInterface(self: &LoadInterface, id: u32) -> *mut QueryInterface;

        type ScaleformInterface;

        type SerializationInterface;

        pub fn Version(self: &SerializationInterface) -> u32;
        pub fn SetUniqueID(self: &SerializationInterface, uniqueID: u32);
    }
}
