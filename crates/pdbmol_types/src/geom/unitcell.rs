//! # Further reading
//!
//! - <https://docs.lammps.org/Howto_triclinic.html#crystallographic-general-triclinic-representation-of-a-simulation-box>
//! - <https://doi.org/10.1002/(SICI)1096-987X(19971130)18:15%3C1930::AID-JCC8%3E3.0.CO;2-P>
//! - <http://docs.openmm.org/latest/userguide/theory/05_other_features.html#periodic-boundary-conditions>
//! - <https://manual.gromacs.org/2024.2/reference-manual/algorithms/periodic-boundary-conditions.html>

use std::f32::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

mod conversions;
mod representations;
mod tilings;

pub use representations::*;

use crate::geom::math_utils::{add, mul};

pub enum UnitCellShape {
    RhombicDodecahedron,
    RhombicDodecahedronHex,
    Cube,
    TruncatedOctahedron,
}

impl UnitCellShape {
    fn vectors_with_image_distance(
        &self,
        d: f32,
    ) -> TriclinicUnitCell {
        TriclinicUnitCell(match self {
            Self::RhombicDodecahedron => [
                [d, 0., 0.],
                [0., d, 0.],
                [d / 2., d / 2., d * FRAC_1_SQRT_2],
            ],
            Self::RhombicDodecahedronHex => [
                [d, 0., 0.],
                [d / 2., d * f32::sqrt(3.) / 2., 0.],
                [d / 2., d * f32::sqrt(3.) / 6., d * f32::sqrt(6.) / 3.0],
            ],
            Self::Cube => [[d, 0., 0.], [0., d, 0.], [0., 0., d]],
            Self::TruncatedOctahedron => [
                [d, 0., 0.],
                [d / 3., d * SQRT_2 * 2. / 3., 0.],
                [d / -3., d * f32::sqrt(2.) / 3., d * f32::sqrt(6.) / 3.],
            ],
        })
    }
}

pub trait UnitCell: Into<TriclinicUnitCell> + TryFrom<TriclinicUnitCell> + Clone {
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
        Self::try_from(TriclinicUnitCell(vectors))
    }

    fn from_lengths(
        x: f32,
        y: f32,
        z: f32,
    ) -> Result<Self, Self::Error> {
        Self::try_from(OrthogonalUnitCell { x, y, z }.into())
    }

    fn from_length(l: f32) -> Result<Self, Self::Error> {
        Self::try_from(CubicUnitCell(l).into())
    }

    fn from_image_distance_and_shape(
        d: f32,
        shape: UnitCellShape,
    ) -> Result<Self, Self::Error> {
        shape
            .vectors_with_image_distance(d)
            .try_into()
    }

    fn to_general_triclinic(&self) -> TriclinicUnitCell {
        self.clone().into()
    }

    fn to_vectors(&self) -> [[f32; 3]; 3] {
        self.to_general_triclinic().0
    }

    fn to_md_triclinic(&self) -> RestrictedTriclinicUnitCell {
        self.to_general_triclinic()
            .restrict_orientation()
            .reduce_tilt()
    }

    fn to_md_vectors(&self) -> [[f32; 3]; 3] {
        self.to_md_triclinic().to_vectors()
    }

    fn to_crystallographic(&self) -> CrystallographicUnitCell {
        self.to_md_triclinic().into()
    }

    fn to_brick(&self) -> OrthogonalUnitCell {
        self.to_md_triclinic().orthogonalize()
    }

    fn has_orientation(&self) -> bool {
        RestrictedTriclinicUnitCell::try_from(self.to_general_triclinic()).is_err()
    }

    fn has_tilt(&self) -> bool {
        OrthogonalUnitCell::try_from(
            self.to_general_triclinic()
                .restrict_orientation(),
        )
        .is_err()
    }

    fn scale(
        &self,
        scale: f32,
    ) -> Self {
        let TriclinicUnitCell([a, b, c]) = self.clone().into();
        let scaled = Self::try_from(TriclinicUnitCell([
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

impl<T> UnitCell for T where T: Into<TriclinicUnitCell> + TryFrom<TriclinicUnitCell> + Clone {}
