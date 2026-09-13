use crate::{engine::{keyboard::Keyboard, mouse::Mouse}, utils::vec2::Vec2};

pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
    target_zoom: f32,
    speed: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Vec2::zero(),
            zoom: 0.0005,
            target_zoom: 0.0005,
            speed: 1.0,
        }
    }

    pub fn handle_scroll(&mut self, scroll_y: f32, keyboard: &Keyboard) {
        self.update_zoom_target(scroll_y, keyboard);
        self.update_speed(scroll_y, keyboard);
    }

    fn update_zoom_target(&mut self, scroll_y: f32, keyboard: &Keyboard) {
        if !keyboard.lctrl {
            if scroll_y > 0.0 {
                self.target_zoom *= 1.05;
                self.target_zoom = self.target_zoom.min(50.0);
            } else if scroll_y < 0.0 {
                self.target_zoom /= 1.05;
                self.target_zoom = self.target_zoom.max(0.00001);
            }
        }
    }

    /// pans the camera by dragging with the mouse. the world point under the
    /// cursor follows the cursor, so the scene is grabbed and moved around.
    pub fn pan(&mut self, mouse: &Mouse, screen_height: f32) {
        if !mouse.left_down {
            return;
        }
        let dx = mouse.pos.x - mouse.prev_pos.x;
        let dy = mouse.pos.y - mouse.prev_pos.y;
        self.pos.x -= 2.0 * dx / (self.zoom * screen_height);
        self.pos.y += 2.0 * dy / (self.zoom * screen_height);
    }

    /// smoothly interpolates zoom toward the target and re-anchors the camera
    /// so the world point currently under the cursor stays in place while zooming.
    pub fn update(&mut self, mouse: &Mouse, screen_size: (f32, f32)) {
        let old_zoom = self.zoom;
        self.zoom += (self.target_zoom - self.zoom) * 0.15;
        let new_zoom = self.zoom;
        if new_zoom == old_zoom {
            return;
        }
        let (w, h) = screen_size;
        let inv_delta = 1.0 / old_zoom - 1.0 / new_zoom;
        self.pos.x += 2.0 * (mouse.pos.x - w / 2.0) / h * inv_delta;
        self.pos.y += (1.0 - 2.0 * mouse.pos.y / h) * inv_delta;
    }

    fn update_speed(&mut self, scroll_y: f32, keyboard: &Keyboard) {
        if keyboard.lctrl {
            if scroll_y > 0.0 {
                self.speed += 0.1;
                self.speed = self.speed.min(50.0);
            } else if scroll_y < 0.0 {
                self.speed -= 0.1;
                self.speed = self.speed.max(1.0);
            }
        }
    }
}