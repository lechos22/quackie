#[derive(Debug, Clone, Copy, PartialEq)]
struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Triangle2D {
    point_a: Point2D,
    point_b: Point2D,
    point_c: Point2D,
}

impl Triangle2D {
    fn new(point_a: Point2D, point_b: Point2D, point_c: Point2D) -> Self {
        Self {
            point_a,
            point_b,
            point_c,
        }
    }

    fn is_point_inside(&self, point: &Point2D) -> bool {
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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_inside_triangle() {
        let triangle = Triangle2D::new(
            Point2D::new(0.0, 0.0),
            Point2D::new(4.0, 0.0),
            Point2D::new(2.0, 4.0),
        );

        // Test points inside the triangle
        assert!(triangle.is_point_inside(&Point2D::new(2.0, 1.0)));
        assert!(triangle.is_point_inside(&Point2D::new(1.0, 1.0)));
        assert!(triangle.is_point_inside(&Point2D::new(2.0, 2.0)));
        assert!(triangle.is_point_inside(&Point2D::new(3.0, 1.0)));
    }

    #[test]
    fn test_point_outside_triangle() {
        let triangle = Triangle2D::new(
            Point2D::new(0.0, 0.0),
            Point2D::new(4.0, 0.0),
            Point2D::new(2.0, 4.0),
        );

        // Test points outside the triangle
        assert!(!triangle.is_point_inside(&Point2D::new(0.0, 5.0)));
        assert!(!triangle.is_point_inside(&Point2D::new(6.0, 1.0)));
        assert!(!triangle.is_point_inside(&Point2D::new(-1.0, 0.0)));
        assert!(!triangle.is_point_inside(&Point2D::new(2.0, -1.0)));
    }

    #[test]
    fn test_point_on_triangle_edges() {
        let triangle = Triangle2D::new(
            Point2D::new(0.0, 0.0),
            Point2D::new(4.0, 0.0),
            Point2D::new(2.0, 4.0),
        );

        // Test points on triangle edges
        assert!(triangle.is_point_inside(&Point2D::new(0.0, 0.0)));
        assert!(triangle.is_point_inside(&Point2D::new(4.0, 0.0)));
        assert!(triangle.is_point_inside(&Point2D::new(2.0, 4.0)));
        assert!(triangle.is_point_inside(&Point2D::new(1.0, 2.0)));
        assert!(triangle.is_point_inside(&Point2D::new(3.0, 2.0)));
    }
}
