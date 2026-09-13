use crate::utils::vec2::Vec2;

/// A particle struct, the essence of a \"body\" in this n-body sim.
/// Tuned to be exactly 32 bytes in size for cache friendliness.
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub size: f32,
}

impl Particle {
    pub fn new(pos: Vec2, vel: Vec2, mass: f32, radius: f32) -> Self {
        Self {
            pos,
            vel,
            mass,
            size: radius,
            acc: Vec2::zero(),
        }
    }

    /// assumes self.acc stores previous frames acc
    #[inline]
    pub fn integrate_pos(&mut self, dt: f32) {
        self.pos += self.vel * dt + self.acc * dt * dt * 0.5;
    }

    /// assumes self.acc stores current acc
    #[inline]
    pub fn integrate_vel(&mut self, old_acc: Vec2, dt: f32) {
        self.vel += (self.acc + old_acc) * dt * 0.5;
    }
}
