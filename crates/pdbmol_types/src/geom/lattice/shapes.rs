use std::f32::consts::{FRAC_1_SQRT_2, SQRT_2};

use crate::geom::math_utils::mul;

pub struct RhombicDodecahedron;
pub struct RhombicDodecahedronHex;
pub struct Cube;
pub struct TruncatedOctahedron;

const SQRT_3: f32 = 1.7320508;
const SQRT_6: f32 = 2.4494898;

pub trait LatticeShape {
    const UNIT_IMAGE_DISTANCE: [[f32; 3]; 3];

    fn vectors_with_image_distance(
        &self,
        d: f32,
    ) -> [[f32; 3]; 3] {
        let [a, b, c] = Self::UNIT_IMAGE_DISTANCE;
        [mul(d, a), mul(d, b), mul(d, c)]
    }
}

impl LatticeShape for RhombicDodecahedron {
    const UNIT_IMAGE_DISTANCE: [[f32; 3]; 3] = [
        [1., 0., 0.],
        [0., 1., 0.],
        [1. / 2., 1. / 2., 1. * FRAC_1_SQRT_2],
    ];
}

impl LatticeShape for RhombicDodecahedronHex {
    const UNIT_IMAGE_DISTANCE: [[f32; 3]; 3] = [
        [1., 0., 0.],
        [1. / 2., 1. * SQRT_3 / 2., 0.],
        [1. / 2., 1. * SQRT_3 / 6., 1. * SQRT_6 / 3.0],
    ];
}

impl LatticeShape for Cube {
    const UNIT_IMAGE_DISTANCE: [[f32; 3]; 3] = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];
}

impl LatticeShape for TruncatedOctahedron {
    const UNIT_IMAGE_DISTANCE: [[f32; 3]; 3] = [
        [1., 0., 0.],
        [1. / 3., 1. * SQRT_2 * 2. / 3., 0.],
        [1. / -3., 1. * SQRT_2 / 3., 1. * SQRT_6 / 3.],
    ];
}
