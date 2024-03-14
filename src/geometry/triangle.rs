use super::Point2D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle2D {
    pub point_a: Point2D,
    pub point_b: Point2D,
    pub point_c: Point2D,
}

impl Triangle2D {
    pub const fn new(point_a: Point2D, point_b: Point2D, point_c: Point2D) -> Self {
        Self {
            point_a,
            point_b,
            point_c,
        }
    }

    pub fn is_point_inside(&self, point: &Point2D) -> bool {
        let area = 0.5
            * (-self.point_b.y * self.point_c.x
                + self.point_a.y * (-self.point_b.x + self.point_c.x)
                + self.point_a.x * (self.point_b.y - self.point_c.y)
                + self.point_b.x * self.point_c.y);
        let s = 1.0 / (2.0 * area)
            * (self.point_a.y * self.point_c.x - self.point_a.x * self.point_c.y
                + (self.point_c.y - self.point_a.y) * point.x
                + (self.point_a.x - self.point_c.x) * point.y);
        let t = 1.0 / (2.0 * area)
            * (self.point_a.x * self.point_b.y - self.point_a.y * self.point_b.x
                + (self.point_a.y - self.point_b.y) * point.x
                + (self.point_b.x - self.point_a.x) * point.y);

        s >= -0.0001 && t >= -0.0001 && (s + t) <= 1.0001
    }
}
