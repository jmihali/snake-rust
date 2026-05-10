mod game_loop;
mod platform_macroquad;
// mod platform_raylib;

use game_loop::GameLoop;
use platform_macroquad::MacroquadPlatform;
// use platform_raylib::RaylibPlatform;
// use tokio;

const CELL_SIZE: f32 = 20.0;
const GRID_WIDTH: u32 = 20;
const GRID_HEIGHT: u32 = 20;
const MOVE_DELAY: f32 = 0.15; // seconds between moves

#[macroquad::main("Snake in Rust")]
// #[tokio::main(flavor = "current_thread")]
async fn main() {
    let platform = MacroquadPlatform::new();
    // let platform = RaylibPlatform::new(800, 800, "Snake Game - Raylib Edition");

    let mut game_loop = GameLoop::new(GRID_WIDTH, GRID_HEIGHT, CELL_SIZE, MOVE_DELAY, platform);

    game_loop.run_game_loop().await;
}
