use async_trait::async_trait;

pub type MyColor = [f32; 4];

pub enum MyKeyCode {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Q,
}

#[async_trait(?Send)] // Use the async-trait crate if you want async in traits
pub trait Platform {
    // Rendering
    fn draw_rectangle(&self, x: f32, y: f32, width: f32, height: f32, color: MyColor);
    fn draw_circle(&self, x: f32, y: f32, radius: f32, color: MyColor);
    fn draw_rectangle_lines(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        thickness: f32,
        color: MyColor,
    );

    fn set_screen_size(&self, width: f32, height: f32);

    fn clear(&self, color: MyColor);

    fn get_frame_time(&self) -> f32;

    fn is_key_pressed(&self, key: MyKeyCode) -> bool;

    async fn wait_for_frame(&self);
}
