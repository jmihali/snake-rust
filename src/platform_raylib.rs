use crate::game_loop::{MyColor, MyKeyCode, Platform};
use async_trait::async_trait;
use raylib::prelude::*;
use std::cell::RefCell;

pub struct RaylibPlatform {
    // We need the handle to check input and start drawing
    rl: RefCell<RaylibHandle>,
    // The thread token required by Raylib for many operations
    thread: RaylibThread,
    // We store the active drawing context here once 'clear' is called
    draw_handle: RefCell<Option<RaylibDrawHandle<'static>>>,
}

#[async_trait(?Send)]
impl Platform for RaylibPlatform {
    fn clear(&self, color: MyColor) {
        // In Raylib, we MUST begin drawing to clear the screen.
        // We drop any existing handle and start a new one.
        let mut rl = self.rl.borrow_mut();
        let mut d = rl.begin_drawing(&self.thread);

        d.clear_background(raylib::color::Color::color_from_normalized(Vector4::new(
            color[0], color[1], color[2], color[3],
        )));

        // We "cheat" the lifetimes by using unsafe to store the handle.
        // This is necessary because your trait is imperative, but Raylib is scope-based.
        unsafe {
            let static_d =
                std::mem::transmute::<RaylibDrawHandle<'_>, RaylibDrawHandle<'static>>(d);
            *self.draw_handle.borrow_mut() = Some(static_d);
        }
    }

    fn draw_rectangle(&self, x: f32, y: f32, width: f32, height: f32, color: MyColor) {
        if let Some(ref mut d) = *self.draw_handle.borrow_mut() {
            d.draw_rectangle(
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                raylib::color::Color::color_from_normalized(Vector4::new(
                    color[0], color[1], color[2], color[3],
                )),
            );
        }
    }

    fn draw_circle(&self, x: f32, y: f32, radius: f32, color: MyColor) {
        if let Some(ref mut d) = *self.draw_handle.borrow_mut() {
            d.draw_circle(
                x as i32,
                y as i32,
                radius,
                raylib::color::Color::color_from_normalized(Vector4::new(
                    color[0], color[1], color[2], color[3],
                )),
            );
        }
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
        if let Some(ref mut d) = *self.draw_handle.borrow_mut() {
            // Raylib's draw_rectangle_lines doesn't take thickness,
            // so we use draw_rectangle_lines_ex
            let rect = Rectangle::new(x, y, width, height);
            d.draw_rectangle_lines_ex(
                rect,
                thickness,
                raylib::color::Color::color_from_normalized(Vector4::new(
                    color[0], color[1], color[2], color[3],
                )),
            );
        }
    }

    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: MyColor) {
        if let Some(ref mut d) = *self.draw_handle.borrow_mut() {
            d.draw_text(
                text,
                x as i32,
                y as i32,
                font_size as i32,
                raylib::color::Color::color_from_normalized(Vector4::new(
                    color[0], color[1], color[2], color[3],
                )),
            );
        }
    }

    fn get_text_width(&self, text: &str, font_size: f32) -> f32 {
        raylib::text::measure_text(text, font_size as i32)
    }

    fn set_screen_size(&self, width: f32, height: f32) {
        let mut rl = self.rl.borrow_mut();
        rl.set_window_size(width as i32, height as i32);
    }

    fn get_frame_time(&self) -> f32 {
        self.rl.borrow().get_frame_time()
    }

    fn is_key_pressed(&self, key: MyKeyCode) -> bool {
        self.rl.borrow().is_key_pressed(Self::map_key(key))
    }

    async fn wait_for_frame(&self) {
        // 1. End the drawing by dropping the handle
        self.draw_handle.borrow_mut().take();

        // 2. Yield to the async executor
        tokio::task::yield_now().await;
    }
}

impl RaylibPlatform {
    pub fn new(width: i32, height: i32, title: &str) -> Self {
        // 1. Initialize the Raylib window
        let (mut rl, thread) = raylib::init().size(width, height).title(title).build();

        // 2. Optional: Set a target FPS to prevent the CPU from
        // running at 100% in our async loop.
        rl.set_target_fps(60);

        Self {
            rl: RefCell::new(rl),
            thread,
            draw_handle: RefCell::new(None),
        }
    }

    // Helper to map our custom keys to Raylib keys
    fn map_key(key: MyKeyCode) -> KeyboardKey {
        match key {
            MyKeyCode::Up => KeyboardKey::KEY_UP,
            MyKeyCode::Down => KeyboardKey::KEY_DOWN,
            MyKeyCode::Left => KeyboardKey::KEY_LEFT,
            MyKeyCode::Right => KeyboardKey::KEY_RIGHT,
            MyKeyCode::Enter => KeyboardKey::KEY_ENTER,
            MyKeyCode::Q => KeyboardKey::KEY_Q,
        }
    }
}
