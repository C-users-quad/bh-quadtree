use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{particle::Particle, quadtree::QuadTree, vec2::Vec2};

pub struct Simulation {
    pub tree: QuadTree,
    pub particles: Vec<Particle>,
    pub paused: bool,
}

impl Simulation {
    pub fn new(particles: Vec<Particle>) -> Self {
        Self {
            tree: QuadTree::new(&particles),
            particles,
            paused: false,
        }
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
            .par_iter_mut()
            .enumerate()
            .for_each(|(p_idx, p)| {
                p.integrate_pos();
                let old_acc = p.acc;
                p.acc = Vec2::zero();
                tree.calculate_acc(p, QuadTree::ROOT, p_idx);
                p.integrate_vel(old_acc);
            });
    }
}
