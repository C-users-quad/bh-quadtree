use crate::{
    boundary::Boundary,
    particle::{Particle, PseudoParticle},
    quadnode::QuadNode,
    vec_2::Vec2,
};

/// A Barnes-Hut quadtree for approximating gravitational forces in an N-body simulation.
///
/// The tree is rebuilt every simulation step from the current particle positions,
/// computing centers of mass for all nodes so that [`QuadTree::query`] can
/// approximate forces using the Barnes-Hut algorithm.
pub struct QuadTree {
    /// The root node of the tree, whose boundary contains all particles.
    root: QuadNode,
}

impl QuadTree {
    /// Constructs a new `QuadTree` with a root node covering `boundary`.
    ///
    /// Prefer [`QuadTree::build`] to construct a tree from particles directly.
    pub fn new(boundary: Boundary) -> Self {
        QuadTree {
            root: QuadNode::new(boundary, 0),
        }
    }

    /// Rebuilds the tree from scratch using the current particle positions.
    ///
    /// Computes a tight bounding box around all particles, resets the root node,
    /// inserts all particles, and computes centers of mass for all nodes.
    ///
    /// Should be called once per simulation step before any calls to [`QuadTree::query`].
    ///
    /// # Preconditions
    ///  - `particles` is non-empty
    ///
    /// # Postconditions
    ///  - the tree is fully built and ready to be queried
    ///  - all nodes have valid centers of mass
    pub fn build(&mut self, particles: &Vec<Particle>) {
        let (min_x, max_x, min_y, max_y): (f32, f32, f32, f32) = particles
            .iter()
            .fold((f32::INFINITY, f32::NEG_INFINITY, f32::INFINITY, f32::NEG_INFINITY),
            |(min_x, max_x, min_y, max_y), p| (
                min_x.min(p.position.x),
                max_x.max(p.position.x),
                min_y.min(p.position.y),
                max_y.max(p.position.y)
            ));
        let boundary = Boundary::new(min_x, min_y, max_x - min_x, max_y - min_y);
        self.reset_root(boundary);
        self.insert(particles);
        self.root.calculate_com(particles);
    }

    /// Populates `buffer` with pseudo particles approximating the gravitational
    /// influence on a particle at `position`.
    ///
    /// Delegates to [`QuadNode::query`] on the root node.
    ///
    /// # Preconditions
    ///  - [`QuadTree::build`] has been called this frame
    ///  - `buffer` has been cleared before calling this
    ///
    /// # Postconditions
    ///  - `buffer` is populated with pseudo particles for force calculation
    pub fn query(&self, position: &Vec2, buffer: &mut Vec<PseudoParticle>) {
        self.root.query(position, buffer);
    }

    /// Inserts all particles into the tree by index.
    ///
    /// # Preconditions
    ///  - [`QuadTree::reset_root`] has been called with a boundary that contains
    ///    all particles
    fn insert(&mut self, particles: &Vec<Particle>) {
        (0..particles.len()).for_each(|i| {
            self.root.insert(i, particles);
        });
    }

    /// Resets the root node with a new boundary, discarding all existing nodes.
    fn reset_root(&mut self, boundary: Boundary) {
        self.root = QuadNode::new(boundary, 0)
    }
}
