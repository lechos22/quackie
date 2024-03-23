use super::Vector2D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle2D {
    pub points: [Vector2D; 3],
}

impl Triangle2D {
    pub const fn new(points: [Vector2D; 3]) -> Self {
        Self { points }
    }

    pub fn is_point_inside(&self, point: &Vector2D) -> bool {
        let area = self.calculate_signed_area();
        let s = self.calculate_s(point, area);
        let t = self.calculate_t(point, area);

        self.is_barycentric_point_inside(s, t)
    }

    fn calculate_s(&self, point: &Vector2D, area: f64) -> f64 {
        1.0 / (2.0 * area)
            * (self.points[0].y * self.points[2].x - self.points[0].x * self.points[2].y
                + (self.points[2].y - self.points[0].y) * point.x
                + (self.points[0].x - self.points[2].x) * point.y)
    }

    fn calculate_t(&self, point: &Vector2D, area: f64) -> f64 {
        1.0 / (2.0 * area)
            * (self.points[0].x * self.points[1].y - self.points[0].y * self.points[1].x
                + (self.points[0].y - self.points[1].y) * point.x
                + (self.points[1].x - self.points[0].x) * point.y)
    }

    fn is_barycentric_point_inside(&self, s: f64, t: f64) -> bool {
        s >= -0.0001 && t >= -0.0001 && (s + t) <= 1.0001
    }

    fn calculate_signed_area(&self) -> f64 {
        0.5 * (-self.points[1].y * self.points[2].x
            + self.points[0].y * (-self.points[1].x + self.points[2].x)
            + self.points[0].x * (self.points[1].y - self.points[2].y)
            + self.points[1].x * self.points[2].y)
    }
}

