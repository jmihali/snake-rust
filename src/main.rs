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
use tokio::runtime::Builder;

mod snake_algorithm_bfs;
mod snake_algorithm_naive;
use clap::Parser;
use snake_algorithm_bfs::SnakeAlgorithmBFS;

use crate::game_loop::{Platform, SnakeAlgorithm};

const CELL_SIZE: f32 = 20.0;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 20, value_parser = clap::value_parser!(u32).range(10..=50))]
    grid_width: u32,
    #[arg(long, default_value_t = 20, value_parser = clap::value_parser!(u32).range(10..=30))]
    grid_height: u32,
    #[arg(long, default_value_t = 3, value_parser = clap::value_parser!(u32).range(1..=5))]
    speed: u32,
    #[arg(long, default_value_t = false)]
    ai: bool,
}

async fn run_game<P: Platform>(
    platform: P,
    args: Args,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let snake_algorithm: Option<Box<dyn SnakeAlgorithm>> = if args.ai {
        Some(Box::new(SnakeAlgorithmBFS::new()))
    } else {
        None
    };

    let move_delay = match args.speed {
        1 => 0.3,
        2 => 0.15,
        3 => 0.1,
        4 => 0.05,
        5 => 0.01,
        _ => 0.15,
    };

    let mut game_loop: GameLoop<P> = GameLoop::new(
        args.grid_width,
        args.grid_height,
        CELL_SIZE,
        move_delay,
        platform,
        snake_algorithm,
    );

    game_loop
        .run_game_loop()
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

#[cfg(feature = "macroquad_support")]
#[macroquad::main("Snake in Rust")]
async fn main() {
    let args = Args::parse();
    let platform = MacroquadPlatform::new();

    if let Err(err) = run_game(platform, args).await {
        eprintln!("Game loop failed with error {}", err);
        std::process::exit(-1);
    }
}

#[cfg(feature = "raylib_support")]
fn main() {
    let args = Args::parse();
    let platform = RaylibPlatform::new(800, 800, "Snake Game - Raylib Edition");

    let rt = Builder::new_current_thread().build().unwrap();
    let res = rt.block_on(run_game(platform, args));

    if let Err(err) = res {
        eprintln!("Game loop failed with error {}", err);
        std::process::exit(-1);
    }
}
