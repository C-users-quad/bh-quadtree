use crate::{physics::constants::MIN_NODE_SIZE, utils::boundary::Boundary, utils::vec2::Vec2};

/// A single node in the Barnes-Hut quadtree, tuned to fit in 32 bytes for
/// cache efficiency (2 nodes per 64-byte cache line).
///
/// A node is in one of three states:
/// - **Empty leaf**: a leaf with no particle (`idx == NO_PARTICLE`)
/// - **Occupied leaf**: a leaf holding one particle (`idx` is a particle index)
/// - **Branch**: an internal node with 4 children (`idx` encodes a child index)
///
/// The leaf/branch distinction is encoded in the high bit of `idx` via
/// [`BRANCH_FLAG`](QuadNode::BRANCH_FLAG) to avoid storing a separate field.
/// Always use [`set_particle_idx`](QuadNode::set_particle_idx) and
/// [`set_child_idx`](QuadNode::set_child_idx) to write `idx` — never write it
/// directly.
pub struct QuadNode {
    /// The axis-aligned bounding box that this node covers.
    pub boundary: Boundary,
    /// The total mass of all particles contained within this node's boundary.
    /// Zero if the node is an empty leaf.
    pub mass: f32,
    /// The center of mass of all particles contained within this node's boundary.
    /// Meaningless if `mass == 0.0`.
    pub com: Vec2,
    /// The squared side length of this node's boundary (`size²`), cached to
    /// avoid recomputing it during the Barnes-Hut opening angle check.
    pub s2: f32,
    /// Encodes either a particle index or a child index depending on node state.
    /// The high bit is used as a branch flag — do not read or write this directly.
    /// Use [`get_particle_idx`](QuadNode::get_particle_idx),
    /// [`set_particle_idx`](QuadNode::set_particle_idx),
    /// [`get_child_idx`](QuadNode::get_child_idx), and
    /// [`set_child_idx`](QuadNode::set_child_idx) instead.
    idx: u32,
}

impl QuadNode {
    /// Sentinel bit stored in the high bit of `idx` to mark this node as a branch.
    /// A branch has 4 children whose indices start at `get_child_idx()` in the
    /// master nodes vec.
    pub const BRANCH_FLAG: u32 = 1 << 31;

    /// Sentinel value for `idx` indicating an empty leaf (no particle assigned).
    /// Equal to `BRANCH_FLAG - 1`, which is the largest `u32` with the high bit
    /// clear, ensuring it is never mistaken for a branch.
    pub const NO_PARTICLE: u32 = Self::BRANCH_FLAG - 1;

    /// Creates a new empty leaf node covering the given boundary.
    /// `mass` is initialized to `0.0`, `com` to the origin, and `idx` to
    /// [`NO_PARTICLE`](QuadNode::NO_PARTICLE).
    pub fn new(boundary: Boundary) -> Self {
        Self {
            s2: boundary.size * boundary.size,
            mass: 0.0,
            com: Vec2::zero(),
            idx: Self::NO_PARTICLE,
            boundary,
        }
    }

    /// Returns `true` if this node cannot be subdivided further because its
    /// boundary has reached the minimum allowed size (`MIN_NODE_SIZE`).
    pub fn at_max_level(&self) -> bool {
        self.boundary.size < MIN_NODE_SIZE
    }

    /// Returns `true` if this node is a leaf (either empty or holding one particle).
    /// Use [`get_particle_idx`](QuadNode::get_particle_idx) to retrieve the particle
    /// index, and check against [`NO_PARTICLE`](QuadNode::NO_PARTICLE) to distinguish
    /// empty from occupied.
    pub fn is_leaf(&self) -> bool {
        self.idx & Self::BRANCH_FLAG == 0
    }

    /// Returns the particle index stored in this leaf node.
    ///
    /// # Panics (debug)
    /// Only call this if [`is_leaf`](QuadNode::is_leaf) returns `true`. If the node
    /// is an empty leaf, the returned value will be [`NO_PARTICLE`](QuadNode::NO_PARTICLE).
    pub fn get_particle_idx(&self) -> usize {
        self.idx as usize
    }

    /// Assigns a particle to this leaf node by storing its index in the master
    /// particles vec. Clears the branch flag, marking this node as a leaf.
    ///
    /// # Panics (debug)
    /// `idx` must be less than [`NO_PARTICLE`](QuadNode::NO_PARTICLE) to avoid
    /// colliding with the sentinel value or the branch flag.
    pub fn set_particle_idx(&mut self, idx: usize) {
        self.idx = idx as u32;
    }

    /// Returns the index of the first child of this branch node in the master
    /// nodes vec. The 4 children occupy indices `[i, i+1, i+2, i+3]` in
    /// `[NW, NE, SW, SE]` order.
    ///
    /// # Panics (debug)
    /// Only call this if [`if_leaf`](QuadNode::is_branch) returns `false`.
    pub fn get_child_idx(&self) -> usize {
        (self.idx & !Self::BRANCH_FLAG) as usize
    }

    /// Marks this node as a branch and stores the index of its first child in
    /// the master nodes vec. The 4 children are assumed to occupy indices
    /// `[idx, idx+1, idx+2, idx+3]` in `[NW, NE, SW, SE]` order.
    pub fn set_child_idx(&mut self, idx: usize) {
        self.idx = idx as u32 | Self::BRANCH_FLAG;
    }

    /// Determines whether this node contains a particle's index.
    ///
    /// # Panics (debug)
    /// Only call this if [`is_leaf`](QuadNode::is_leaf) returns `true`.
    pub fn is_empty(&self) -> bool {
        self.idx == Self::NO_PARTICLE
    }

    /// Returns the index of the child in the master nodes vec that spatially
    /// contains a particle at position `pos`.
    ///
    /// # Preconditions
    ///  - [`is_leaf`](QuadNode::is_branch) returns `false`.
    ///  - `pos` is spatially contained by this node.
    #[inline(always)]
    pub fn get_child_idx_of(&self, pos: Vec2) -> usize {
        let top = self.boundary.top;
        let left = self.boundary.left;
        let s_div_2 = self.boundary.size * 0.5;
        let mid_w = left + s_div_2;
        let mid_h = top + s_div_2;
        let next = self.get_child_idx();
        if pos.x >= mid_w {
            if pos.y >= mid_h { next + 3 } else { next + 1 }
        } else {
            if pos.y >= mid_h { next + 2 } else { next }
        }
    }
}
