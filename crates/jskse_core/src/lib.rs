#![deny(future_incompatible, clippy::unwrap_used)]
#![warn(rust_2018_idioms, trivial_casts)]

pub mod bridge;
pub mod cxx;
pub mod js;
pub mod module_loader;
pub mod ops;
pub mod skse_poly;
pub mod ui;
/*
I want to preface this by saying I hate Rust. My brain wasn't built for Rust.

This library is split into multiple parts for organization.

- `bridge` contains all definitions that are exposed to C++ from Rust.
- `cxx` contains all definitions that are exposed to Rust from C++.
- `skse_poly` contains definitions for Rust that cannot be pulled directly from C++
due to CXX bridging limitations

*/
