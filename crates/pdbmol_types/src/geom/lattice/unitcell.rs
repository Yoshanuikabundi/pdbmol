use super::representations::*;
use super::*;
use crate::geom::math_utils::{add, mul};
use std::f32::consts::*;

pub trait UnitCell:
    Into<OrientedTriclinicUnitCell> + TryFrom<OrientedTriclinicUnitCell> + Clone
{
    fn from_lengths_and_angles_rad(
        a: f32,
        b: f32,
        c: f32,
        alpha: f32,
        beta: f32,
        gamma: f32,
    ) -> Result<Self, Self::Error> {
        Self::try_from(CrystallographicUnitCell { a, b, c, alpha, beta, gamma }.into())
    }

    fn from_lengths_and_angles_deg(
        a: f32,
        b: f32,
        c: f32,
        alpha: f32,
        beta: f32,
        gamma: f32,
    ) -> Result<Self, Self::Error> {
        Self::from_lengths_and_angles_rad(
            a * 180.0 / PI,
            b * 180.0 / PI,
            c * 180.0 / PI,
            alpha,
            beta,
            gamma,
        )
    }

    fn from_vectors(vectors: [[f32; 3]; 3]) -> Result<Self, Self::Error> {
        Self::try_from(OrientedTriclinicUnitCell(vectors))
    }

    fn from_lengths(
        x: f32,
        y: f32,
        z: f32,
    ) -> Result<Self, Self::Error> {
        Self::try_from(OrthorhombicUnitCell { x, y, z }.into())
    }

    fn from_length(l: f32) -> Result<Self, Self::Error> {
        Self::try_from(CubicUnitCell(l).into())
    }

    fn from_image_distance_and_shape(
        d: f32,
        shape: impl LatticeShape,
    ) -> Result<Self, Self::Error> {
        shape
            .vectors_with_image_distance(d)
            .to_general_triclinic()
            .try_into()
    }

    fn to_general_triclinic(&self) -> OrientedTriclinicUnitCell {
        self.clone().into()
    }

    fn to_vectors(&self) -> [[f32; 3]; 3] {
        self.to_general_triclinic().0
    }

    fn lattice(&self) -> Lattice {
        Lattice::from(self.to_general_triclinic())
    }

    fn to_md_vectors(&self) -> [[f32; 3]; 3] {
        self.lattice().to_vectors()
    }

    fn to_crystallographic(&self) -> CrystallographicUnitCell {
        self.lattice().into()
    }

    fn has_orientation(&self) -> bool {
        TriclinicUnitCell::try_from(self.to_general_triclinic()).is_err()
    }

    fn has_tilt(&self) -> bool {
        OrthorhombicUnitCell::try_from(
            self.to_general_triclinic()
                .restrict_orientation(),
        )
        .is_err()
    }

    fn scale(
        &self,
        scale: f32,
    ) -> Self;

    fn tile_points(
        &self,
        points: &[[f32; 3]],
        iterations: [usize; 3],
    ) -> Vec<[f32; 3]>;
}

impl<T> UnitCell for T
where
    T: Into<OrientedTriclinicUnitCell> + TryFrom<OrientedTriclinicUnitCell> + Clone,
{
    fn scale(
        &self,
        scale: f32,
    ) -> Self {
        let OrientedTriclinicUnitCell([a, b, c]) = self.clone().into();
        let scaled = Self::try_from(OrientedTriclinicUnitCell([
            mul(scale, a),
            mul(scale, b),
            mul(scale, c),
        ]));
        if let Ok(scaled) = scaled {
            scaled
        } else {
            panic!("converting triclinic representation back to original representation")
        }
    }

    fn tile_points(
        &self,
        points: &[[f32; 3]],
        iterations: [usize; 3],
    ) -> Vec<[f32; 3]> {
        let cell_vectors = self.to_vectors();
        let mut tiled =
            Vec::with_capacity(points.len() * iterations[0] * iterations[1] * iterations[2]);
        tiled.extend_from_slice(points);

        // Tile along each dimension one at a time
        for dim in 0..3 {
            for i in 0..iterations[dim] {
                tiled.extend(
                    points
                        .iter()
                        .map(|&point| add(point, mul(i as f32, cell_vectors[dim]))),
                );
            }
        }

        tiled
    }
}
