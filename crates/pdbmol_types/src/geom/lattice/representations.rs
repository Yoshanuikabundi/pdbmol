// 9 parameter representations - 3 sizes, 3 tilts, 3 orientations
#[derive(Debug, Clone, Copy)]
pub struct OrientedTriclinicUnitCell(pub [[f32; 3]; 3]);

impl OrientedTriclinicUnitCell {
    /// Unit cell vectors of a "brick" whose tiling by the lattice vectors covers 3D space.
    ///
    /// The brick has the same volume and orientation as the original unit cell,
    /// and it has dimensions such that it can be tiled by the original
    /// triclinic lattice vectors to fill 3D space. This makes it convenient for
    /// implementing triclinic boxes in molecular simulation code, or for tiling
    /// boxes of different shapes.
    ///
    /// Note that this reduction is not appropriate for converting a solvated
    /// triclinic simulation system to an orthogonal one: a freely rotating
    /// solute will sweep out a sphere, and so non-cubic orthogonal boxes
    /// waste space and require more solvent. In particular, this method
    /// produces an irregular rectangular prism with a much smaller image
    /// distance than the original.
    ///
    /// Sorting the lattice vectors before calling this function may make it
    /// easier to compute minimum image distances for points later on; see [^1].
    ///
    /// [^1]: Bekker, H. (1997), *Unification of box shapes in molecular
    /// simulations.* [J. Comput. Chem., 18: 1930-1942.]
    ///
    /// [J. Comput. Chem., 18: 1930-1942.]: https://doi.org/10.1002/(SICI)1096-987X(19971130)18:15<1930::AID-JCC8>3.0.CO;2-P
    pub fn brick(&self) -> [[f32; 3]; 3] {
        use crate::geom::math_utils::*;
        let [k, l, m] = self.0;

        let u = k;
        let v = sub(l, mul(dot(l, unit(k)), unit(k)));
        let w = mul(dot(m, unit(cross(k, l))), unit(cross(k, l)));

        [u, v, w]
    }
}

// 6 parameter representations - 3 sizes, 3 tilts
#[derive(Debug, Clone, Copy)]
pub struct CrystallographicUnitCell {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32,
}

#[derive(Debug, Clone, Copy)]
/// A representation of a triclinic unit cell without an orientation.
///
/// A triclinic unit cell represents the unit cell of the most general crystal
/// system. It has 6 parameters, is shaped like a parallelepiped, and can
/// represent any shape whose translational tesselations tile 3D space. It can
/// be generated from an [orthorhombic] unit cell (an axis-aligned rectangular
/// prism) by first pushing the top face to the right and then the back face up
/// and to the right.
///
/// A triclinic unit cell's translation vectors are:
///
/// >&nbsp;                  A = [   aₓ    0     0   ]
///
/// >&nbsp;                  B = [   bₓ    bᵧ    0   ]
///
/// >&nbsp;                  C = [   cₓ    cᵧ    c𝓏   ]
///
/// Any triclinic unit cell can be reduced to this form by rotating it such that
/// A is aligned to the x-axis and B is in the x-y plane.
///
/// The diagonal elements aₓ, bᵧ and c𝓏 are called "size parameters". They
/// represent the side lengths of the orthorhombic cell used to generate this
/// triclinic cell. The off diagonal values bₓ, cₓ, and cᵧ are called "tilt
/// parameters".  They represent the distances that the faces of
/// the generating orthorhombic were pushed to produce the triclinic cell. They
/// are respectively also called (for example, in the LAMMPS documentation)
/// `xy`, `xz`, and `yz`.
///
/// This representation does not uniquely define a triclinic lattice; each
/// representation can be modified without changing the lattice either by
/// the tilt reduction procedure described below or by certain combinations of
/// rotations around the axes, which produce variants that have differently
/// signed size parameters.
///
/// [orthorhombic]: OrthorhombicUnitCell
///
/// # Tilt parameters
///
/// Applying the tilt parameters to an orthorhombic unit cell skews (or tilts)
/// the cell from an axis-aligned rectangular prism to a parallelepiped. The
/// first tilt parameter bₓ pushes the top face of the rectangular prism to
/// the right (positive x direction); the second cₓ pushes the back face to the
/// right, and the third cᵧ pushes the back face up (positive y direction).
///
/// For example, imagine pushing the top face of a unit cube to the right some
/// distance `d` while holding the bottom face still and without moving either
/// face up or down. This operation produces a triclinic cell with bₓ tilt
/// parameter `d`. The x component of the new cell's B vector is also now `d`.
/// The other eight components of the three unit cell vectors are unchanged: the
/// three diagonal elements are still equal, and the two off-diagonal elements
/// of C are still zero.
///
/// The tilt parameters can be increased indefinitely to produce a more and more
/// skewed box, but when they exceed the size parameter associated with the
/// pushing direction the lattice represented by the cell "wraps around". For
/// example, think of how the unit cell vectors of an orthorhombic cell stretch
/// from a point in one image to the same point in a neighbouring image. The A
/// vector, which is x-aligned, points to the image to the right, the B vector,
/// which is y-aligned, points to the image above, and so the vector B - A
/// points to the same point in the image above and to the left. Increasing
/// the tilt parameter bₓ tilts both B and B - A to the right, and as bₓ equals
/// the length of the A vector aₓ, B - A becomes vertical, just as B was before
/// we started pushing. The new unit cell is a different shape, but represents
/// the same orthorhombic lattice. The fully tilted B now points to the image
/// above and to the right of the original orthorhombic unit cell.
///
/// The tilt parameters can therefore be reduced without changing the lattice
/// by subtracting any integer multiple of the relevant size parameter. The
/// relevant size parameter is the diagonal element of the tilt parameter's
/// column, so tilt parameters bₓ and cₓ are associated with size parameter aₓ
/// and tilt parameter cᵧ is associated with bᵧ. By convention, reduced tilts
/// are restricted to a range centered on zero. To perform this reduction, see
/// [`TriclinicUnitCell.reduce_tilt()`]
pub struct TriclinicUnitCell {
    pub size_parameters: [f32; 3],
    pub tilt_parameters: [f32; 3],
}

impl TriclinicUnitCell {
    /// Rotate the unit cell so that all size parameters are positive without changing the lattice.
    pub fn restrict_orientation(&self) -> Self {
        OrientedTriclinicUnitCell::from(*self).restrict_orientation()
    }

    /// Reduce the tilt parameters without changing the unit cell lattice.
    ///
    /// This method produces a lattice-equivalent unit cell that conforms to the
    /// convention required by OpenMM, GROMACS, and until recently LAMMPS:
    ///
    /// >&nbsp;                    |bₓ| ≤ aₓ ⁄ 2
    ///
    /// >&nbsp;                    |cₓ| ≤ aₓ ⁄ 2
    ///
    /// >&nbsp;                    |cᵧ| ≤ bᵧ ⁄ 2
    ///
    /// bₓ, cₓ, and cᵧ are called the `xy`, `xz`, and `yz` "tilt parameters",
    /// respectively. The tilt parameters are the nonzero off-diagonal
    /// components of the restricted representation of a triclinic cell. For
    /// more details, see [`TriclinicUnitCell`]
    pub fn reduce_tilt(&self) -> Self {
        let &TriclinicUnitCell {
            size_parameters: [a_x, b_y, c_z],
            tilt_parameters: [b_x, c_x, c_y],
        } = self;

        fn reduce_tilt_inner(
            tilt: f32,
            size: f32,
        ) -> f32 {
            let tilt = tilt % size;
            if tilt <= size / 2. {
                tilt
            } else {
                tilt - size
            }
        }

        TriclinicUnitCell {
            size_parameters: [a_x, b_y, c_z],
            tilt_parameters: [
                reduce_tilt_inner(b_x, a_x),
                reduce_tilt_inner(c_x, a_x),
                reduce_tilt_inner(c_y, b_y),
            ],
        }
    }
}

// 3 parameter representations - 3 sizes
//
// Unit cell is axis aligned and orthogonal.
#[derive(Debug, Clone, Copy)]
pub struct OrthorhombicUnitCell {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// 1 parameter representations - 1 size
//
// Unit cell is axis aligned, orthogonal, and regular.
#[derive(Debug, Clone, Copy)]
pub struct CubicUnitCell(pub f32);
