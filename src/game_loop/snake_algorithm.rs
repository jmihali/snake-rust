use crate::game_loop::{Apple, Orientation, Snake};

pub trait SnakeAlgorithm {
    fn decide_next_move(
        &self,
        snake: &Snake,
        apple: &Apple,
        grid_width: u32,
        grid_height: u32,
    ) -> Option<Orientation>;
}
