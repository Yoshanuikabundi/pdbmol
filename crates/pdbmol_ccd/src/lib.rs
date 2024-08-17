use std::{collections::HashMap, sync::LazyLock};

use pdbmol_types::ResidueDefinition;

pub static CCD: LazyLock<HashMap<&str, ResidueDefinition<'static>>> = LazyLock::new(|| {
    let cif_str = include_str!("../../../data/ccd-20240406.cif");
    pdbmol_ccdparse::parse_ccd(cif_str).unwrap()
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    /// Takes a little over a minute with --release (much longer w/o optimizations)
    /// Run with `cargo test -p pdbmol_ccd --release -- --ignored`
    fn test_ccd() {
        let phenylalanine = &CCD["PHE"];
        assert_eq!(phenylalanine.id, "PHE")
    }
}
