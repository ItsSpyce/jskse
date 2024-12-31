use crate::js::initialize_engine;

#[cxx::bridge]
pub mod jskse_cxx {
    #[namespace = "jskse"]
    extern "Rust" {
        fn initialize_engine();
    }

    unsafe extern "C++" {
        // Each of these blocks declares things C++ exposes to Rust.
        // All of the C++ is quarantined inside unsafe blocks, aka an instruction
        // to rustc to just trust us, bro.
    }

    #[namespace = "util"]
    unsafe extern "C++" {
        // exports from specific C++ namespaces to Rust
        include!("util.h");

        fn notify_player(message: &CxxString);
        fn lookup_translation(key: &CxxString) -> String;
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
    }
}
