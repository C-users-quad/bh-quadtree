/// Maximum depth of the Barnes-Hut quadtree.
/// Nodes at this level will not subdivide further — overflow particles are dropped.
pub const MAX_LEVEL: usize = 6;

/// Maximum number of particles a leaf node can hold before subdividing.
pub const CAPACITY: usize = 10;

/// Precomputed `theta²` used in the Barnes-Hut criterion `s² < theta² * d²`.
/// Lower values yield more accurate but slower force approximations.
/// `theta = 0.75` is a commonly used default.
pub const THETA_2: f32 = 0.75 * 0.75;

/// Gravitational constant. Set to `1.0` for normalized simulation units.
pub const G: f32 = 5.0;

/// Softening factor added to squared distances to prevent singularities
/// when two particles are very close together.
pub const EPSILON: f32 = 1.0;

/// Number of particles in the simulation. Fixed at compile time.
pub const NUM_PARTICLES: usize = 70_000;

/// Simulation time step in seconds.
pub const DT: f32 = 0.01;

/// An estimate of how many pseudo particles will be returned by querying the quadtree.
/// Used to reduce heavy memory reallocations during the first few frames when the
/// buffers are being constructed.
pub const BUFFER_CAPACITY: usize = 128;

/// The mass of the central particle.
pub const CENTRAL_MASS: f32 = 1_000_000_000f32;

/// The radius of the central particle.
pub const CENTRAL_RADIUS: f32 = 100f32;

/// The mass of the orbital particles
pub const ORBITAL_MASS: f32 = 1f32;

/// The radius of the orbital particles.
pub const ORBITAL_RADIUS: f32 = 1f32;

/// The maximum radius of the uniform disk that the
/// particles are initialized in, squared.
pub const MAX_DISK_RADIUS_2: f32 = 2500f32 * 2500f32;

/// The minimum radius that particles can be initialized
/// at in the uniform disk, squared.
pub const MIN_DISK_RADIUS_2: f32 = (CENTRAL_RADIUS + 100f32) * (CENTRAL_RADIUS + 100f32);
