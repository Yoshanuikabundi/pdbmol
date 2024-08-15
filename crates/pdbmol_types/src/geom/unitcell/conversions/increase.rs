//!
//! Infallible conversions that preserve information exactly. These are
//! conversions that represent the same unit cell with more parameters.

use crate::geom::unitcell::*;

impl From<CubicUnitCell> for OrthogonalUnitCell {
    fn from(value: CubicUnitCell) -> Self {
        let CubicUnitCell(l) = value;
        Self { x: l, y: l, z: l }
    }
}

impl From<OrthogonalUnitCell> for RestrictedTriclinicUnitCell {
    fn from(value: OrthogonalUnitCell) -> Self {
        Self {
            size_parameters: [value.x, value.y, value.z],
            tilt_parameters: [0.0, 0.0, 0.0],
        }
    }
}

impl From<OrthogonalUnitCell> for CrystallographicUnitCell {
    fn from(value: OrthogonalUnitCell) -> Self {
        let OrthogonalUnitCell { x: a, y: b, z: c } = value;
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

impl From<RestrictedTriclinicUnitCell> for TriclinicUnitCell {
    fn from(value: RestrictedTriclinicUnitCell) -> Self {
        let RestrictedTriclinicUnitCell {
            size_parameters: [lx, ly, lz],
            tilt_parameters: [xy, xz, yz],
        } = value;

        Self([[lx, 0.0, 0.0], [xy, ly, 0.0], [xz, yz, lz]])
    }
}

mod composite {
    //! Infallible conversions composed of the above

    use super::*;

    impl From<CubicUnitCell> for RestrictedTriclinicUnitCell {
        fn from(value: CubicUnitCell) -> Self {
            OrthogonalUnitCell::from(value).into()
        }
    }

    impl From<CubicUnitCell> for CrystallographicUnitCell {
        fn from(value: CubicUnitCell) -> Self {
            RestrictedTriclinicUnitCell::from(value).into()
        }
    }

    impl From<CubicUnitCell> for TriclinicUnitCell {
        fn from(value: CubicUnitCell) -> Self {
            RestrictedTriclinicUnitCell::from(value).into()
        }
    }

    impl From<OrthogonalUnitCell> for TriclinicUnitCell {
        fn from(value: OrthogonalUnitCell) -> Self {
            RestrictedTriclinicUnitCell::from(value).into()
        }
    }

    impl From<CrystallographicUnitCell> for TriclinicUnitCell {
        fn from(value: CrystallographicUnitCell) -> Self {
            RestrictedTriclinicUnitCell::from(value).into()
        }
    }
}
