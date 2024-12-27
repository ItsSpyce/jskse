//! Logging functions exposed to C++ to initialize and unify logs from both sides.
//!
//! There's an initialization function that must be called in `main.cpp` to tell
//! the plugin where to log. The other functions are for C++ to use to share a log
//! file with Rust. For now, C++ must pass a preformatted-string to these functions.
//! This is wasteful, but exposing Rust macros to C++ is not possible.

#[cfg(target_os = "windows")]
use simplelog::*;
use std::fs::File;

use crate::skse_poly;

pub fn configure_logging() {
    let log_directory = match skse_poly::log_directory() {
        Ok(dir) => dir,
        Err(err) => {
            log::error!("Error getting log directory: {}", err);
            panic!("Error getting log directory: {}", err);
        }
    };
    let log_file = log_directory.join("jskse.log");
    let Ok(log_file) = File::create(log_file) else {
        panic!("Failed to create log file");
    };
    let log_level = log::LevelFilter::Debug;
    let config = simplelog::ConfigBuilder::new()
        .set_thread_level(LevelFilter::Off)
        .set_level_padding(simplelog::LevelPadding::Right)
        .set_location_level(LevelFilter::Trace)
        .set_target_level(LevelFilter::Trace)
        .build();
    let Ok(_) = WriteLogger::init(log_level, config, log_file) else {
        panic!("Failed to initialize logger");
    };
    log::info!("Initialized logging, version {}", env!("CARGO_PKG_VERSION"));
}

/// For C++, log at the error level.
pub fn log_error(message: String) {
    log::error!("{}", message);
}

/// For C++, log at the warn level.
pub fn log_warn(message: String) {
    log::warn!("{}", message);
}

/// For C++, log at the info level.
pub fn log_info(message: String) {
    log::info!("{}", message);
}

/// For C++, log at the debug level.
pub fn log_debug(message: String) {
    log::debug!("{}", message);
}

/// For C++, log at the trace level.
pub fn log_trace(message: String) {
    log::trace!("{}", message);
}

#[cxx::bridge]
pub mod logs {
    #[namespace = "logging"]
    extern "Rust" {
        fn configure_logging();
        fn log_warn(message: String);
        fn log_info(message: String);
        fn log_debug(message: String);
        fn log_trace(message: String);
        fn log_error(message: String);
    }
}
