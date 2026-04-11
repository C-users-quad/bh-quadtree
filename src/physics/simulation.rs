use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::{physics::particle::Particle, physics::quadtree::QuadTree, utils::presets::Presets};

pub struct Simulation {
    pub tree: QuadTree,
    pub particles: Vec<Particle>,
    pub paused: bool,
}

impl Simulation {
    pub fn new(preset: Presets) -> Self {
        let particles = preset.get_particles();
        Self {
            tree: QuadTree::new(&particles),
            particles,
            paused: false,
        }
    }

    pub fn load_preset(&mut self, preset: Presets) {
        self.particles = preset.get_particles();
    }

    pub fn step(&mut self) {
        if self.paused {
            return;
        }

        // build the tree
        self.tree.build(&self.particles);

        // update particles
        let tree = &self.tree;
        self.particles
            .par_chunks_mut(1024)
            .enumerate()
            .for_each(|(chunk_i, chunk)| {
                for (p_idx, p) in chunk.iter_mut().enumerate() {
                    let actual_p_idx = p_idx + chunk_i * 1024;
                    p.integrate_pos();
                    let old_acc = p.acc;
                    tree.calculate_acc(p, actual_p_idx);
                    p.integrate_vel(old_acc);
                }
            });
    }
}
