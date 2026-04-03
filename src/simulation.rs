use crate::{
    boundary::Boundary,
    constants::{BUFFER_CAPACITY, NUM_PARTICLES},
    particle::{Particle, PseudoParticle},
    quadtree::QuadTree,
};
use rayon::prelude::*;

/// The top-level N-body simulation.
///
/// Owns all particles, a Barnes-Hut quadtree, and a set of per-particle
/// pseudo particle buffers. Each call to [`Simulation::step`] rebuilds the
/// tree, queries forces for every particle in parallel, and integrates their
/// positions and velocities using velocity Verlet integration.
pub struct Simulation {
    /// All particles in the simulation. Fixed size — no creation or destruction.
    pub particles: Vec<Particle>,
    /// Per-particle buffers of pseudo particles used to approximate gravitational forces.
    /// Cleared and repopulated every step. Reused across frames to avoid allocation.
    pseudo_particle_buffers: Vec<Vec<PseudoParticle>>,
    /// The Barnes-Hut quadtree. Rebuilt every step from current particle positions.
    tree: QuadTree,
}

impl Simulation {
    /// Constructs a new `Simulation` from an initial particle configuration.
    ///
    /// Initializes pseudo particle buffers and performs an initial tree build
    /// so the tree is valid before the first call to [`Simulation::step`].
    pub fn new(particles: Vec<Particle>) -> Self {
        let mut sim = Simulation {
            particles,
            pseudo_particle_buffers: (0..NUM_PARTICLES)
                .map(|_| Vec::with_capacity(BUFFER_CAPACITY))
                .collect(),
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
        let tree = &self.tree;
        self.particles[1..]
            .par_iter_mut()
            .zip(self.pseudo_particle_buffers.par_iter_mut())
            .for_each(|(p, buf): (&mut Particle, &mut Vec<PseudoParticle>)| {
                buf.clear();
                tree.query(&p.position, buf);
                p.update(buf);
            });
    }
}
