use std::{collections::HashMap, sync::LazyLock};

pub use pdbmol_ccdparse::datatypes;

pub static CCD: LazyLock<HashMap<&str, datatypes::Residue>> = LazyLock::new(|| {
    let cif_str = include_str!("../../../data/ccd-20240406.cif");
    pdbmol_ccdparse::parse_ccd(cif_str).unwrap()
});
