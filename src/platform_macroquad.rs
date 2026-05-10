use crate::game_loop::{MyColor, MyKeyCode, Platform};
use async_trait::async_trait;
use macroquad;

pub struct MacroquadPlatform {}

#[async_trait]
impl Platform for MacroquadPlatform {
    fn draw_rectangle(&self, x: f32, y: f32, width: f32, height: f32, color: MyColor) {
        macroquad::shapes::draw_rectangle(
            x,
            y,
            width,
            height,
            macroquad::color::Color::from_vec(color.into()),
        );
    }

    fn draw_circle(&self, x: f32, y: f32, radius: f32, color: MyColor) {
        macroquad::shapes::draw_circle(
            x,
            y,
            radius,
            macroquad::color::Color::from_vec(color.into()),
        );
    }

    fn draw_rectangle_lines(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        thickness: f32,
        color: MyColor,
    ) {
        macroquad::shapes::draw_rectangle_lines(
            x,
            y,
            width,
            height,
            thickness,
            macroquad::color::Color::from_vec(color.into()),
        );
    }

    fn set_screen_size(&self, width: f32, height: f32) {
        macroquad::window::request_new_screen_size(width, height);
    }

    fn clear(&self, color: MyColor) {
        macroquad::window::clear_background(macroquad::color::Color::from_vec(color.into()));
    }

    fn is_key_pressed(&self, key: MyKeyCode) -> bool {
        let key_macroquad = match key {
            MyKeyCode::Down => macroquad::input::KeyCode::Down,
            MyKeyCode::Up => macroquad::input::KeyCode::Up,
            MyKeyCode::Left => macroquad::input::KeyCode::Left,
            MyKeyCode::Right => macroquad::input::KeyCode::Right,
            MyKeyCode::Enter => macroquad::input::KeyCode::Enter,
            MyKeyCode::Q => macroquad::input::KeyCode::Q,
        };
        macroquad::input::is_key_pressed(key_macroquad)
    }

    fn get_frame_time(&self) -> f32 {
        macroquad::time::get_frame_time()
    }

    async fn wait_for_frame(&self) {
        macroquad::window::next_frame().await;
    }
}

impl MacroquadPlatform {
    pub fn new() -> Self {
        Self {}
    }
}
