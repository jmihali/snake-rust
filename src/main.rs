mod game;
use game::*;

const CELL_SIZE: f32 = 20.0;
const GRID_WIDTH: u32 = 20;
const GRID_HEIGHT: u32 = 20;
const MOVE_DELAY: f32 = 0.15; // seconds between moves

#[macroquad::main("Snake in Rust")]
async fn main() {
    let mut game_loop = GameLoop::new(GRID_WIDTH, GRID_HEIGHT, CELL_SIZE, MOVE_DELAY);

    game_loop.run_game_loop().await;
}
