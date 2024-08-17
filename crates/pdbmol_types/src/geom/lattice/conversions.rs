use super::representations::*;
use crate::geom::math_utils::{cross, dot, norm, unit};

/// Conversions from and to external types
mod external;
/// Fallible conversions that represent the same unit cell in fewer parameters.
mod fallible_decrease;
/// Infallible conversions that represent the same unit cell with more parameters.
mod increase;

// Infallible conversions that preserve information down to machine precision
//
// These are conversions between representations with the same numbers of
// parameters.

impl From<CrystallographicUnitCell> for TriclinicUnitCell {
    fn from(value: CrystallographicUnitCell) -> Self {
        let CrystallographicUnitCell { a, b, c, alpha, beta, gamma } = value;

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

impl From<TriclinicUnitCell> for CrystallographicUnitCell {
    fn from(value: TriclinicUnitCell) -> Self {
        let TriclinicUnitCell {
            size_parameters: [lx, ly, lz],
            tilt_parameters: [xy, xz, yz],
        } = value;

        let a = lx;
        let b = (ly.powi(2) + xy.powi(2)).sqrt();
        let c = (lz.powi(2) + xz.powi(2) + yz.powi(2)).sqrt();
        let alpha = ((xy * xz + ly * yz) / (b * c)).acos();
        let beta = (xz / c).acos();
        let gamma = (xy / b).acos();

        Self { a, b, c, alpha, beta, gamma }
    }
}

// Infallible conversions that lose real information
//
// These are conversions that deliberately discard some information, usually
// to achieve a cell with fewer parameters.

impl OrientedTriclinicUnitCell {
    /// Reduce the unit cell representation by removing orientation information.
    ///
    /// The resulting unit cell is oriented such that A is aligned to the
    /// x-axis, B is in the x-y plane, and By and Cz are positive. This is the
    /// conventional orientation used by GROMACS, LAMMPS and OpenMM.
    ///
    /// This method does not reduce the unit cell's tilt, and so the generated
    /// box may not be compatible with MD engines. To reduce the tilt and attain
    /// the convential MD reduced vector representation, follow a call to this
    /// function with a call to [`TriclinicUnitCell::reduce_tilt()`].
    ///
    /// This conversion method discards the orientation of the unit cell. For a
    /// method that returns an error if orientation would be discarded, see
    /// [`TriclinicUnitCell::try_from()`]
    ///
    /// Implemented according to the [LAMMPS manual].
    ///
    /// [LAMMPS manual]: https://docs.lammps.org/Howto_triclinic.html#transformation-from-general-to-restricted-triclinic-boxes
    pub fn restrict_orientation(&self) -> TriclinicUnitCell {
        let &Self([a, b, c]) = self;

        let a_x = norm(a);
        let b_x = dot(b, unit(a));
        let b_y = norm(cross(unit(a), b));
        let c_x = dot(c, unit(a));
        let c_y = dot(c, cross(unit(cross(a, b)), unit(a)));
        let c_z = (dot(c, unit(cross(a, b)))).abs();

        TriclinicUnitCell {
            size_parameters: [a_x, b_y, c_z],
            tilt_parameters: [b_x, c_x, c_y],
        }
    }
}
