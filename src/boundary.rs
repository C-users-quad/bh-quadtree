use crate::vec_2::Vec2;

/// An axis-aligned bounding box (AABB) in 2D world space.
///
/// Used by [`crate::quadnode::QuadNode`] to define the region of space
/// each node covers. Boundaries are defined by a top-left corner,
/// a width, and a height.
#[derive(Copy, Clone)]
pub struct Boundary {
    /// X coordinate of the left edge.
    pub left: f32,
    /// Y coordinate of the top edge.
    pub top: f32,
    /// Width of the boundary.
    pub width: f32,
    /// Height of the boundary.
    pub height: f32,
}

impl Boundary {
    /// Constructs a new [`Boundary`] with the given position and dimensions.
    pub fn new(left: f32, top: f32, width: f32, height: f32) -> Self {
        Boundary {
            left,
            top,
            width,
            height,
        }
    }

    /// Returns `true` if `point` lies within this boundary.
    ///
    /// The left and top edges are inclusive, the right and bottom edges
    /// are exclusive — this prevents double-counting particles that lie
    /// exactly on a shared edge between two nodes.
    pub fn contains(&self, point: &Vec2) -> bool {
        point.x >= self.left
            && point.x < self.left + self.width
            && point.y >= self.top
            && point.y < self.top + self.height
    }

    /// Partitions this boundary into four equal quadrants.
    ///
    /// Returns boundaries in order: NW, NE, SW, SE.
    ///
    /// # Postconditions
    ///  - the four returned boundaries exactly partition this boundary
    ///  - each child boundary has half the width and half the height
    pub fn subdivide(&self) -> [Boundary; 4] {
        let w_div_2 = self.width / 2.0;
        let h_div_2 = self.height / 2.0;
        [
            Boundary::new(self.left, self.top, w_div_2, h_div_2),
            Boundary::new(self.left + w_div_2, self.top, w_div_2, h_div_2),
            Boundary::new(self.left, self.top + h_div_2, w_div_2, h_div_2),
            Boundary::new(self.left + w_div_2, self.top + h_div_2, w_div_2, h_div_2),
        ]
    }
}
