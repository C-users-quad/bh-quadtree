use crate::{constants::DT_PHYSICS, vec2::Vec2};

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub radius: f32,
}

impl Particle {
    pub fn new(pos: Vec2, vel: Vec2, mass: f32, radius: f32) -> Self {
        Self {
            pos,
            vel,
            mass,
            radius,
            acc: Vec2::zero(),
        }
    }

    /// assumes self.acc stores previous frames acc
    pub fn integrate_pos(&mut self) {
        self.pos += self.vel * DT_PHYSICS + self.acc * DT_PHYSICS * DT_PHYSICS * 0.5;
    }

    /// assumes self.acc stores current acc
    pub fn integrate_vel(&mut self, old_acc: Vec2) {
        self.vel += (self.acc + old_acc) * DT_PHYSICS * 0.5;
    }
}
