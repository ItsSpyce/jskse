//! Sometimes we want wrappers for the C++ functions we're calling.

// a conditional use statement...
#[cfg(not(test))]
use crate::cxx::jskse_cxx::{lookup_translation, notify_player};

/// Convenience function for printing a message on the screen.
#[cfg(not(test))]
pub fn notify(msg: &str) {
    cxx::let_cxx_string!(message = msg);
    notify_player(&message);
}

// Note the stub to prevent test compilation from trying to pull in the game dll.
#[cfg(test)]
pub fn notify(_msg: &str) {}

/// Convenience function for doing the cxx macro boilerplate before
/// calling C++ with a string.
#[cfg(not(test))]
pub fn translated_key(key: &str) -> String {
    cxx::let_cxx_string!(cxxkey = key);
    lookup_translation(&cxxkey)
}

// Note the stub to prevent test compilation from trying to pull in the game dll.
#[cfg(test)]
pub fn translated_key(key: &str) -> String {
    format!("translation of {key}")
}

#[cxx::bridge]
pub mod wrappers {
    #[namespace = "wrappers"]
    extern "Rust" {
        fn notify(msg: &str);
        fn translated_key(key: &str) -> String;
    }
}
