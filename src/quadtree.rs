use crate::{
    boundary::Boundary,
    constants::{EPSILON, G, THETA2},
    particle::Particle,
    quadnode::QuadNode,
};

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

        loop {
            if self.nodes[node_idx].is_leaf() {
                if self.nodes[node_idx].is_empty() {
                    self.nodes[node_idx].set_particle_idx(p_idx);
                    break;
                } else if self.nodes[node_idx].at_max_level() {
                    self.nodes[node_idx].mass += p.mass;
                    break;
                } else {
                    // store old particle index and subdivide the node
                    let old_p_idx = self.nodes[node_idx].get_particle_idx();
                    self.subdivide(node_idx);

                    // place the old particle into the correct child node
                    let old_p = &particles[old_p_idx];
                    let old_p_child = self.nodes[node_idx].get_child_idx_of(old_p.pos);
                    self.nodes[old_p_child].set_particle_idx(old_p_idx);

                    // get the next child node to check for insertion
                    node_idx = self.nodes[node_idx].get_child_idx_of(p.pos);
                }
            } else {
                node_idx = self.nodes[node_idx].get_child_idx_of(p.pos);
            }
        }
    }

    pub fn calculate_com(&mut self, particles: &[Particle], node_idx: usize) {
        if self.nodes[node_idx].is_leaf() {
            if self.nodes[node_idx].is_empty() {
                return;
            }

            let p = &particles[self.nodes[node_idx].get_particle_idx()];
            self.nodes[node_idx].com = p.pos;
            self.nodes[node_idx].mass = p.mass;
        } else {
            let next = self.nodes[node_idx].get_child_idx();
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
    pub fn calculate_acc(&self, p: &mut Particle, node_idx: usize, p_idx: usize) {
        if self.nodes[node_idx].is_massless() {
            return;
        }

        let is_leaf = self.nodes[node_idx].is_leaf();
        let d2 = self.nodes[node_idx].com.d2(p.pos);
        let bh_condition = self.nodes[node_idx].s2 < THETA2 * d2;
        if bh_condition || is_leaf {
            // avoids particles accelerating themselves
            if is_leaf && self.nodes[node_idx].get_particle_idx() == p_idx {
                return;
            }

            let diff = self.nodes[node_idx].com - p.pos;
            let dir = diff / d2.sqrt();
            let mag = G * self.nodes[node_idx].mass / (d2 + EPSILON);
            p.acc += mag * dir;
        } else {
            let next = self.nodes[node_idx].get_child_idx();
            for child_idx in next..next + 4 {
                self.calculate_acc(p, child_idx, p_idx);
            }
        }
    }

    fn subdivide(&mut self, node_idx: usize) {
        // flags parent node as branch and supplies it the index of its first child
        let next = self.nodes.len();
        self.nodes[node_idx].set_child_idx(next);

        // subdivide the node and push its children onto nodes
        let child_boundaries = self.nodes[node_idx].boundary.subdivide();
        for b in child_boundaries {
            self.nodes.push(QuadNode::new(b));
        }
    }
}
