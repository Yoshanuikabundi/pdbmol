//!
//! These are conversions that represent the same unit cell with fewer
//! parameters. They raise errors if the unit cell cannot be represented that
//! way.

use thiserror::Error;

use crate::geom::unitcell::*;

#[derive(Error, Debug)]
#[error(
    "orientation is not reduced: A not aligned to x axis, B not in xy plane, or By or Cz negative"
)]
pub struct NonReducedOrientationErr;

impl TryFrom<TriclinicUnitCell> for RestrictedTriclinicUnitCell {
    type Error = NonReducedOrientationErr;

    /// This conversion method returns an error if the unit cell is not in the
    /// restricted orientation. No allowance is made for nonzero floating point
    /// values; even an orientation machine epsilon away from the restricted
    /// orientation will return an error. For a method that discards the
    /// orientation, see [`RestrictedTriclinicUnitCell::from()`]
    fn try_from(value: TriclinicUnitCell) -> Result<Self, Self::Error> {
        if let TriclinicUnitCell([[lx, 0.0, 0.0], [xy, ly, 0.0], [xz, yz, lz]]) = value {
            Ok(Self { size_parameters: [lx, ly, lz], tilt_parameters: [xy, xz, yz] })
        } else {
            Err(NonReducedOrientationErr)
        }
    }
}

#[derive(Error, Debug)]
#[error("unit cell is not orthogonal: all vectors must be perpendicular")]
pub struct NonOrthogonalUnitCellError;

impl TryFrom<RestrictedTriclinicUnitCell> for OrthogonalUnitCell {
    type Error = NonOrthogonalUnitCellError;

    /// This conversion method returns an error if the unit cell is not already
    /// orthogonal. No allowance is made for nonzero floating point values; even
    /// a tilt machine epsilon away from orthogonal will return an error.
    fn try_from(value: RestrictedTriclinicUnitCell) -> Result<Self, Self::Error> {
        if let RestrictedTriclinicUnitCell {
            size_parameters: [x, y, z],
            tilt_parameters: [0.0, 0.0, 0.0],
        } = value
        {
            Ok(Self { x, y, z })
        } else {
            Err(NonOrthogonalUnitCellError)
        }
    }
}

#[derive(Error, Debug)]
#[error("unit cell is not regular: all cell distances must be identical")]
pub struct NonRegularUnitCellError;

impl TryFrom<OrthogonalUnitCell> for CubicUnitCell {
    type Error = NonRegularUnitCellError;

    /// This conversion method returns an error if the unit cell is not already
    /// cubic. No allowance is made for nonzero floating point values; even
    /// a stretch machine epsilon away from cubic will return an error.
    fn try_from(value: OrthogonalUnitCell) -> Result<Self, Self::Error> {
        let OrthogonalUnitCell { x, y, z } = value;
        if (x == y) & (x == z) {
            Ok(Self(x))
        } else {
            Err(NonRegularUnitCellError)
        }
    }
}

mod composite {
    //! Fallible conversions composed of the above

    use super::*;
    use itertools::Either;

    impl TryFrom<TriclinicUnitCell> for CrystallographicUnitCell {
        type Error = NonReducedOrientationErr;

        fn try_from(value: TriclinicUnitCell) -> Result<Self, Self::Error> {
            Ok(RestrictedTriclinicUnitCell::try_from(value)?.into())
        }
    }

    impl TryFrom<TriclinicUnitCell> for OrthogonalUnitCell {
        type Error = Either<NonReducedOrientationErr, NonOrthogonalUnitCellError>;

        fn try_from(value: TriclinicUnitCell) -> Result<Self, Self::Error> {
            RestrictedTriclinicUnitCell::try_from(value)
                .map_err(Either::Left)?
                .try_into()
                .map_err(Either::Right)
        }
    }

    impl TryFrom<TriclinicUnitCell> for CubicUnitCell {
        type Error = Either<
            Either<NonReducedOrientationErr, NonOrthogonalUnitCellError>,
            NonRegularUnitCellError,
        >;

        fn try_from(value: TriclinicUnitCell) -> Result<Self, Self::Error> {
            OrthogonalUnitCell::try_from(value)
                .map_err(Either::Left)?
                .try_into()
                .map_err(Either::Right)
        }
    }

    impl TryFrom<RestrictedTriclinicUnitCell> for CubicUnitCell {
        type Error = Either<NonOrthogonalUnitCellError, NonRegularUnitCellError>;

        fn try_from(value: RestrictedTriclinicUnitCell) -> Result<Self, Self::Error> {
            OrthogonalUnitCell::try_from(value)
                .map_err(Either::Left)?
                .try_into()
                .map_err(Either::Right)
        }
    }

    impl TryFrom<CrystallographicUnitCell> for OrthogonalUnitCell {
        type Error = NonOrthogonalUnitCellError;

        fn try_from(value: CrystallographicUnitCell) -> Result<Self, Self::Error> {
            RestrictedTriclinicUnitCell::from(value).try_into()
        }
    }

    impl TryFrom<CrystallographicUnitCell> for CubicUnitCell {
        type Error = Either<NonOrthogonalUnitCellError, NonRegularUnitCellError>;

        fn try_from(value: CrystallographicUnitCell) -> Result<Self, Self::Error> {
            OrthogonalUnitCell::try_from(value)
                .map_err(Either::Left)?
                .try_into()
                .map_err(Either::Right)
        }
    }
}
