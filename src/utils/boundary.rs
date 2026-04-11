/// square bounding box with a topleft position and a side length size
#[derive(Copy, Clone)]
pub struct Boundary {
    pub left: f32,
    pub top: f32,
    pub size: f32,
}

impl Boundary {
    pub fn new(left: f32, top: f32, size: f32) -> Self {
        Self { left, top, size }
    }

    /// [NW, NE, SW, SE]
    pub fn subdivide(&self) -> [Boundary; 4] {
        let size = self.size / 2.0;
        let l_p_size = self.left + size;
        let t_p_size = self.top + size;
        [
            Boundary::new(self.left, self.top, size),
            Boundary::new(l_p_size, self.top, size),
            Boundary::new(self.left, t_p_size, size),
            Boundary::new(l_p_size, t_p_size, size),
        ]
    }
}
