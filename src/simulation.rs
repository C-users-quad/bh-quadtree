use crate::{
    boundary::Boundary,
    constants::NUM_PARTICLES,
    particle::{Particle, PseudoParticle},
    quadtree::QuadTree,
};

/// The top-level N-body simulation.
///
/// Owns all particles, a Barnes-Hut quadtree, and a set of per-particle
/// pseudo particle buffers. Each call to [`Simulation::step`] rebuilds the
/// tree, queries forces for every particle, and integrates their positions
/// and velocities using velocity Verlet integration.
pub struct Simulation {
    /// All particles in the simulation. Fixed size — no creation or destruction.
    particles: [Particle; NUM_PARTICLES],
    /// Per-particle buffers of pseudo particles used to approximate gravitational forces.
    /// Cleared and repopulated every step. Reused across frames to avoid allocation.
    pseudo_particle_buffers: [Vec<PseudoParticle>; NUM_PARTICLES],
    /// The Barnes-Hut quadtree. Rebuilt every step from current particle positions.
    tree: QuadTree,
}

impl Simulation {
    /// Constructs a new `Simulation` from an initial particle configuration.
    ///
    /// Initializes pseudo particle buffers and performs an initial tree build
    /// so the tree is valid before the first call to [`Simulation::step`].
    pub fn new(particles: [Particle; NUM_PARTICLES]) -> Self {
        let mut sim = Simulation {
            particles,
            pseudo_particle_buffers: std::array::from_fn(|_| Vec::new()),
            tree: QuadTree::new(Boundary::new(0.0, 0.0, 0.0, 0.0)),
        };
        sim.tree.build(&sim.particles);
        sim
    }

    /// Advances the simulation by one time step [`crate::constants::DT`].
    ///
    /// Rebuilds the quadtree from current particle positions, then updates
    /// all particles by querying forces and integrating.
    ///
    /// # Postconditions
    ///  - all particles have updated positions, velocities, and accelerations
    pub fn step(&mut self) {
        self.tree.build(&self.particles);
        self.update_particles();
    }

    /// Clears and repopulates every particle's pseudo particle buffer,
    /// then updates each particle's position, velocity, and acceleration.
    ///
    /// Split into two passes to satisfy the borrow checker — the first pass
    /// borrows `particles` immutably alongside `tree` and `buffers`, the second
    /// pass borrows `particles` mutably alongside `buffers` only.
    ///
    /// # Preconditions
    ///  - [`QuadTree::build`] has been called this step
    ///
    /// # Postconditions
    ///  - all particles have been updated
    ///  - all buffers reflect the current frame's pseudo particles
    fn update_particles(&mut self) {
        // first pass: query tree into each particle's buffer
        self.particles.iter().enumerate().for_each(|(i, p)| {
            self.pseudo_particle_buffers[i].clear();
            self.tree
                .query(&p.position, &mut self.pseudo_particle_buffers[i]);
        });

        // second pass: integrate each particle using its buffer
        self.particles.iter_mut().enumerate().for_each(|(i, p)| {
            p.update(&self.pseudo_particle_buffers[i]);
        });
    }
}
