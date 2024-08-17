//!
//! Infallible conversions that preserve information exactly. These are
//! conversions that represent the same unit cell with more parameters.

use super::super::representations::*;

impl From<CubicUnitCell> for OrthorhombicUnitCell {
    fn from(value: CubicUnitCell) -> Self {
        let CubicUnitCell(l) = value;
        Self { x: l, y: l, z: l }
    }
}

impl From<OrthorhombicUnitCell> for TriclinicUnitCell {
    fn from(value: OrthorhombicUnitCell) -> Self {
        Self {
            size_parameters: [value.x, value.y, value.z],
            tilt_parameters: [0.0, 0.0, 0.0],
        }
    }
}

impl From<OrthorhombicUnitCell> for CrystallographicUnitCell {
    fn from(value: OrthorhombicUnitCell) -> Self {
        let OrthorhombicUnitCell { x: a, y: b, z: c } = value;
        Self {
            a,
            b,
            c,
            alpha: std::f32::consts::FRAC_PI_2,
            beta: std::f32::consts::FRAC_PI_2,
            gamma: std::f32::consts::FRAC_PI_2,
        }
    }
}

impl From<TriclinicUnitCell> for OrientedTriclinicUnitCell {
    fn from(value: TriclinicUnitCell) -> Self {
        let TriclinicUnitCell {
            size_parameters: [lx, ly, lz],
            tilt_parameters: [xy, xz, yz],
        } = value;

        Self([[lx, 0.0, 0.0], [xy, ly, 0.0], [xz, yz, lz]])
    }
}

mod composite {
    //! Infallible conversions composed of the above

    use super::*;

    impl From<CubicUnitCell> for TriclinicUnitCell {
        fn from(value: CubicUnitCell) -> Self {
            OrthorhombicUnitCell::from(value).into()
        }
    }

    impl From<CubicUnitCell> for CrystallographicUnitCell {
        fn from(value: CubicUnitCell) -> Self {
            TriclinicUnitCell::from(value).into()
        }
    }

    impl From<CubicUnitCell> for OrientedTriclinicUnitCell {
        fn from(value: CubicUnitCell) -> Self {
            TriclinicUnitCell::from(value).into()
        }
    }

    impl From<OrthorhombicUnitCell> for OrientedTriclinicUnitCell {
        fn from(value: OrthorhombicUnitCell) -> Self {
            TriclinicUnitCell::from(value).into()
        }
    }

    impl From<CrystallographicUnitCell> for OrientedTriclinicUnitCell {
        fn from(value: CrystallographicUnitCell) -> Self {
            TriclinicUnitCell::from(value).into()
        }
    }
}
