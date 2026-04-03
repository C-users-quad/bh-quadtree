use crate::{
    constants::{DT, EPSILON, G},
    vec_2::Vec2,
};
use std::fmt::Debug;

/// A particle in the simulation with position, velocity, acceleration, mass, and radius.
///
/// Particles are updated each frame using velocity Verlet integration,
/// with gravitational forces approximated via the Barnes-Hut algorithm.
#[derive(Default, Debug)]
pub struct Particle {
    /// Position of the particle in world space.
    pub position: Vec2,
    /// Velocity of the particle.
    pub velocity: Vec2,
    /// Acceleration of the particle. Updated every frame by [`Particle::integrate_velocity`].
    pub acceleration: Vec2,
    /// Mass of the particle. Used in gravitational force calculations.
    pub mass: f32,
    /// Radius of the particle. Used for collision detection.
    pub radius: f32,
}

impl Particle {
    /// Constructs a new `Particle` with the given properties.
    pub fn new(position: Vec2, velocity: Vec2, mass: f32, radius: f32) -> Self {
        Particle {
            position,
            velocity,
            acceleration: Vec2::default(),
            mass,
            radius,
        }
    }

    /// Updates the particle's position, velocity, and acceleration
    /// for one simulation step.
    ///
    /// Integrates position using the current acceleration, then computes a new
    /// acceleration from `buffer` and integrates velocity using both old and new
    /// accelerations (velocity Verlet).
    ///
    /// # Preconditions
    ///  - `buffer` is populated with [`PseudoParticle`]s
    ///    — call [`crate::quadtree::QuadTree::query`]
    ///
    /// # Postconditions
    ///  - `self.position`, `self.velocity`, and `self.acceleration` have been updated.
    pub fn update(&mut self, buffer: &[PseudoParticle]) {
        self.integrate_position();
        self.integrate_velocity(buffer);
    }

    /// Computes the gravitational acceleration on this particle from `buffer`.
    ///
    /// For each pseudo particle, computes the acceleration contribution:
    /// `a = G * m / (d² + ε)^(3/2) * direction`
    /// where `ε` ([`EPSILON`]) is a softening factor that prevents singularities
    /// when two particles are very close.
    ///
    /// # Returns
    ///  - [`Vec2`] representing the total gravitational acceleration
    fn compute_acceleration(&self, buffer: &[PseudoParticle]) -> Vec2 {
        let (ax, ay) = buffer.iter().fold((0.0, 0.0), |(ax, ay), p| {
            let dx = p.position.x - self.position.x;
            let dy = p.position.y - self.position.y;
            let d2 = dx * dx + dy * dy + EPSILON;
            let inv_d3 = 1.0 / (d2 * d2.sqrt());
            let intermediate = G * p.mass * inv_d3;
            (ax + intermediate * dx, ay + intermediate * dy)
        });
        Vec2::new(ax, ay)
    }

    /// Mutates `self.position` according to velocity Verlet.
    ///
    /// `position += velocity * dt + acceleration * 0.5 * dt²`
    ///
    /// # Postconditions
    ///  - `self.position` contains the new position of the particle
    fn integrate_position(&mut self) {
        self.position += self.velocity * DT + self.acceleration * DT * DT * 0.5;
    }

    /// Mutates `self.velocity` and `self.acceleration` according to velocity Verlet.
    ///
    /// `velocity += (old_acceleration + new_acceleration) * 0.5 * dt`
    ///
    /// # Preconditions
    ///  - [`Particle::integrate_position`] has been called this frame
    ///
    /// # Postconditions
    ///  - `self.velocity` contains the new velocity of the particle
    ///  - `self.acceleration` contains the new acceleration of the particle
    fn integrate_velocity(&mut self, buffer: &[PseudoParticle]) {
        let new_acc = self.compute_acceleration(buffer);
        self.velocity += (self.acceleration + new_acc) * DT * 0.5;
        self.acceleration = new_acc;
    }
}

/// A mass aggregate used to approximate gravitational forces via the Barnes-Hut algorithm.
///
/// Represents either a single particle or an entire region of space collapsed
/// to its center of mass, depending on the Barnes-Hut criterion.
#[derive(Debug)]
pub struct PseudoParticle {
    /// Position of the center of mass.
    pub position: Vec2,
    /// Total mass represented by this pseudo particle.
    pub mass: f32,
}
