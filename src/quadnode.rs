use crate::{
    boundary::Boundary,
    constants::{CAPACITY, MAX_LEVEL, THETA_2},
    particle::{Particle, PseudoParticle},
    vec_2::Vec2,
};

/// A node in a Barnes-Hut quadtree.
///
/// Each node represents a rectangular region of 2D space and either stores
/// particle indices directly (leaf node) or delegates to four child nodes
/// that partition its space (internal node). Centers of mass are computed
/// recursively and used to approximate gravitational forces via the
/// Barnes-Hut algorithm.
pub struct QuadNode {
    /// The rectangular boundary of this node in world space.
    boundary: Boundary,
    /// Indices into the master particle array for particles in this node.
    /// Only populated for leaf nodes — drained on subdivision.
    particle_indices: Vec<usize>,
    /// The four child nodes partitioning this node's space (NW, NE, SW, SE).
    /// `None` if this is a leaf node.
    children: Option<Box<[QuadNode; 4]>>,
    /// The center of mass of all particles within this node's boundary.
    /// Computed by [`QuadNode::calculate_com`].
    center_of_mass: Vec2,
    /// The total mass of all particles within this node's boundary.
    /// Computed by [`QuadNode::calculate_com`].
    total_mass: f32,
    /// The depth of this node in the tree. Root is level 0.
    level: usize,
    /// Precomputed `max(width, height)²` used in the Barnes-Hut criterion
    /// to avoid recomputing it on every query.
    s2: f32,
    /// Total number of particles within this subtree.
    particle_count: usize,
}

impl QuadNode {
    pub fn new(boundary: Boundary, level: usize) -> Self {
        QuadNode {
            boundary,
            particle_indices: Vec::new(),
            children: None,
            center_of_mass: Vec2::default(),
            total_mass: 0.0,
            level,
            s2: boundary.width.max(boundary.height).powi(2),
            particle_count: 0,
        }
    }

    /// Creates 4 child nodes of uniform dimensions that partition the
    /// space of this node.
    ///
    /// # Preconditions
    /// - `self.children` should be `None` — calling on an already subdivided node
    ///   overwrites existing children and loses all their particles.
    ///
    /// # Postconditon
    ///  - `self.children` is populated with the 4 child [`QuadNode`]s.
    ///
    /// # Example
    /// ```
    /// node.subdivide();
    /// assert!(node.children.is_some())
    /// ```
    pub fn subdivide(&mut self) {
        let [nw, ne, sw, se] = self.boundary.subdivide();
        let next_lvl = self.level + 1;
        self.children = Some(Box::new([
            QuadNode::new(nw, next_lvl),
            QuadNode::new(ne, next_lvl),
            QuadNode::new(sw, next_lvl),
            QuadNode::new(se, next_lvl),
        ]))
    }

    /// Attempts to insert particle at `particles[particle_index]` into the node.
    ///
    /// ### A particle is suitable for insertion into a given node if:
    ///  - it is located within the nodes boundaries
    ///  - the node is a leaf node (that is, it does not have children)
    ///  - the node isnt full
    ///
    /// ### Insertion may fail due to overflow, which occurs when:
    ///  - [`MAX_LEVEL`] has been reached
    ///  - the leaf node at max level that bounds the particle is full
    ///  - the node does not contain the particle
    ///
    /// ### If the node is full and it isnt a child:
    ///  - the node subdivides into 4 child nodes one level up from the current level
    ///  - all particles currently in the node plus the particle in question
    ///    are inserted into the new child nodes.
    ///  - `self.particle_indices` is drained.
    ///
    /// # Preconditions
    ///  - `particle_index` must be a valid index in `particles`
    ///
    /// # Panics
    ///  - `particle_index` is out of bounds
    ///
    /// # Returns
    ///  - `true` if the particle was inserted
    ///  - `false` otherwise
    ///
    /// # Example
    /// ```
    /// let particles: [Particle] = [Particle::new()];
    /// let inserted: bool = node.insert(0, &particles);
    /// assert!(inserted);
    /// ```
    pub fn insert(&mut self, particle_index: usize, particles: &Vec<Particle>) -> bool {
        // alias current particle we are inserting as p
        let p = &particles[particle_index];

        if !self.boundary.contains(&p.position) {
            // if the particle isnt contained by the node, exit.
            false
        } else if self.is_leaf() && !self.is_full() {
            // if the node is a leaf and isnt full, push the particle.
            self.particle_indices.push(particle_index);
            self.particle_count += 1;
            true
        } else if self.is_leaf() && self.is_full() && !self.is_at_bottom() {
            // if the node is a leaf, is full, and isnt at max level,
            // subdivide and redistribute.
            self.particle_indices.push(particle_index);
            self.subdivide();
            self.redistribute(particles);
            true
        } else if !self.is_leaf() {
            if let Some(children) = &mut self.children {
                for child in children.iter_mut() {
                    if child.insert(particle_index, particles) {
                        return true;
                    };
                }
            }
            false
        } else {
            // leaf is full, its at the max level - overflow: drop the particle.
            false
        }
    }

    /// Calculates the total mass of this node and the position of
    /// the center of mass. Mutates the node in place and updates it
    /// with these values.
    ///
    /// ### If the node is empty:
    ///  - center of mass will not be calculated
    ///
    /// ### If the node isnt a leaf node:
    ///  - this nodes children are recursively operated on in order to calculate
    ///    their centers of mass which are used to calculate this nodes center of mass.
    ///
    /// # Preconditions
    ///  - all nodes are populated with particles — call [`QuadNode::insert`]
    ///
    /// # Postconditions
    ///  - `self.total_mass` and `self.center_of_mass` have been modified to reflect
    ///    this functions results.
    ///
    /// # Example
    /// ```
    /// let particles = [Particle::new()];
    /// let boundary = Boundary::new(-100.0, -100.0, 200.0, 200.0);
    /// let mut node = QuadNode::new(boundary, 0);
    /// node.insert(0, &particles);
    /// node.calculate_com(&particles);
    /// assert_eq!(node.center_of_mass.x, 1.0);
    /// assert_eq!(node.center_of_mass.y, 2.0);
    /// assert_eq!(node.total_mass, 10.0);
    /// ```
    pub fn calculate_com(&mut self, particles: &Vec<Particle>) {
        // calculates the total mass of the leaf node
        if self.is_leaf() {
            // if theres no particles in the leaf node, just exit; mass remains default.
            if self.is_empty() {
                return;
            }

            // calculate total mass
            self.total_mass = self
                .particle_indices
                .iter()
                .map(|p_i| particles[*p_i].mass)
                .sum();

            // calculate center of mass
            let (cx, cy): (f32, f32) =
                self.particle_indices
                    .iter()
                    .fold((0.0, 0.0), |(cx, cy), p_i| {
                        let p = &particles[*p_i];
                        (cx + p.position.x * p.mass, cy + p.position.y * p.mass)
                    });

            self.center_of_mass.x = cx / self.total_mass;
            self.center_of_mass.y = cy / self.total_mass;
        }
        // calculates the total mass of an internal node
        // by summing the total masses of its children
        else if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                child.calculate_com(particles);
                self.total_mass += child.total_mass;
                self.center_of_mass.x += child.total_mass * child.center_of_mass.x;
                self.center_of_mass.y += child.total_mass * child.center_of_mass.y;
            }
            self.center_of_mass.x /= self.total_mass;
            self.center_of_mass.y /= self.total_mass;
        }
    }

    /// Populates `result` with pseudo particles that can be used to approximate
    /// the gravitational forces for a particle at position `position` according
    /// to the Barnes-Hut algorithm.
    ///
    /// ### The Barnes-Hut condition (written as `s/d < theta`) inqures that:
    ///  - the width, `s`, of the node,
    ///  - divided by the distance, `d`, of the nodes center of mass to the querying
    ///    particles position
    ///  - is less than a value `theta`.
    ///
    /// ### If the node is empty:
    ///  - exit immediately (to prevent doing unneccessary checks).
    ///
    /// ### If the node satisfies the Barnes-Hut condition:
    ///  - append the [`PseudoParticle`] representing this node center of mass
    ///    to result.
    ///
    /// Note that i did some algebraic manipulations for the sake of performace
    /// in order to avoid the division in the right hand side of the condition
    /// and the square root in the distance equation, which yielded me
    /// `s^2 < theta^2 * d^2`.
    ///
    /// # Preconditions
    ///  - all nodes are populated with particles — call [`QuadNode::insert`]
    ///  - all nodes have had their centers of mass calculated
    ///    — call [`QuadNode::calculate_com`]
    ///  - position is located within the root nodes boundaries
    ///  - position relates to the position attribute of a [`Particle`] in
    ///    the central array of particles
    ///
    /// # Postconditions
    ///  - `result` is populated with all the `PseudoParticles` yielded by `query`
    pub fn query(&self, position: &Vec2, result: &mut Vec<PseudoParticle>) {
        if self.is_empty() {
            return;
        }

        if self.s2 < THETA_2 * self.center_of_mass.distance_squared(position) || self.is_leaf() {
            result.push(self.get_pseudo_particle())
        } else if let Some(children) = &self.children {
            for child in children.iter() {
                child.query(position, result);
            }
        }
    }

    /// Redistributes all particles in [`QuadNode::particle_indices`]
    /// into the node's children.
    ///
    /// Drains `particle_indices` and attempts to insert each index into the correct
    /// child node based on boundary containment. Each particle is inserted into the
    /// first child whose boundary contains it.
    ///
    /// # Panics
    /// May panic if `particles` is shorter than the indices stored in
    /// `particle_indices`.
    ///
    /// # Preconditions
    /// - `self.children` must be `Some` — call [`QuadNode::subdivide`]
    ///   before calling this.
    /// - `self.particle_indices` should be non-empty — calling on an
    ///   empty node is a no-op.
    /// - Every index in `self.particle_indices` must be
    ///   a valid index into `particles`.
    ///
    /// # Example
    /// ```
    /// node.subdivide();
    /// node.particle_indices.push(0);
    /// node.redistribute(&particles);
    /// assert!(node.particle_indices.is_empty());
    /// ```
    fn redistribute(&mut self, particles: &Vec<Particle>) {
        let indices: Vec<usize> = self.particle_indices.drain(..).collect();
        if let Some(children) = &mut self.children {
            indices.into_iter().for_each(|idx| {
                children.iter_mut().any(|c| c.insert(idx, particles));
            });
        }
    }

    /// Gets a pseudo particle that represents the nodes by inheriting the
    /// properties of its center of mass.
    ///
    /// # Precondition
    ///  - this nodes center of mass has been calculated
    ///    — call [`QuadNode::calculate_com`]
    ///
    /// # Returns
    ///  - `PseudoParticle` with position `self.center_of_mass`
    ///    and mass `self.total_mass`
    fn get_pseudo_particle(&self) -> PseudoParticle {
        PseudoParticle {
            position: self.center_of_mass,
            mass: self.total_mass,
        }
    }

    /// Determines if this node has no particles in it.
    ///
    /// # Returns
    ///  - true if `self.particle_indices.is_empty()`
    ///  - false otherwise
    fn is_empty(&self) -> bool {
        self.particle_count == 0
    }

    /// Determines if this node is at capacity.
    ///
    /// # Returns
    ///  - true if `self.particle_indices.len()` >= [`CAPACITY`]
    ///  - false otherwise
    fn is_full(&self) -> bool {
        self.particle_indices.len() >= CAPACITY
    }

    /// Determines if this node is at the max level.
    ///
    /// # Returns
    ///  - true if `self.level` >= [`MAX_LEVEL`]
    ///  - false otherwise
    ///
    fn is_at_bottom(&self) -> bool {
        self.level >= MAX_LEVEL
    }

    /// Determines if this node is a leaf, as in it has no children.
    ///
    /// # Returns
    ///  - true if `self.children.is_none()`
    ///  - false if not
    fn is_leaf(&self) -> bool {
        self.children.is_none()
    }
}
