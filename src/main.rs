mod game;
use game::*;

const CELL_SIZE: f32 = 20.0;
const GRID_WIDTH: u32 = 20;
const GRID_HEIGHT: u32 = 20;

#[macroquad::main("Snake in Rust")]
async fn main() {
    run_game_loop(GRID_WIDTH, GRID_HEIGHT, CELL_SIZE).await;
}
