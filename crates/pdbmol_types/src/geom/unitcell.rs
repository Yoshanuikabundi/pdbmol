use std::f32::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

mod conversions;
mod representations;

pub use representations::*;

pub enum UnitCellShape {
    RhombicDodecahedron,
    RhombicDodecahedronHex,
    Cube,
    TruncatedOctahedron,
}

impl UnitCellShape {
    fn vectors_with_image_distance(&self, d: f32) -> TriclinicUnitCell {
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
        Self::try_from(
            CrystallographicUnitCell {
                a,
                b,
                c,
                alpha,
                beta,
                gamma,
            }
            .into(),
        )
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

    fn from_lengths(x: f32, y: f32, z: f32) -> Result<Self, Self::Error> {
        Self::try_from(OrthogonalUnitCell { x, y, z }.into())
    }

    fn from_length(l: f32) -> Result<Self, Self::Error> {
        Self::try_from(CubicUnitCell(l).into())
    }

    fn from_image_distance_and_shape(d: f32, shape: UnitCellShape) -> Result<Self, Self::Error> {
        shape.vectors_with_image_distance(d).try_into()
    }

    fn to_vectors(&self) -> [[f32; 3]; 3] {
        self.clone().into().0
    }

    fn to_md_triclinic(&self) -> RestrictedTriclinicUnitCell {
        self.clone().into().restrict_orientation().reduce_tilt()
    }

    fn to_md_vectors(&self) -> [[f32; 3]; 3] {
        self.to_md_triclinic().to_vectors()
    }

    fn to_crystallographic(&self) -> CrystallographicUnitCell {
        self.to_md_triclinic().into()
    }
}

impl<T> UnitCell for T where T: Into<TriclinicUnitCell> + TryFrom<TriclinicUnitCell> + Clone {}
