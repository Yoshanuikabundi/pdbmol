use std::f32::consts::PI;

use super::representations::{
    CrystallographicUnitCell, CubicUnitCell, OrientedTriclinicUnitCell, OrthorhombicUnitCell,
    TriclinicUnitCell,
};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
enum LatticeCreationError {
    #[error("reduce this unit cell to produce a lattice")]
    ReducibleLattice(TriclinicUnitCell),
    #[error("NaN or infinite value in lattice parameters")]
    NonFiniteParameters(TriclinicUnitCell),
}

/// A conventionally reduced, unique lattice representation.
///
/// Any *lattice* may be represented by this type, but not any *unit cell*. For
/// instance, rotating a cube 45 degrees around the x axis produces a different
/// unit cell, but the same lattice. Both cells would have the same
/// representation in this type. For representations of specific unit cells, see
/// [`super::representations`].
///
/// Any distinct lattice has a unique representation in this type. Round trip
/// conversions through this type will produce a unit cell with the same lattice
/// as the original cell, but in the conventional representation. This means
/// information will be lost if the unit cell is not already in the conventional
/// representation. For a type that can represent an arbitrary unit cell, see
/// [`OrientedTriclinicUnitCell`] (though note that even this representation has
/// a vertex fixed at the origin).
///
/// # Conventional representation
///
/// The conventional representation of a triclinic cell has the following cell
/// vectors:
///
/// >&nbsp;                  A = [   aₓ    0     0   ]
///
/// >&nbsp;                  B = [   bₓ    bᵧ    0   ]
///
/// >&nbsp;                  C = [   cₓ    cᵧ    c𝓏   ]
///
/// In addition, the conventional representation has the following constraints:
///
/// >&nbsp;                            aₓ > 0
///
/// >&nbsp;                            bᵧ > 0
///
/// >&nbsp;                            c𝓏 > 0
///
/// >&nbsp;                      -aₓ⁄2 < bₓ ≤ aₓ⁄2
///
/// >&nbsp;                      -aₓ⁄2 < cₓ ≤ aₓ⁄2
///
/// >&nbsp;                      -bᵧ⁄2 < cᵧ ≤ bᵧ⁄2
///
/// This combination of definition and constraints means that:
/// - A is aligned to the x-axis
/// - B is in the x-y plane
/// - Size parameters are all positive
/// - Tilt parameters are all as close as possible to 0 for the given lattice
///
///
/// Note that this representation has a subtle additional constraint to those
/// in GROMACS, OpenMM and LAMMPS: the tilt parameters here have an open lower
/// bound, while in those packages documentations the bounds are closed on
/// both sides. This means that the conventional representations described in
/// those packages technically have degeneracies when the tilt parameters are
/// equal to their bounds. In reality of course, all these calculations are made
/// with limited precision and floating point types, and this subtlety is
/// extremely unlikely to ever matter.
// TODO: Do the bounds on the tilt parameters imply easily expressible bounds on cell angles?
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lattice {
    size_parameters: [f32; 3],
    tilt_parameters: [f32; 3],
}

impl Lattice {
    pub fn size_parameters(&self) -> [f32; 3] {
        self.size_parameters
    }

    pub fn tilt_parameters(&self) -> [f32; 3] {
        self.tilt_parameters
    }

    pub fn ax(&self) -> f32 {
        self.size_parameters[0]
    }

    pub fn bx(&self) -> f32 {
        self.tilt_parameters[0]
    }

    pub fn by(&self) -> f32 {
        self.size_parameters[1]
    }

    pub fn cx(&self) -> f32 {
        self.tilt_parameters[1]
    }

    pub fn cy(&self) -> f32 {
        self.tilt_parameters[2]
    }

    pub fn cz(&self) -> f32 {
        self.size_parameters[2]
    }

    pub fn a_vector(&self) -> [f32; 3] {
        let Lattice {
            size_parameters: [ax, _, _],
            tilt_parameters: [_, _, _],
        } = *self;
        [ax, 0.0, 0.0]
    }

    pub fn b_vector(&self) -> [f32; 3] {
        let Lattice {
            size_parameters: [_, by, _],
            tilt_parameters: [bx, _, _],
        } = *self;
        [bx, by, 0.0]
    }

    pub fn c_vector(&self) -> [f32; 3] {
        let Lattice {
            size_parameters: [_, _, cz],
            tilt_parameters: [_, cx, cy],
        } = *self;
        [cx, cy, cz]
    }

    pub fn vectors(&self) -> [[f32; 3]; 3] {
        let Lattice {
            size_parameters: [ax, by, cz],
            tilt_parameters: [bx, cx, cy],
        } = *self;
        [[ax, 0.0, 0.0], [bx, by, 0.0], [cx, cy, cz]]
    }

    pub fn a(&self) -> f32 {
        let Lattice { size_parameters: [ax, _, _], .. } = *self;
        ax
    }

    pub fn b(&self) -> f32 {
        let Lattice {
            size_parameters: [_, by, _],
            tilt_parameters: [bx, _, _],
        } = *self;
        f32::sqrt(bx.powi(2) + by.powi(2))
    }

    pub fn c(&self) -> f32 {
        let Lattice {
            size_parameters: [_, _, cz],
            tilt_parameters: [_, cx, cy],
        } = *self;
        f32::sqrt(cx.powi(2) + cy.powi(2) + cz.powi(2))
    }

    pub fn alpha_rad(&self) -> f32 {
        let Lattice {
            size_parameters: [_, by, _],
            tilt_parameters: [bx, cx, cy],
        } = *self;
        let b = self.b();
        let c = self.c();

        (bx * cx + by * cy) / b * c
    }

    pub fn beta_rad(&self) -> f32 {
        let Lattice {
            size_parameters: [_, _, _],
            tilt_parameters: [_, cx, _],
        } = *self;
        let c = self.c();

        f32::acos(cx / c)
    }

    pub fn gamma_rad(&self) -> f32 {
        let Lattice {
            size_parameters: [_, _, _],
            tilt_parameters: [bx, _, _],
        } = *self;
        let b = self.b();

        bx / b
    }

    pub fn alpha_deg(&self) -> f32 {
        self.alpha_rad() * 180.0 / PI
    }

    pub fn beta_deg(&self) -> f32 {
        self.beta_rad() * 180.0 / PI
    }

    pub fn gamma_deg(&self) -> f32 {
        self.beta_rad() * 180.0 / PI
    }

    /// Side lengths of a "brick" whose tiling by the lattice vectors covers 3D space.
    ///
    /// The brick has the same volume as the original lattice, and it has
    /// dimensions such that it can be tiled by the original cell vectors to
    /// fill 3D space. This makes it convenient for implementing triclinic boxes
    /// in molecular simulation code, or for tiling points.
    ///
    /// Note that this reduction is not appropriate for converting a solvated
    /// triclinic simulation system to an orthogonal one: a freely rotating
    /// solute will sweep out a sphere, and so non-cubic orthogonal boxes
    /// waste space and require more solvent. In particular, this method
    /// produces a box with a much smaller image distance than the original.
    pub fn brick(&self) -> [f32; 3] {
        let [u, v, w] = OrientedTriclinicUnitCell::from(*self).brick();

        debug_assert!(u[1] < 128. * f32::EPSILON);
        debug_assert!(u[2] < 128. * f32::EPSILON);
        debug_assert!(v[0] < 128. * f32::EPSILON);
        debug_assert!(v[2] < 128. * f32::EPSILON);
        debug_assert!(w[0] < 128. * f32::EPSILON);
        debug_assert!(w[1] < 128. * f32::EPSILON);

        [u[0], v[1], w[2]]
    }

    fn new(
        size_parameters: [f32; 3],
        tilt_parameters: [f32; 3],
    ) -> Result<Self, LatticeCreationError> {
        // Ensuring parameters are finite should allow us to implement Eq,
        // and it's a nice invariant to have
        let all_parameters_are_finite = size_parameters
            .into_iter()
            .chain(tilt_parameters)
            .all(f32::is_finite);

        // The lattice representation ensures that A is aligned to the (positive
        // or negative) x axis and that B is in the XY plane, so orientation
        // requirements are covered by ensuring the size parameters are positive
        let size_parameters_are_positive = size_parameters
            .into_iter()
            .all(|size| size > 0.0);

        let [ax, by, _] = size_parameters;
        let tilt_parameters_are_reduced = tilt_parameters
            .into_iter()
            .zip([ax, by, by])
            .all(|(tilt, size)| (size / 2.0 < tilt) & (tilt <= size / 2.0));

        if !all_parameters_are_finite {
            Err(LatticeCreationError::NonFiniteParameters(
                TriclinicUnitCell { size_parameters, tilt_parameters },
            ))
        } else if size_parameters_are_positive & tilt_parameters_are_reduced {
            Ok(Self { size_parameters, tilt_parameters })
        } else {
            Err(LatticeCreationError::ReducibleLattice(TriclinicUnitCell {
                size_parameters,
                tilt_parameters,
            }))
        }
    }
}

impl From<OrientedTriclinicUnitCell> for Lattice {
    fn from(value: OrientedTriclinicUnitCell) -> Self {
        value.restrict_orientation().into()
    }
}

impl From<TriclinicUnitCell> for Lattice {
    fn from(value: TriclinicUnitCell) -> Self {
        let TriclinicUnitCell { size_parameters, tilt_parameters } = value
            .restrict_orientation()
            .reduce_tilt();
        match Self::new(size_parameters, tilt_parameters) {
            Err(LatticeCreationError::ReducibleLattice(_)) => unreachable!(
                "restrict_orientation().reduce_tilt() should always produce a conventional lattice"
            ),
            Err(e @ LatticeCreationError::NonFiniteParameters(_)) => panic!("{e}"),
            Ok(lattice) => lattice,
        }
    }
}

impl From<CrystallographicUnitCell> for Lattice {
    fn from(value: CrystallographicUnitCell) -> Self {
        TriclinicUnitCell::from(value).into()
    }
}

impl From<OrthorhombicUnitCell> for Lattice {
    fn from(value: OrthorhombicUnitCell) -> Self {
        TriclinicUnitCell::from(value).into()
    }
}

impl From<CubicUnitCell> for Lattice {
    fn from(value: CubicUnitCell) -> Self {
        TriclinicUnitCell::from(value).into()
    }
}

impl From<Lattice> for OrientedTriclinicUnitCell {
    fn from(value: Lattice) -> Self {
        TriclinicUnitCell::from(value).into()
    }
}

impl From<Lattice> for TriclinicUnitCell {
    fn from(value: Lattice) -> Self {
        let Lattice { size_parameters, tilt_parameters } = value;
        Self { size_parameters, tilt_parameters }
    }
}

impl From<Lattice> for CrystallographicUnitCell {
    fn from(value: Lattice) -> Self {
        TriclinicUnitCell::from(value).into()
    }
}

impl TryFrom<Lattice> for OrthorhombicUnitCell {
    type Error = <OrthorhombicUnitCell as TryFrom<TriclinicUnitCell>>::Error;

    fn try_from(value: Lattice) -> Result<Self, Self::Error> {
        TriclinicUnitCell::from(value).try_into()
    }
}

impl TryFrom<Lattice> for CubicUnitCell {
    type Error = <CubicUnitCell as TryFrom<TriclinicUnitCell>>::Error;

    fn try_from(value: Lattice) -> Result<Self, Self::Error> {
        TriclinicUnitCell::from(value).try_into()
    }
}
