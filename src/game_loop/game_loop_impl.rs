use crate::game_loop::colors::*;
use crate::game_loop::error::{Error, Result};
use crate::game_loop::game_core::Apple;
use crate::game_loop::game_loop_decl::{GameEvent, GameLoop, GameState};
use crate::game_loop::platform::{MyKeyCode, Platform};

impl<P: Platform> GameLoop<P> {
    pub async fn run_game_loop(&mut self) -> Result<()> {
        self.initialize_entities()?;

        let mut timer = 0.0;
        let mut new_head_orientation = self.snake.get_head_orientation();

        self.platform.set_screen_size(
            self.grid_width as f32 * self.cell_size,
            self.grid_height as f32 * self.cell_size,
        );

        loop {
            self.platform.clear(BLACK);

            // Press Q to quit
            if self.platform.is_key_pressed(MyKeyCode::Q) {
                return Ok(());
            }

            match self.state {
                GameState::Running => {
                    if self.snake_algorithm.is_none()
                        && let Some(orientation) = self.get_orientation_from_input()
                    {
                        new_head_orientation = orientation;
                    }

                    let dt = self.platform.get_frame_time();
                    timer += dt;

                    if timer >= self.move_delay {
                        timer -= self.move_delay;

                        if let Some(algorithm) = &self.snake_algorithm
                            && let Some(orientation) = algorithm.decide_next_move(
                                &self.snake,
                                &self.apple,
                                self.grid_width,
                                self.grid_height,
                            )
                        {
                            new_head_orientation = orientation;
                        }

                        self.snake.set_head_orientation(new_head_orientation);

                        let grow = self.is_apple_about_to_be_reached()?;
                        self.snake
                            .advance(grow)
                            .map_err(|_| Error::FailedToAdvanceSnake)?;

                        if let Some(game_event) = self.check_for_game_event()? {
                            match game_event {
                                GameEvent::Collision => {
                                    self.state = GameState::GameOver;
                                }
                                GameEvent::FoodReached => {
                                    if let Some(apple) = Apple::random_grid_except(
                                        self.grid_width,
                                        self.grid_height,
                                        self.snake.get_body(),
                                    ) {
                                        self.apple = apple;
                                    } else {
                                        // if no apple can be spawned, it means that there is no space in the grid anymore
                                        self.state = GameState::GameWon;
                                    }
                                }
                            }
                        }
                    }
                }
                GameState::GameOver | GameState::GameWon => {
                    // press Enter to restart
                    if self.platform.is_key_pressed(MyKeyCode::Enter) {
                        self.initialize_entities()?;
                        self.state = GameState::Running;
                        timer = 0.0;
                    }
                }
            }

            // draw grid (purely cosmetic)
            self.draw_background_grid();
            self.draw_snake();
            self.draw_apple();

            if self.state == GameState::GameOver {
                self.add_text_overlay("Game Over! Press Enter to restart or Q to exit", 20.0);
            } else if self.state == GameState::GameWon {
                self.add_text_overlay(
                    "Congratulations! You won! Press Enter to restart or Q to exit",
                    20.0,
                );
            }

            self.platform.wait_for_frame().await;
        }
    }
}
