//! # Further reading
//!
//! - <https://docs.lammps.org/Howto_triclinic.html#crystallographic-general-triclinic-representation-of-a-simulation-box>
//! - <https://doi.org/10.1002/(SICI)1096-987X(19971130)18:15%3C1930::AID-JCC8%3E3.0.CO;2-P>
//! - <http://docs.openmm.org/latest/userguide/theory/05_other_features.html#periodic-boundary-conditions>
//! - <https://manual.gromacs.org/2024.2/reference-manual/algorithms/periodic-boundary-conditions.html>

mod conversions;
mod lattice;
pub mod representations;
pub mod shapes;
mod tilings;
mod unitcell;

pub use lattice::Lattice;
pub use shapes::LatticeShape;
pub use unitcell::UnitCell;
