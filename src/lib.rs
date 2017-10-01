
extern crate crossbeam;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate layout;

// // --- macOS ---

// #[cfg(all(target_os = "macos", not(feature = "gtk")))]
// #[macro_use]
// extern crate objc;
// #[cfg(all(target_os = "macos", not(feature = "gtk")))]
// extern crate cocoa;
// #[cfg(all(target_os = "macos", not(feature = "gtk")))]
// extern crate core_graphics;

// #[cfg(all(target_os = "macos", not(feature = "gtk")))]
// #[path = "cocoa/mod.rs"]
// mod cacao;

// #[cfg(all(target_os = "macos", not(feature = "gtk")))]
// pub use cacao::program;

// // --- gtk ---

// #[cfg(any(feature = "gtk", not(target_os = "macos")))]
// extern crate gtk;

// #[cfg(any(feature = "gtk", not(target_os = "macos")))]
// #[path = "gtk/mod.rs"]
// mod gtk3;

// #[cfg(any(feature = "gtk", not(target_os = "macos")))]
// pub use gtk3::program;

// // --- common ---

#[macro_use]
mod tree;
mod stream;
mod program;

pub mod dom;
pub use program::program;
