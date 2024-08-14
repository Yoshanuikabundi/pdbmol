#![doc = include_str!("../README.md")]

#[doc(inline)]
pub use pdbmol_ccd as ccd;

pub use pdbmol_types::Element;

pub mod parser {
    #[doc(inline)]
    pub use pdbmol_cif as cif;

    #[doc(inline)]
    pub use pdbmol_pdb as pdb;
}
