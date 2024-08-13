#![doc = include_str!("../README.md")]

pub use pdbmol_ccd as ccd;
pub mod parser {
    pub use pdbmol_cif as cif;
    pub use pdbmol_pdb as pdb;
}
