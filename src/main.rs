mod game;

use game::*;

use macroquad::prelude::*;
use macroquad::rand::gen_range;

const CELL_SIZE: f32 = 20.0;
const GRID_WIDTH: u32 = 20;
const GRID_HEIGHT: u32 = 20;

enum GameState {
    Running,
    GameOver,
}

pub fn draw_snake(snake: &Snake) {
    for (i, coordinates) in snake.get_body().iter().enumerate() {
        let px = coordinates.get_x() as f32 * CELL_SIZE;
        let py = coordinates.get_y() as f32 * CELL_SIZE;

        let color = if i == 0 { GREEN } else { DARKGREEN };
        draw_rectangle(px, py, CELL_SIZE, CELL_SIZE, color);
    }
}

pub fn draw_apple(apple: &Apple) {
    let px = apple.get_coordinates().get_x() as f32 * CELL_SIZE + CELL_SIZE / 2.0;
    let py = apple.get_coordinates().get_y() as f32 * CELL_SIZE + CELL_SIZE / 2.0;
    draw_circle(px, py, CELL_SIZE / 2.5, RED);
}

pub fn draw_grid(width: u32, height: u32, cell_size: f32) {
    for i in 0..width {
        for j in 0..height {
            let x = i as f32 * cell_size;
            let y = j as f32 * cell_size;
            draw_rectangle_lines(x, y, cell_size, cell_size, 1.0, DARKGRAY);
        }
    }
}

pub fn has_snake_reached_apple(snake: &Snake, apple: &Apple) -> bool {
    // todo: do not use unwrap
    snake.get_head().unwrap() == apple.get_coordinates()
}

#[macroquad::main("Snake in Rust")]
async fn main() {
    let mut snake = Snake::new(Coordinates::new(1, 1), Orientation::East);
    let mut apple = Apple::random(GRID_WIDTH as i32, GRID_HEIGHT as i32, &snake);

    let mut state = GameState::Running;
    let mut timer = 0.0;
    let move_delay = 0.3; // seconds between moves

    loop {
        let mut grow = false;

        clear_background(BLACK);

        match state {
            GameState::Running => {
                if is_key_pressed(KeyCode::Up) {
                    snake.set_head_orientation(Orientation::North);
                } else if is_key_pressed(KeyCode::Down) {
                    snake.set_head_orientation(Orientation::South);
                } else if is_key_pressed(KeyCode::Left) {
                    snake.set_head_orientation(Orientation::West);
                } else if is_key_pressed(KeyCode::Right) {
                    snake.set_head_orientation(Orientation::East);
                }

                let dt = get_frame_time();
                timer += dt;

                if timer >= move_delay {
                    timer = 0.0;

                    // todo: remove unwrap
                    if snake.has_collided(GRID_WIDTH, GRID_HEIGHT).unwrap() {
                        state = GameState::GameOver;
                    } else if snake.has_collided_with_itself().unwrap() {
                        state = GameState::GameOver;
                    } else if has_snake_reached_apple(&snake, &apple) {
                        grow = true;
                        apple = Apple::random(GRID_WIDTH as i32, GRID_HEIGHT as i32, &snake);
                    }

                    snake.advance(grow);
                }
            }
            GameState::GameOver => {
                // press Enter to restart
                if is_key_pressed(KeyCode::Enter) {
                    snake = Snake::new(Coordinates::new(1, 1), Orientation::East);
                    apple = Apple::random(GRID_WIDTH as i32, GRID_HEIGHT as i32, &snake);
                    state = GameState::Running;
                    timer = 0.0;
                }
            }
        }

        // draw grid (purely cosmetic)
        draw_grid(GRID_WIDTH, GRID_HEIGHT, CELL_SIZE);

        draw_snake(&snake);
        draw_apple(&apple);
        next_frame().await;
    }
}
