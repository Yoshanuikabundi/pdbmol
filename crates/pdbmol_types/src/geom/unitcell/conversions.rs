//! # Further reading
//!
//! - <https://docs.lammps.org/Howto_triclinic.html#crystallographic-general-triclinic-representation-of-a-simulation-box>
//! - <https://doi.org/10.1002/(SICI)1096-987X(19971130)18:15%3C1930::AID-JCC8%3E3.0.CO;2-P>
//! - <http://docs.openmm.org/latest/userguide/theory/05_other_features.html#periodic-boundary-conditions>
//! - <https://manual.gromacs.org/2024.2/reference-manual/algorithms/periodic-boundary-conditions.html>

use std::error;

use itertools::Either;
use thiserror::Error;

use super::representations::*;
use crate::geom::math_utils::{cross, dot, mul, norm, sub, unit};

// Infallible conversions that preserve information exactly
//
// These are conversions that represent the same unit cell with more parameters.

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

// Fallible conversions that preserve information exactly
//
// These are conversions that represent the same unit cell with fewer
// parameters.

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
            Ok(Self {
                size_parameters: [lx, ly, lz],
                tilt_parameters: [xy, xz, yz],
            })
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

// Infallible conversions that preserve information down to machine precision
//
// These are conversions between representations with the same numbers of
// parameters.

impl From<CrystallographicUnitCell> for RestrictedTriclinicUnitCell {
    fn from(value: CrystallographicUnitCell) -> Self {
        let CrystallographicUnitCell {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        } = value;

        let lx = a;
        let xy = b * gamma.cos();
        let xz = c * beta.cos();
        let ly = (b.powi(2) - xy * xz).sqrt();
        let yz = b * c * alpha.cos() - xy * xz;
        let lz = (c.powi(2) - xz.powi(2) - yz.powi(2)).sqrt();

        Self {
            size_parameters: [lx, ly, lz],
            tilt_parameters: [xy, xz, yz],
        }
    }
}

impl From<RestrictedTriclinicUnitCell> for CrystallographicUnitCell {
    fn from(value: RestrictedTriclinicUnitCell) -> Self {
        let RestrictedTriclinicUnitCell {
            size_parameters: [lx, ly, lz],
            tilt_parameters: [xy, xz, yz],
        } = value;

        let a = lx;
        let b = (ly.powi(2) + xy.powi(2)).sqrt();
        let c = (lz.powi(2) + xz.powi(2) + yz.powi(2)).sqrt();
        let alpha = ((xy * xz + ly * yz) / (b * c)).acos();
        let beta = (xz / c).acos();
        let gamma = (xy / b).acos();

        Self {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        }
    }
}

// Infallible conversions that lose real information
//
// These are conversions that deliberately discard some information, usually
// to achieve a cell with fewer parameters.

impl TriclinicUnitCell {
    /// Reduce the unit cell by removing orientation information.
    ///
    /// The resulting unit cell is oriented such that A is aligned to the
    /// x-axis, B is in the x-y plane, and By and Cz are positive. This is the
    /// conventional orientation used by GROMACS, LAMMPS and OpenMM. This method
    /// additionally reduces the cell vectors' tilts without changing the
    /// lattice so that they each point to the nearest periodic image; see
    /// [`RestrictedTriclinicUnitCell::reduce_tilt()`].
    ///
    /// This conversion method discards the orientation of the unit cell. For a
    /// method that returns an error if orientation would be discarded, see
    /// [`RestrictedTriclinicUnitCell::try_from()`]
    ///
    /// Implemented according to the [LAMMPS manual].
    ///
    /// [LAMMPS manual]: https://docs.lammps.org/Howto_triclinic.html#transformation-from-general-to-restricted-triclinic-boxes
    pub fn restrict_orientation(&self) -> RestrictedTriclinicUnitCell {
        let &Self([a, b, c]) = self;

        let a_x = norm(a);
        let b_x = dot(b, unit(a));
        let b_y = norm(cross(unit(a), b));
        let c_x = dot(c, unit(a));
        let c_y = dot(c, cross(unit(cross(a, b)), unit(a)));
        let c_z = (dot(c, unit(cross(a, b)))).abs();

        RestrictedTriclinicUnitCell {
            size_parameters: [a_x, b_y, c_z],
            tilt_parameters: [b_x, c_x, c_y],
        }
        .reduce_tilt()
    }
}

impl RestrictedTriclinicUnitCell {
    /// Reduce the tilt parameters without changing the unit cell lattice.
    ///
    /// This method produces a lattice-equivalent unit cell that conforms to the
    /// convention required by OpenMM, GROMACS, and until recently LAMMPS:
    ///
    /// |bₓ| ≤ aₓ⁄2
    /// |cₓ| ≤ aₓ⁄2
    /// |cᵧ| ≤ bᵧ⁄2
    ///
    /// bₓ, cₓ, and cᵧ are called the `xy`, `xz`, and `yz` "tilt parameters",
    /// respectively. The tilt parameters are the nonzero off-diagonal
    /// components of the restricted representation of a triclinic cell. In
    /// other words, they are the distance that the "top" of an orthogonal unit
    /// cell vector whose "bottom" is fixed at the origin must be "pushed",
    /// along the direction of one of the other orthogonal axes, to arrive at
    /// the triclinic cell. This "pushing" skews (or tilts) the cell from a
    /// rectangular prism to a parallelepiped.
    ///
    /// You can also think of it as the distance a face of an orthogonal unit
    /// cell must be pushed sideways to tilt it into a parallelpiped. For
    /// example, imagine pushing the top face of a unit cube to the right some
    /// distance `d` while holding the bottom face still and without moving
    /// either face up or down. This operation produces a restricted triclinic
    /// cell with `xy` tilt parameter `d`. The x component of the new cell's B
    /// vector is also `d`. The other eight components of the three unit cell
    /// vectors are unchanged.
    ///
    /// While increasing the tilt parameter past the width of the cell continues
    /// to produce a more and more skewed box, the lattice represented by the
    /// unit cell is equivalent whether the cell vector points to the image
    /// above the origin cell or to the same point in any other image (as long
    /// as the vector has nonzero length). Subtracting the appropriate size
    /// parameter from a tilt parameter therefore produces an identical lattice.
    pub fn reduce_tilt(&self) -> Self {
        let &RestrictedTriclinicUnitCell {
            size_parameters: [a_x, b_y, c_z],
            tilt_parameters: [b_x, c_x, c_y],
        } = self;

        fn reduce_tilt(tilt: f32, size: f32) -> f32 {
            let tilt = tilt % size;
            if tilt <= size / 2. {
                tilt
            } else {
                tilt - size
            }
        }

        RestrictedTriclinicUnitCell {
            size_parameters: [a_x, b_y, c_z],
            tilt_parameters: [
                reduce_tilt(b_x, a_x),
                reduce_tilt(c_x, a_x),
                reduce_tilt(c_y, b_y),
            ],
        }
    }

    /// Reduce the unit cell to an orthogonal "brick" with the same volume.
    ///
    /// The resulting unit cell has the same volume as the original, and it has
    /// dimensions such that it can be tiled by the original cell vectors to
    /// fill 3D space. This makes it convenient for implementing triclinic boxes
    /// in molecular simulation code.
    ///
    /// Note that this reduction is not appropriate for converting a solvated
    /// triclinic simulation system to an orthogonal one: a freely rotating
    /// solute will sweep out a sphere, and so non-cubic orthogonal boxes
    /// waste space and require more solvent. In particular, this method
    /// produces a box with a much smaller image distance than the original. For
    /// a method that preserves image distance while producing an orthogonal
    /// unit cell, see [`to_cube()`].
    pub fn orthogonalize(&self) -> OrthogonalUnitCell {
        let TriclinicUnitCell([k, l, m]) = self.reduce_tilt().into();

        let u = k;
        let v = sub(l, mul(dot(l, unit(k)), unit(k)));
        let w = mul(dot(m, unit(cross(k, l))), unit(cross(k, l)));

        debug_assert!(u[1] < 128. * f32::EPSILON);
        debug_assert!(u[2] < 128. * f32::EPSILON);
        debug_assert!(v[0] < 128. * f32::EPSILON);
        debug_assert!(v[2] < 128. * f32::EPSILON);
        debug_assert!(w[0] < 128. * f32::EPSILON);
        debug_assert!(w[1] < 128. * f32::EPSILON);

        OrthogonalUnitCell {
            x: u[0],
            y: v[1],
            z: w[2],
        }
    }

    /// Reduce the unit cell to a cube with the same periodic image distance.
    ///
    /// Note that the new representation will have a much larger volume. The
    /// periodic image distance is the shortest distance between a point in
    /// one cell and the same point in another cell. Since a freely rotating
    /// solute sweeps out a sphere, this is the most efficient way to convert
    /// a triclinic box to a cubic box without comprimising the buffer distance.
    pub fn to_cube(&self) -> CubicUnitCell {
        let RestrictedTriclinicUnitCell {
            size_parameters: [a_x, b_y, c_z],
            tilt_parameters: [b_x, c_x, c_y],
        } = self.reduce_tilt();

        let a = a_x;
        let b = f32::sqrt(b_x.powi(2) + b_y.powi(2));
        let c = f32::sqrt(c_x.powi(2) + c_y.powi(2) + c_z.powi(2));

        CubicUnitCell(a.min(b).min(c))
    }
}

// Infallible conversions composed of the above

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

// Fallible conversions composed of the above

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
