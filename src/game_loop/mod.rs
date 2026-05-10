// region:    --- Modules
mod error;
mod game_core;
mod game_loop_impl;
mod platform;

// -- Flatten
use error::*;
pub use game_core::*;
pub use game_loop_impl::*;
pub use platform::*;
