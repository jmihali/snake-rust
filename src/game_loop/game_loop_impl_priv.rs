use crate::game_loop::colors::*;
use crate::game_loop::error::{Error, Result};
use crate::game_loop::game_core::{Apple, Coordinates, Orientation, Snake};
use crate::game_loop::game_loop_decl::{GameEvent, GameLoop};
use crate::game_loop::platform::{MyKeyCode, Platform};

impl<P: Platform> GameLoop<P> {
    pub(crate) fn initialize_entities(&mut self) -> Result<()> {
        self.snake = Snake::new(Coordinates::new(1, 1), Orientation::East);
        self.apple =
            Apple::random_grid_except(self.grid_width, self.grid_height, self.snake.get_body())
                .ok_or(Error::FailedToSpawnApple)?;
        Ok(())
    }

    pub(crate) fn check_for_game_event(&self) -> Result<Option<GameEvent>> {
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

    pub(crate) fn is_apple_about_to_be_reached(&self) -> Result<bool> {
        let head = self
            .snake
            .get_head()
            .map_err(|_| Error::FailedToAdvanceSnake)?;
        let (dx, dy) = self.snake.get_head_orientation().get_delta();
        let next_head = Coordinates::new(head.get_x() + dx, head.get_y() + dy);
        Ok(next_head == *self.apple.get_coordinates())
    }

    pub(crate) fn draw_snake(&self) {
        for (i, coordinates) in self.snake.get_body().iter().enumerate() {
            let px = coordinates.get_x() as f32 * self.cell_size;
            let py = coordinates.get_y() as f32 * self.cell_size;

            let color = if i == 0 { GREEN } else { DARKGREEN };
            self.platform
                .draw_rectangle(px, py, self.cell_size, self.cell_size, color);
        }
    }

    pub(crate) fn draw_apple(&self) {
        let px =
            self.apple.get_coordinates().get_x() as f32 * self.cell_size + self.cell_size / 2.0;
        let py =
            self.apple.get_coordinates().get_y() as f32 * self.cell_size + self.cell_size / 2.0;
        self.platform.draw_circle(px, py, self.cell_size / 2.5, RED);
    }

    pub(crate) fn draw_background_grid(&self) {
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

    pub(crate) fn add_text_overlay(&self, text: &str, font_size: f32) {
        let screen_width = self.grid_width as f32 * self.cell_size;
        let screen_height = self.grid_height as f32 * self.cell_size;
        let text_width = self.platform.get_text_width(text, font_size);
        self.platform.draw_text(
            text,
            screen_width / 2.0 - text_width / 2.0,
            screen_height / 2.0,
            font_size,
            WHITE,
        );
    }

    pub(crate) fn get_orientation_from_input(&self) -> Option<Orientation> {
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
