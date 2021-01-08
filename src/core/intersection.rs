use nalgebra::*;

/// From the Möller-Trumbore algorithm.
/// See [here](https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm)
pub fn intersection(
    ray: &Vector3<f32>,
    origin: &Point3<f32>,
    triangle: &[Point3<f32>; 3],
) -> Option<f32> {
    const EPSILON: f32 = 0.000001;

    let edge_1 = triangle[1] - triangle[0];
    let edge_2 = triangle[2] - triangle[0];
    let h = ray.cross(&edge_2);
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
    let v = f * ray.dot(&q);
    if v < 0.0 || 1. < u + v {
        return None;
    }

    let t = f * edge_2.dot(&q);
    if t > EPSILON {
        // ray intersection
        let distance = (ray * t).norm();
        return Some(distance);
    } else {
        // This means that there is a line intersection but not a ray intersection.
        return None;
    }
}
