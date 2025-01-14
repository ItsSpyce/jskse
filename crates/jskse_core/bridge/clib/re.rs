use std::marker::PhantomData;

pub struct GFxMovieView {}

pub struct GFxValue {}

pub struct InventoryEntryData {}

pub struct TESForm {}

pub struct BGSEquipSlot {}

pub struct ButtonEvent {}

pub mod bs_script {
    pub struct IVirtualMachine {}
}

#[cxx::bridge]
pub mod re {
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
}
