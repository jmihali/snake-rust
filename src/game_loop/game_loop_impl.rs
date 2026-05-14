use crate::game_loop::error::{Error, Result};
use crate::game_loop::game_core::{Apple, Coordinates, Orientation, Snake};
use crate::game_loop::platform::{MyColor, MyKeyCode, Platform};
use crate::game_loop::snake_algorithm::SnakeAlgorithm;

pub const RED: MyColor = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: MyColor = [0.0, 1.0, 0.0, 1.0];
pub const DARKGREEN: MyColor = [0.0, 0.4, 0.0, 1.0];
pub const DARKGRAY: MyColor = [0.3, 0.3, 0.3, 1.0];
pub const BLACK: MyColor = [0.0, 0.0, 0.0, 1.0];

#[derive(Default, PartialEq, Eq)]
enum GameState {
    #[default]
    Running,
    GameOver,
    GameWon,
}

enum GameEvent {
    Collision,
    FoodReached,
}

pub struct GameLoop<P: Platform, A: SnakeAlgorithm> {
    snake: Snake,
    apple: Apple,
    state: GameState,
    grid_width: u32,
    grid_height: u32,
    cell_size: f32,
    move_delay: f32,
    platform: P,
    snake_algorithm: Option<A>,
}

impl<P: Platform, A: SnakeAlgorithm> GameLoop<P, A> {
    fn initialize_entities(&mut self) -> Result<()> {
        self.snake = Snake::new(Coordinates::new(1, 1), Orientation::East);
        self.apple =
            Apple::random_grid_except(self.grid_width, self.grid_height, self.snake.get_body())
                .ok_or(Error::FailedToSpawnApple)?;
        Ok(())
    }

    fn check_for_game_event(&self) -> Result<Option<GameEvent>> {
        if self
            .snake
            .has_collided_with_edge(self.grid_width, self.grid_height)
            .map_err(|_| Error::FailedToCheckCollisionWithEdge)?
            || self
                .snake
                .has_collided_with_itself()
                .map_err(|_| Error::FailedToCheckCollisionWithItself)?
        {
            return Ok(Some(GameEvent::Collision));
        }

        if self
            .snake
            .has_reached_apple(&self.apple)
            .map_err(|_| Error::FailedToCheckIfHasReachedApple)?
        {
            return Ok(Some(GameEvent::FoodReached));
        }

        Ok(None)
    }

    fn draw_snake(&self) {
        for (i, coordinates) in self.snake.get_body().iter().enumerate() {
            let px = coordinates.get_x() as f32 * self.cell_size;
            let py = coordinates.get_y() as f32 * self.cell_size;

            let color = if i == 0 { GREEN } else { DARKGREEN };
            self.platform
                .draw_rectangle(px, py, self.cell_size, self.cell_size, color);
        }
    }

    fn draw_apple(&self) {
        let px =
            self.apple.get_coordinates().get_x() as f32 * self.cell_size + self.cell_size / 2.0;
        let py =
            self.apple.get_coordinates().get_y() as f32 * self.cell_size + self.cell_size / 2.0;
        self.platform.draw_circle(px, py, self.cell_size / 2.5, RED);
    }

    fn draw_background_grid(&self) {
        for i in 0..self.grid_width {
            for j in 0..self.grid_height {
                let x = i as f32 * self.cell_size;
                let y = j as f32 * self.cell_size;
                self.platform.draw_rectangle_lines(
                    x,
                    y,
                    self.cell_size,
                    self.cell_size,
                    1.0,
                    DARKGRAY,
                );
            }
        }
    }

    fn get_orientation_from_input(&self) -> Option<Orientation> {
        if self.platform.is_key_pressed(MyKeyCode::Up) {
            return Some(Orientation::North);
        } else if self.platform.is_key_pressed(MyKeyCode::Down) {
            return Some(Orientation::South);
        } else if self.platform.is_key_pressed(MyKeyCode::Left) {
            return Some(Orientation::West);
        } else if self.platform.is_key_pressed(MyKeyCode::Right) {
            return Some(Orientation::East);
        }
        None
    }
}

impl<P: Platform, A: SnakeAlgorithm> GameLoop<P, A> {
    pub fn new(
        grid_width: u32,
        grid_height: u32,
        cell_size: f32,
        move_delay: f32,
        platform: P,
        snake_algorithm: Option<A>,
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

    pub async fn run_game_loop(&mut self) -> Result<()> {
        self.initialize_entities()?;

        let mut timer = 0.0;
        let mut grow = false;
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
                    if self.snake_algorithm.is_none() {
                        if let Some(orientation) = self.get_orientation_from_input() {
                            new_head_orientation = orientation;
                        }
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
                        self.snake
                            .advance(grow)
                            .map_err(|_| Error::FailedToAdvanceSnake)?;
                        grow = false;

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
                                        grow = true;
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
                    if self.state == GameState::GameWon {
                        println!(
                            "CONGRATULATIONS! YOU WON THE GAME! :) Press Enter to restart or Q to exit"
                        );
                    }

                    // press Enter to restart
                    if self.platform.is_key_pressed(MyKeyCode::Enter) {
                        self.initialize_entities()?;
                        self.state = GameState::Running;
                        timer = 0.0;
                        grow = false;
                    }
                }
            }

            // draw grid (purely cosmetic)
            self.draw_background_grid();
            self.draw_snake();
            self.draw_apple();

            self.platform.wait_for_frame().await;
        }
    }
}
