#![doc = include_str!("../README.md")]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

pub use pdbmol_ccd as ccd;
pub mod parser {
    pub use pdbmol_cif as cif;
}
