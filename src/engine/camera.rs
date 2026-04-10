use crate::{engine::keyboard::Keyboard, utils::vec2::Vec2};

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
            zoom: 0.0001,
            target_zoom: 0.0001,
            speed: 1.0,
        }
    }

    pub fn handle_scroll(&mut self, scroll_y: f32, keyboard: &Keyboard) {
        self.update_zoom(scroll_y, keyboard);
        self.update_speed(scroll_y, keyboard);
    }

    fn update_zoom(&mut self, scroll_y: f32, keyboard: &Keyboard) {
        if !keyboard.lctrl {
            if scroll_y > 0.0 {
                self.target_zoom *= 1.05;
                self.target_zoom = self.target_zoom.min(50.0);
            } else if scroll_y < 0.0 {
                self.target_zoom /= 1.05;
                self.target_zoom = self.target_zoom.max(0.00001);
            }
        }
        // smoothly interpolate toward target zoom
        self.zoom += (self.target_zoom - self.zoom) * 0.15;
    }

    pub fn update_pos(&mut self, keyboard: &Keyboard, dt: f32) {
        let move_speed = self.speed / self.zoom;
        if keyboard.w {
            self.pos.y += move_speed * dt;
        }
        if keyboard.a {
            self.pos.x -= move_speed * dt;
        }
        if keyboard.s {
            self.pos.y -= move_speed * dt;
        }
        if keyboard.d {
            self.pos.x += move_speed * dt;
        }
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
