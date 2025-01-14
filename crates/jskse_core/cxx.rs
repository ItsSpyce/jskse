#[cxx::bridge]
pub mod jskse_cxx {
    #[namespace = "jskse"]
    extern "Rust" {}

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
}
