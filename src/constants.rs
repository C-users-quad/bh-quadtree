/// Maximum depth of the Barnes-Hut quadtree.
/// Nodes at this level will not subdivide further — overflow particles are dropped.
pub const MAX_LEVEL: usize = 6;

/// Maximum number of particles a leaf node can hold before subdividing.
pub const CAPACITY: usize = 128;

/// Precomputed `theta²` used in the Barnes-Hut criterion `s² < theta² * d²`.
/// Lower values yield more accurate but slower force approximations.
/// `theta = 0.75` is a commonly used default.
pub const THETA_2: f32 = 0.75 * 0.75;

/// Gravitational constant. Set to `1.0` for normalized simulation units.
pub const G: f32 = 1.0;

/// Softening factor added to squared distances to prevent singularities
/// when two particles are very close together.
pub const EPSILON: f32 = 1.0;

/// Number of particles in the simulation. Fixed at compile time.
pub const NUM_PARTICLES: usize = 10;

/// Simulation time step in seconds.
pub const DT: f32 = 0.01;
