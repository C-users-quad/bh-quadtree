use std::time::Instant;

use crate::{
    physics::{constants::{EPSILON2, G, THETA2}, particle::Particle, quadnode::QuadNode},
    utils::{boundary::Boundary, rsqrt::rsqrt, vec2::Vec2},
};
// TODO: make insertion cheaper

pub struct QuadTree {
    pub nodes: Vec<QuadNode>,
}

impl QuadTree {
    /// index of root node in self.nodes
    pub const ROOT: usize = 0;

    pub fn new(particles: &[Particle]) -> Self {
        Self {
            nodes: Vec::with_capacity(particles.len() * 4),
        }
    }

    /// resets the quadtree by recreating the root and resetting the nodes vec.
    /// call this before calling any other methods on a quadtree.
    pub fn build(&mut self, particles: &[Particle]) {
        // get the min and max positions of all particles
        let (min_x, max_x, min_y, max_y) = particles.iter().fold(
            (
                f32::INFINITY,
                f32::NEG_INFINITY,
                f32::INFINITY,
                f32::NEG_INFINITY,
            ),
            |(min_x, max_x, min_y, max_y), p| {
                (
                    min_x.min(p.pos.x),
                    max_x.max(p.pos.x),
                    min_y.min(p.pos.y),
                    max_y.max(p.pos.y),
                )
            },
        );
        // make the size the longest side of the bounding box containing all particles
        let size = (max_x - min_x).max(max_y - min_y);
        // clear self.nodes in order to reset the quadtrees data
        self.nodes.clear();
        // create the root with a square boundary that contains all particles positions
        self.nodes
            .push(QuadNode::new(Boundary::new(min_x, min_y, size)));

        // insert all particles into the tree
        for p_idx in 0..particles.len() {
            self.insert(p_idx, particles);
        }
        // calculate every nodes center of mass
        self.calculate_com(particles, Self::ROOT);
    }

    /// inserts a particle into the tree. also construct the trees structure.
    pub fn insert(&mut self, p_idx: usize, particles: &[Particle]) {
        let mut node_idx = Self::ROOT;
        let p = &particles[p_idx];
        let p_mass = p.mass;
        let p_pos = p.pos;

        loop {
            let node = &self.nodes[node_idx];

            if node.is_leaf() {
                if node.is_empty() {
                    self.nodes[node_idx].set_particle_idx(p_idx);
                    break;
                } else if node.at_max_level() {
                    self.nodes[node_idx].mass += p_mass;
                    break;
                } else {
                    // store old particle index and subdivide the node
                    let old_p_idx = node.get_particle_idx();
                    self.subdivide(node_idx);

                    // place the old particle into the correct child node
                    let old_p = &particles[old_p_idx];
                    let old_p_child = self.nodes[node_idx].get_child_idx_of(old_p.pos);
                    self.nodes[old_p_child].set_particle_idx(old_p_idx);

                    // get the next child node to check for insertion
                    node_idx = self.nodes[node_idx].get_child_idx_of(p_pos);
                }
            } else {
                node_idx = node.get_child_idx_of(p_pos);
            }
        }
    }

    pub fn calculate_com(&mut self, particles: &[Particle], node_idx: usize) {
        let node = &mut self.nodes[node_idx];

        if node.is_leaf() {
            if node.is_empty() {
                return;
            }

            let p = &particles[node.get_particle_idx()];
            node.com = p.pos;
            node.mass = p.mass;
        } else {
            let next = node.get_child_idx();
            for child_idx in next..next + 4 {
                self.calculate_com(particles, child_idx);
                let delta_com = self.nodes[child_idx].com * self.nodes[child_idx].mass;
                self.nodes[node_idx].com += delta_com;
                self.nodes[node_idx].mass += self.nodes[child_idx].mass;
            }
            let total_mass = self.nodes[node_idx].mass;
            self.nodes[node_idx].com /= total_mass;
        }
    }

    /// increments `p.acc` in-place as it traverses the tree using the barnes-hut algorithm.
    pub fn calculate_acc(&self, p: &mut Particle, p_idx: usize) {
        let mut stack = [0usize; 128];
        let mut stack_ptr = 1;
        stack[0] = Self::ROOT;

        let p_pos = p.pos;
        let mut p_acc = Vec2::zero();

        while stack_ptr > 0 {
            stack_ptr -= 1;
            let node = &self.nodes[stack[stack_ptr]];

            let node_com = node.com;
            let node_mass = node.mass;

            if node_mass == 0.0 {
                continue;
            }

            let is_leaf = node.is_leaf();
            let d2 = node_com.d2(p_pos);
            if is_leaf || node.s2 < THETA2 * d2 {
                // avoids particles accelerating themselves
                if is_leaf && node.get_particle_idx() == p_idx {
                    continue;
                }
                let diff = node_com - p_pos;
                let inv_d = rsqrt(d2 + EPSILON2);
                p_acc += G * node_mass * inv_d * inv_d * inv_d * diff;
            } else {
                let next = node.get_child_idx();
                stack[stack_ptr] = next;
                stack[stack_ptr + 1] = next + 1;
                stack[stack_ptr + 2] = next + 2;
                stack[stack_ptr + 3] = next + 3;
                stack_ptr += 4;
            }
        }
        p.acc = p_acc;
    }

    fn subdivide(&mut self, node_idx: usize) {
        // flags parent node as branch and supplies it the index of its first child
        let next = self.nodes.len();
        let node = &mut self.nodes[node_idx];
        node.set_child_idx(next);

        // subdivide the node and push its children onto nodes
        let child_boundaries = node.boundary.subdivide();
        for b in child_boundaries {
            self.nodes.push(QuadNode::new(b));
        }
    }
}
