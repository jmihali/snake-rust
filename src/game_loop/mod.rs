// region:    --- Modules
mod colors;
mod error;
mod game_core;
mod game_loop_decl;
mod game_loop_impl;
mod game_loop_impl_priv;
mod platform;
mod snake_algorithm;

// -- Flatten
pub(crate) use game_core::*;
pub use game_loop_decl::*;
pub use platform::*;
pub use snake_algorithm::*;
