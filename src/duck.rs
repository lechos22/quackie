use quackie::geometry::{triangle::Triangle2D, vector::Vector2D};

pub const BASE_TRIANGLES: &[Triangle2D] = &[
    Triangle2D::new([
        Vector2D::new(0.3, 0.45),
        Vector2D::new(0.3, 0.75),
        Vector2D::new(0.13, 0.5),
    ]),
    Triangle2D::new([
        Vector2D::new(0.3, 0.45),
        Vector2D::new(0.3, 0.75),
        Vector2D::new(0.7, 0.75),
    ]),
    Triangle2D::new([
        Vector2D::new(0.3, 0.45),
        Vector2D::new(0.7, 0.45),
        Vector2D::new(0.7, 0.75),
    ]),
    Triangle2D::new([
        Vector2D::new(0.7, 0.45),
        Vector2D::new(0.7, 0.75),
        Vector2D::new(0.8, 0.55),
    ]),
    // Head
    Triangle2D::new([
        Vector2D::new(0.4, 0.45),
        Vector2D::new(0.6, 0.45),
        Vector2D::new(0.35, 0.3),
    ]),
    Triangle2D::new([
        Vector2D::new(0.45, 0.2),
        Vector2D::new(0.6, 0.45),
        Vector2D::new(0.35, 0.3),
    ]),
    Triangle2D::new([
        Vector2D::new(0.45, 0.2),
        Vector2D::new(0.6, 0.45),
        Vector2D::new(0.6, 0.25),
    ]),
    Triangle2D::new([
        Vector2D::new(0.45, 0.2),
        Vector2D::new(0.55, 0.2),
        Vector2D::new(0.6, 0.25),
    ]),
];

pub const EYE_TRIANGLE: Triangle2D = Triangle2D::new([
    Vector2D::new(0.525, 0.3),
    Vector2D::new(0.5, 0.25),
    Vector2D::new(0.475, 0.3),
]);

pub const BEAK_TRIANGLE: Triangle2D = Triangle2D::new([
    Vector2D::new(0.75, 0.365),
    Vector2D::new(0.6, 0.45),
    Vector2D::new(0.6, 0.3),
]);
