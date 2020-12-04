/// Conversion from a `nalgebra` matrix to a four by four float array.
/// The resulting matrix is transposed so it can be imported directly
/// in openGL.
pub fn to_array(mat: &nalgebra::Matrix4<f32>) -> [[f32; 4]; 4] {
    [
        [mat[(0, 0)], mat[(1, 0)], mat[(2, 0)], mat[(3, 0)]],
        [mat[(0, 1)], mat[(1, 1)], mat[(2, 1)], mat[(3, 1)]],
        [mat[(0, 2)], mat[(1, 2)], mat[(2, 2)], mat[(3, 2)]],
        [mat[(0, 3)], mat[(1, 3)], mat[(2, 3)], mat[(3, 3)]],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_array() {
        let matrix = nalgebra::Matrix4::new(
            1., 0., 0., 10., 0., 1., 0., 20., 0., 0., 1., 30., 0., 0., 0., 1.,
        );
        let array = [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [10., 20., 30., 1.],
        ];
        assert_eq!(to_array(&matrix), array);
    }
}
