use crate::game::Apple;
use crate::game::Snake;
use macroquad::color::*;
use macroquad::shapes::*;

pub fn draw_snake(snake: &Snake, cell_size: f32) {
    for (i, coordinates) in snake.get_body().iter().enumerate() {
        let px = coordinates.get_x() as f32 * cell_size;
        let py = coordinates.get_y() as f32 * cell_size;

        let color = if i == 0 { GREEN } else { DARKGREEN };
        draw_rectangle(px, py, cell_size, cell_size, color);
    }
}

pub fn draw_apple(apple: &Apple, cell_size: f32) {
    let px = apple.get_coordinates().get_x() as f32 * cell_size + cell_size / 2.0;
    let py = apple.get_coordinates().get_y() as f32 * cell_size + cell_size / 2.0;
    draw_circle(px, py, cell_size / 2.5, RED);
}

pub fn draw_background_grid(width: u32, height: u32, cell_size: f32) {
    for i in 0..width {
        for j in 0..height {
            let x = i as f32 * cell_size;
            let y = j as f32 * cell_size;
            draw_rectangle_lines(x, y, cell_size, cell_size, 1.0, DARKGRAY);
        }
    }
}
