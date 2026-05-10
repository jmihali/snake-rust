use crate::game_loop::game_core::{Apple, Snake};

pub trait Renderer {
    fn draw_snake(&self, snake: &Snake, cell_size: f32);
    fn draw_apple(&self, apple: &Apple, cell_size: f32);
    fn draw_background_grid(&self, width: u32, height: u32, cell_size: f32);
    fn set_screen_size(&self, width: f32, height: f32);
}
