use std::ops::{Add, AddAssign, Div, DivAssign, Mul};

/// A 2D vector with `f32` components.
///
/// Used throughout the simulation to represent positions, velocities,
/// accelerations, and centers of mass. Supports standard arithmetic
/// operations via operator overloading.
#[derive(Default, Copy, Clone, Debug)]
pub struct Vec2 {
    /// Horizontal component.
    pub x: f32,
    /// Vertical component.
    pub y: f32,
}

impl Vec2 {
    /// Constructs a new `Vec2` with the given components.
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    /// Returns the squared distance between this vector and `other`.
    ///
    /// Prefer this over computing actual distance when only relative
    /// distances are needed, as it avoids an expensive square root.
    pub fn distance_squared(&self, other: &Vec2) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        dx * dx + dy * dy
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// Scales a `Vec2` by a scalar: `v * s`.
impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

/// Adds two `Vec2`s component-wise: `a + b`.
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

/// Adds a `Vec2` to this one in place: `a += b`.
impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

/// Divides a `Vec2` by a scalar: `v / s`
impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x / scalar, self.y / scalar)
    }
}

/// Divides a `Vec2` by a scalar in-place: `v /= s`
impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}
