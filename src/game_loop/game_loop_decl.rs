use crate::game_loop::game_core::{Apple, Snake};
use crate::game_loop::platform::Platform;
use crate::game_loop::snake_algorithm::SnakeAlgorithm;

#[derive(Default, PartialEq, Eq)]
pub(crate) enum GameState {
    #[default]
    Running,
    GameOver,
    GameWon,
}

pub(crate) enum GameEvent {
    Collision,
    FoodReached,
}

pub struct GameLoop<P: Platform> {
    pub(crate) snake: Snake,
    pub(crate) apple: Apple,
    pub(crate) state: GameState,
    pub(crate) grid_width: u32,
    pub(crate) grid_height: u32,
    pub(crate) cell_size: f32,
    pub(crate) move_delay: f32,
    pub(crate) platform: P,
    pub(crate) snake_algorithm: Option<Box<dyn SnakeAlgorithm>>,
}

impl<P: Platform> GameLoop<P> {
    pub fn new(
        grid_width: u32,
        grid_height: u32,
        cell_size: f32,
        move_delay: f32,
        platform: P,
        snake_algorithm: Option<Box<dyn SnakeAlgorithm>>,
    ) -> Self {
        Self {
            snake: Snake::default(),
            apple: Apple::default(),
            state: GameState::Running,
            grid_width,
            grid_height,
            cell_size,
            move_delay,
            platform,
            snake_algorithm,
        }
    }
}
