use glium::winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton},
};

use crate::utils::vec2::Vec2;

/// tracks mouse state relevant to camera controls.
/// `pos` holds the current cursor position in physical pixels, `prev_pos`
/// the position as of the end of the previous frame so camera panning can
/// measure how far the mouse moved between frames.
#[derive(Copy, Clone)]
pub struct Mouse {
    pub left_down: bool,
    pub pos: Vec2,
    pub prev_pos: Vec2,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            left_down: false,
            pos: Vec2::zero(),
            prev_pos: Vec2::zero(),
        }
    }

    pub fn handle_button_input(&mut self, state: ElementState, button: MouseButton) {
        if button == MouseButton::Left {
            self.left_down = state == ElementState::Pressed;
        }
    }

    pub fn handle_cursor_moved(&mut self, position: PhysicalPosition<f64>) {
        self.pos.set(position.x as f32, position.y as f32);
    }

    /// stores the current cursor position as the previous position so that
    /// camera panning can detect the mouse movement between frames.
    pub fn end_frame(&mut self) {
        self.prev_pos = self.pos;
    }
}