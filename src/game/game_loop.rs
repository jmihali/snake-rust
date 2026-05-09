use crate::game::apple::*;
use crate::game::coordinates::*;
use crate::game::orientation::*;
use crate::game::rendering::*;
use crate::game::snake::*;
use macroquad::prelude::*;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, Error>;

enum GameState {
    Running,
    GameOver,
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

fn update_game_state(
    snake: &mut Snake,
    apple: &mut Apple,
    grid_width: u32,
    grid_height: u32,
    grow: &mut bool,
    state: &mut GameState,
) -> Result<()> {
    // todo: handle case where state is GameOver
    snake.advance(*grow)?;

    if snake.has_collided_with_grid(grid_width, grid_height)? || snake.has_collided_with_itself()? {
        *state = GameState::GameOver;
        return Ok(());
    }

    if snake.has_reached_apple(apple)? {
        *grow = true;
        *apple = Apple::random_grid_except(grid_width, grid_height, snake.get_body()).unwrap();
    } else {
        *grow = false;
    }

    *state = GameState::Running;

    Ok(())
}

pub async fn run_game_loop(
    grid_width: u32,
    grid_height: u32,
    cell_size: f32,
    move_delay: f32,
) -> Result<()> {
    let mut snake = Snake::new(Coordinates::new(1, 1), Orientation::East);
    let mut apple = Apple::random_grid_except(grid_width, grid_height, snake.get_body()).unwrap();

    let mut state = GameState::Running;
    let mut timer = 0.0;
    let mut grow = false;

    request_new_screen_size(
        grid_width as f32 * cell_size,
        grid_height as f32 * cell_size,
    );

    loop {
        clear_background(BLACK);

        match state {
            GameState::Running => {
                if let Some(orientation) = get_orientation_from_input() {
                    snake.set_head_orientation(orientation);
                }

                let dt = get_frame_time();
                timer += dt;

                if timer >= move_delay {
                    timer = 0.0;

                    update_game_state(
                        &mut snake,
                        &mut apple,
                        grid_width,
                        grid_height,
                        &mut grow,
                        &mut state,
                    )?;
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
        draw_background_grid(grid_width, grid_height, cell_size);
        draw_snake(&snake, cell_size);
        draw_apple(&apple, cell_size);
        next_frame().await;
    }
}
