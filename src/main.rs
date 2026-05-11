mod game_loop;
use game_loop::GameLoop;

#[cfg(feature = "macroquad_support")]
mod platform_macroquad;
#[cfg(feature = "macroquad_support")]
use platform_macroquad::MacroquadPlatform;

#[cfg(feature = "raylib_support")]
mod platform_raylib;
#[cfg(feature = "raylib_support")]
use platform_raylib::RaylibPlatform;
#[cfg(feature = "raylib_support")]
use tokio::runtime::Runtime;

mod naive_random_ai_model;
use naive_random_ai_model::NaiveRandomAIModel;

const CELL_SIZE: f32 = 20.0;
const GRID_WIDTH: u32 = 20;
const GRID_HEIGHT: u32 = 20;
const MOVE_DELAY: f32 = 0.15; // seconds between moves

#[macroquad::main("Snake in Rust")]
// #[tokio::main(flavor = "current_thread")]
async fn main() {
    #[cfg(feature = "macroquad_support")]
    let platform = MacroquadPlatform::new();

    #[cfg(feature = "raylib_support")]
    let platform = RaylibPlatform::new(800, 800, "Snake Game - Raylib Edition");

    let ai_model = NaiveRandomAIModel::new();
    let mut game_loop = GameLoop::new(
        GRID_WIDTH,
        GRID_HEIGHT,
        CELL_SIZE,
        MOVE_DELAY,
        platform,
        Some(ai_model),
    );

    let _ = game_loop.run_game_loop().await;
}
