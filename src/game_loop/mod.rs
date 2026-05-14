// region:    --- Modules
mod error;
mod game_core;
mod game_loop_impl;
mod platform;
mod snake_algorithm;

// -- Flatten
pub use game_core::*;
pub use game_loop_impl::*;
pub use platform::*;
pub use snake_algorithm::*;
