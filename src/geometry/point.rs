#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn rotate_around(&self, relative_to: Self, rotate_by: f64) -> Self {
        let sine = rotate_by.sin();
        let cosine = rotate_by.cos();
        let transposed = Self::new(self.x - relative_to.x, self.y - relative_to.y);
        let rotated = Self::new(
            cosine * transposed.x - sine * transposed.y,
            sine * transposed.x + cosine * transposed.y,
        );
        let retransposed = Self::new(rotated.x + relative_to.x, rotated.y + relative_to.y);
        retransposed
    }
}
