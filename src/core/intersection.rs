use nalgebra::*;

/// A direction and an origin for ray-traced events
pub struct Ray(Vector3<f32>, Point3<f32>);

impl Ray {
    /// Creates a new ray from a direction and an origin
    pub fn new(dir: Vector3<f32>, origin: Point3<f32>) -> Self {
        Self(dir, origin)
    }

    /// Returns the direction of the ray
    pub fn direction(&self) -> Vector3<f32> {
        self.0
    }

    /// Returns the origin of the ray
    pub fn origin(&self) -> Point3<f32> {
        self.1
    }
}

/// Checks if a ray intersects a triangle,
/// using the [Möller-Trumbore algorithm](https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm)
pub fn intersection(ray: &Ray, triangle: &[Point3<f32>; 3]) -> Option<f32> {
    #![allow(clippy::many_single_char_names)]
    let Ray(direction, origin) = ray;
    const EPSILON: f32 = 0.000001;

    let edge_1 = triangle[1] - triangle[0];
    let edge_2 = triangle[2] - triangle[0];
    let h = direction.cross(&edge_2);
    let a = edge_1.dot(&h);

    // The ray is parallel to the triangle
    if -EPSILON < a && a < EPSILON {
        return None;
    }

    let f = 1.0 / a;
    let s = origin - triangle[0];
    let u = f * s.dot(&h);

    if u < 0. || 1. < u {
        return None;
    }

    let q = s.cross(&edge_1);
    let v = f * direction.dot(&q);
    if v < 0.0 || 1. < u + v {
        return None;
    }

    let t = f * edge_2.dot(&q);
    if t > EPSILON {
        // ray intersection
        let distance = (direction * t).norm();
        Some(distance)
    } else {
        // This means that there is a line intersection but not a ray intersection.
        None
    }
}
