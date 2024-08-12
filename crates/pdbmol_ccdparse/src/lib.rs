use std::{collections::HashMap, error::Error};

pub mod datatypes;

use datatypes::Residue;

pub fn parse_ccd(s: &str) -> Result<HashMap<&str, Residue>, Box<dyn Error>> {
    let cif_parsed = pdbmol_cif::parse(s).map_err(|e| e.to_string())?;
    cif_parsed
        .into_iter()
        .filter(|(key, _)| *key != "UNL") // Filter out special residues
        .map(|(key, value)| {
            Residue::try_from(&value)
                .map(|v| (key, v))
                .map_err(Into::into)
        })
        .collect()
}
