use crate::game::apple::*;
use crate::game::coordinates::*;
use crate::game::orientation::*;
use crate::game::snake::*;

use macroquad::prelude::*;

enum GameState {
    Running,
    GameOver,
}

fn draw_snake(snake: &Snake, cell_size: f32) {
    for (i, coordinates) in snake.get_body().iter().enumerate() {
        let px = coordinates.get_x() as f32 * cell_size;
        let py = coordinates.get_y() as f32 * cell_size;

        let color = if i == 0 { GREEN } else { DARKGREEN };
        draw_rectangle(px, py, cell_size, cell_size, color);
    }
}

fn draw_apple(apple: &Apple, cell_size: f32) {
    let px = apple.get_coordinates().get_x() as f32 * cell_size + cell_size / 2.0;
    let py = apple.get_coordinates().get_y() as f32 * cell_size + cell_size / 2.0;
    draw_circle(px, py, cell_size / 2.5, RED);
}

fn draw_grid(width: u32, height: u32, cell_size: f32) {
    for i in 0..width {
        for j in 0..height {
            let x = i as f32 * cell_size;
            let y = j as f32 * cell_size;
            draw_rectangle_lines(x, y, cell_size, cell_size, 1.0, DARKGRAY);
        }
    }
}

pub async fn run_game_loop(grid_width: u32, grid_height: u32, cell_size: f32) {
    let mut snake = Snake::new(Coordinates::new(1, 1), Orientation::East);
    let mut apple = Apple::random_grid_except(grid_width, grid_height, snake.get_body()).unwrap();

    let mut state = GameState::Running;
    let mut timer = 0.0;
    let move_delay = 0.15; // seconds between moves

    request_new_screen_size(
        grid_width as f32 * cell_size,
        grid_height as f32 * cell_size,
    );

    loop {
        let mut grow = false;

        clear_background(BLACK);

        match state {
            GameState::Running => {
                if is_key_pressed(KeyCode::Up) {
                    snake.set_head_orientation(Orientation::North).unwrap();
                } else if is_key_pressed(KeyCode::Down) {
                    snake.set_head_orientation(Orientation::South).unwrap();
                } else if is_key_pressed(KeyCode::Left) {
                    snake.set_head_orientation(Orientation::West).unwrap();
                } else if is_key_pressed(KeyCode::Right) {
                    snake.set_head_orientation(Orientation::East).unwrap();
                }

                let dt = get_frame_time();
                timer += dt;

                if timer >= move_delay {
                    timer = 0.0;

                    // todo: remove unwrap
                    if snake
                        .has_collided_with_grid(grid_width, grid_height)
                        .unwrap()
                    {
                        state = GameState::GameOver;
                    } else if snake.has_collided_with_itself().unwrap() {
                        state = GameState::GameOver;
                    } else if snake.has_reached_apple(&apple).unwrap() {
                        grow = true;
                        apple =
                            Apple::random_grid_except(grid_width, grid_height, snake.get_body())
                                .unwrap();
                    }

                    snake.advance(grow).unwrap();
                }
            }
            GameState::GameOver => {
                // press Enter to restart
                if is_key_pressed(KeyCode::Enter) {
                    snake = Snake::new(Coordinates::new(1, 1), Orientation::East);
                    apple = Apple::random_grid_except(grid_width, grid_height, snake.get_body())
                        .unwrap();
                    state = GameState::Running;
                    timer = 0.0;
                }
            }
        }

        // draw grid (purely cosmetic)
        draw_grid(grid_width, grid_height, cell_size);
        draw_snake(&snake, cell_size);
        draw_apple(&apple, cell_size);
        next_frame().await;
    }
}
