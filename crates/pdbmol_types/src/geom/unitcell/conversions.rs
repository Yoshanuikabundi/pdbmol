use super::representations::*;
use crate::geom::math_utils::{cross, dot, mul, norm, sub, unit};

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
    /// conventional orientation used by GROMACS, LAMMPS and OpenMM.
    ///
    /// This method does not reduce the unit cell's tilt, and so the generated
    /// box may not be compatible with MD engines. To reduce the tilt and attain
    /// the convential MD reduced vector representation, follow a call to this
    /// function with a call to [`RestrictedTriclinicUnitCell::reduce_tilt()`].
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
}
