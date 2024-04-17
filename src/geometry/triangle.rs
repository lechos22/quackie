use super::vector::Vector2D;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
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

    pub fn to_uv_point(&self, uv_triangle: &Self, point: &Vector2D) -> Vector2D {
        let area = self.calculate_signed_area();
        let s = self.calculate_s(point, area);
        let t = self.calculate_t(point, area);
        uv_triangle.points[0] * s
            + uv_triangle.points[1] * t
            + uv_triangle.points[2] * (1.0 - s - t)
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

#[cfg(test)]
mod tests {
    use crate::geometry::{triangle::Triangle2D, vector::Vector2D};

    #[test]
    fn point_inside_triangle() {
        let triangle = Triangle2D::new([
            Vector2D::new(0.0, 0.0),
            Vector2D::new(4.0, 0.0),
            Vector2D::new(2.0, 4.0),
        ]);

        // Test points inside the triangle
        assert!(triangle.is_point_inside(&Vector2D::new(2.0, 1.0)));
        assert!(triangle.is_point_inside(&Vector2D::new(1.0, 1.0)));
        assert!(triangle.is_point_inside(&Vector2D::new(2.0, 2.0)));
        assert!(triangle.is_point_inside(&Vector2D::new(3.0, 1.0)));
    }

    #[test]
    fn point_outside_triangle() {
        let triangle = Triangle2D::new([
            Vector2D::new(0.0, 0.0),
            Vector2D::new(4.0, 0.0),
            Vector2D::new(2.0, 4.0),
        ]);

        // Test points outside the triangle
        assert!(!triangle.is_point_inside(&Vector2D::new(0.0, 5.0)));
        assert!(!triangle.is_point_inside(&Vector2D::new(6.0, 1.0)));
        assert!(!triangle.is_point_inside(&Vector2D::new(-1.0, 0.0)));
        assert!(!triangle.is_point_inside(&Vector2D::new(2.0, -1.0)));
    }

    #[test]
    fn point_on_triangle_edges() {
        let triangle = Triangle2D::new([
            Vector2D::new(0.0, 0.0),
            Vector2D::new(4.0, 0.0),
            Vector2D::new(2.0, 4.0),
        ]);

        // Test points on triangle edges
        assert!(triangle.is_point_inside(&Vector2D::new(0.0, 0.0)));
        assert!(triangle.is_point_inside(&Vector2D::new(4.0, 0.0)));
        assert!(triangle.is_point_inside(&Vector2D::new(2.0, 4.0)));
        assert!(triangle.is_point_inside(&Vector2D::new(1.0, 2.0)));
        assert!(triangle.is_point_inside(&Vector2D::new(3.0, 2.0)));
    }
}
