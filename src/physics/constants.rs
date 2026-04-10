/// minimum node size. used to prevent infinite levels.
pub const MIN_NODE_SIZE: f32 = 0.1;

/// value from [0,∞), where 0 is least accurate bh-approximation, and ∞ is most accurate.
/// A sensible value is 1.0, which is a good balance between performance and accuracy.
pub const THETA2: f32 = 1.5 * 1.5;

/// used to prevent forces from blowing up when particles are real close
pub const EPSILON2: f32 = 25.0 * 25.0;

/// gravitational constant
pub const G: f32 = 1.0;

/// radius of a central, more massive particle
pub const CENTRAL_RADIUS: f32 = 25.0;

/// radius of a orbital particle with negligible mass
pub const ORBITAL_RADIUS: f32 = 1.0;

/// dt used for velocity verlet and physics math in general
pub const DT_PHYSICS: f32 = 0.01;

/// number orbital particles during initialization
pub const NUM_PARTICLES: u32 = 125_000;
