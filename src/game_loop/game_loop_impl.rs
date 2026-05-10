use std::process::exit;

use crate::game_loop::error::{Error, Result};
use crate::game_loop::game_core::{Apple, Coordinates, Orientation, Snake};
use crate::game_loop::rendering::Renderer;

use macroquad::prelude::*; // todo: remove macroquad dependency from this file

#[derive(Default)]
enum GameState {
    #[default]
    Running,
    GameOver,
}

pub struct GameLoop<R: Renderer> {
    snake: Snake,
    apple: Apple,
    state: GameState,
    grow: bool,
    grid_width: u32,
    grid_height: u32,
    cell_size: f32,
    move_delay: f32,
    renderer: R,
}

impl<R: Renderer> GameLoop<R> {
    fn initialize_entities(&mut self) -> Result<()> {
        self.snake = Snake::new(Coordinates::new(1, 1), Orientation::East);
        self.apple =
            Apple::random_grid_except(self.grid_width, self.grid_height, self.snake.get_body())
                .ok_or(Error::FailedToSpawnApple)?;
        Ok(())
    }

    fn update_game_state(&mut self) -> Result<()> {
        // todo: handle case where state is GameOver
        if self
            .snake
            .has_collided_with_edge(self.grid_width, self.grid_height)
            .map_err(|_| Error::FailedToCheckCollisionWithEdge)?
            || self
                .snake
                .has_collided_with_itself()
                .map_err(|_| Error::FailedToCheckCollisionWithItself)?
        {
            self.state = GameState::GameOver;
            return Ok(());
        }

        if self
            .snake
            .has_reached_apple(&self.apple)
            .map_err(|_| Error::FailedToCheckIfHasReachedApple)?
        {
            self.grow = true;
            self.apple =
                Apple::random_grid_except(self.grid_width, self.grid_height, self.snake.get_body())
                    .ok_or(Error::FailedToSpawnApple)?;
        } else {
            self.grow = false;
        }

        self.state = GameState::Running;

        Ok(())
    }
}

impl<R: Renderer> GameLoop<R> {
    pub fn new(
        grid_width: u32,
        grid_height: u32,
        cell_size: f32,
        move_delay: f32,
        renderer: R,
    ) -> Self {
        Self {
            snake: Snake::default(),
            apple: Apple::default(),
            state: GameState::Running,
            grow: false,
            grid_width,
            grid_height,
            cell_size,
            move_delay,
            renderer,
        }
    }

    pub async fn run_game_loop(&mut self) -> Result<()> {
        self.initialize_entities()?;

        let mut timer = 0.0;
        self.grow = false;

        self.renderer.set_screen_size(
            self.grid_width as f32 * self.cell_size,
            self.grid_height as f32 * self.cell_size,
        );

        loop {
            clear_background(BLACK);

            match self.state {
                GameState::Running => {
                    if let Some(orientation) = get_orientation_from_input() {
                        self.snake.set_head_orientation(orientation);
                    }

                    let dt = get_frame_time();
                    timer += dt;

                    if timer >= self.move_delay {
                        timer = 0.0;

                        self.snake
                            .advance(self.grow)
                            .map_err(|_| Error::FailedToAdvanceSnake)?;

                        self.update_game_state()?;
                    }
                }
                GameState::GameOver => {
                    // press Enter to restart, or Q to quit
                    if is_key_pressed(KeyCode::Enter) {
                        self.initialize_entities()?;
                        self.state = GameState::Running;
                        timer = 0.0;
                        self.grow = false;
                    } else if is_key_pressed(KeyCode::Q) {
                        exit(0);
                    }
                }
            }

            // draw grid (purely cosmetic)
            self.renderer
                .draw_background_grid(self.grid_width, self.grid_height, self.cell_size);
            self.renderer.draw_snake(&self.snake, self.cell_size);
            self.renderer.draw_apple(&self.apple, self.cell_size);
            next_frame().await;
        }
    }
}

fn get_orientation_from_input() -> Option<Orientation> {
    if is_key_pressed(KeyCode::Up) {
        return Some(Orientation::North);
    } else if is_key_pressed(KeyCode::Down) {
        return Some(Orientation::South);
    } else if is_key_pressed(KeyCode::Left) {
        return Some(Orientation::West);
    } else if is_key_pressed(KeyCode::Right) {
        return Some(Orientation::East);
    }
    None
}
