// region:    --- Modules

mod apple;
mod coordinates;
mod game_loop;
mod orientation;
mod rendering;
mod snake;

// -- Flatten
use apple::*;
use coordinates::*;
pub use game_loop::*;
use orientation::*;
use snake::*;
