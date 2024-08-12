//! https://mmcif.wwpdb.org/dictionaries/mmcif_pdbx_v50.dic/Categories/chem_comp.html

use std::{collections::HashMap, str::FromStr};

use crate::parser::cif::ParsedDataBlock;

#[derive(Debug, Clone, PartialEq)]
enum LinkingType {
    DBetaPeptideCGammaLinking,
    DGammaPeptideCDeltaLinking,
    DPeptideCoohCarboxyTerminus,
    DPeptideNh3AminoTerminus,
    DPeptideLinking,
    DSaccharide,
    DSaccharideAlphaLinking,
    DSaccharideBetaLinking,
    DNAOh3PrimeTerminus,
    DNAOh5PrimeTerminus,
    DNALinking,
    LDNALinking,
    LRNALinking,
    LBetaPeptideCGammaLinking,
    LGammaPeptideCDeltaLinking,
    LPeptideCoohCarboxyTerminus,
    LPeptideNh3AminoTerminus,
    LPeptideLinking,
    LSaccharide,
    LSaccharideAlphaLinking,
    LSaccharideBetaLinking,
    RNAOh3PrimeTerminus,
    RNAOh5PrimeTerminus,
    RNALinking,
    NonPolymer,
    Other,
    PeptideLinking,
    PeptideLike,
    Saccharide,
}

impl FromStr for LinkingType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "d-beta-peptide, c-gamma linking" => Ok(Self::DBetaPeptideCGammaLinking),
            "d-gamma-peptide, c-delta linking" => Ok(Self::DGammaPeptideCDeltaLinking),
            "d-peptide cooh carboxy terminus" => Ok(Self::DPeptideCoohCarboxyTerminus),
            "d-peptide nh3 amino terminus" => Ok(Self::DPeptideNh3AminoTerminus),
            "d-peptide linking" => Ok(Self::DPeptideLinking),
            "d-saccharide" => Ok(Self::DSaccharide),
            "d-saccharide, alpha linking" => Ok(Self::DSaccharideAlphaLinking),
            "d-saccharide, beta linking" => Ok(Self::DSaccharideBetaLinking),
            "dna oh 3 prime terminus" => Ok(Self::DNAOh3PrimeTerminus),
            "dna oh 5 prime terminus" => Ok(Self::DNAOh5PrimeTerminus),
            "dna linking" => Ok(Self::DNALinking),
            "l-dna linking" => Ok(Self::LDNALinking),
            "l-rna linking" => Ok(Self::LRNALinking),
            "l-beta-peptide, c-gamma linking" => Ok(Self::LBetaPeptideCGammaLinking),
            "l-gamma-peptide, c-delta linking" => Ok(Self::LGammaPeptideCDeltaLinking),
            "l-peptide cooh carboxy terminus" => Ok(Self::LPeptideCoohCarboxyTerminus),
            "l-peptide nh3 amino terminus" => Ok(Self::LPeptideNh3AminoTerminus),
            "l-peptide linking" => Ok(Self::LPeptideLinking),
            "l-saccharide" => Ok(Self::LSaccharide),
            "l-saccharide, alpha linking" => Ok(Self::LSaccharideAlphaLinking),
            "l-saccharide, beta linking" => Ok(Self::LSaccharideBetaLinking),
            "rna oh 3 prime terminus" => Ok(Self::RNAOh3PrimeTerminus),
            "rna oh 5 prime terminus" => Ok(Self::RNAOh5PrimeTerminus),
            "rna linking" => Ok(Self::RNALinking),
            "non-polymer" => Ok(Self::NonPolymer),
            "other" => Ok(Self::Other),
            "peptide linking" => Ok(Self::PeptideLinking),
            "peptide-like" => Ok(Self::PeptideLike),
            "saccharide" => Ok(Self::Saccharide),
            s => Err(format!("{s} is not a known linking type")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Residue<'s> {
    /// Residue ID code
    ///
    /// `_chem_comp.id` in CCD
    id: &'s str,

    /// Chemical name
    ///
    /// `_chem_comp.name` in CCD
    name: &'s str,

    /// Three-character residue ID code
    ///
    /// `_chem_comp.type` in CCD
    linking_type: LinkingType,

    atoms: Atoms<'s>,
    bonds: Bonds<'s>,
}

impl<'s> TryFrom<&ParsedDataBlock<'s>> for Residue<'s> {
    type Error = String;

    fn try_from(value: &ParsedDataBlock<'s>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value
                .get("chem_comp.id")
                .ok_or("no id")?
                .get(0)
                .ok_or("no id")?,
            name: value
                .get("chem_comp.name")
                .ok_or("no name")?
                .get(0)
                .ok_or("no name")?,
            linking_type: LinkingType::from_str(
                value
                    .get("chem_comp.type")
                    .ok_or("no type")?
                    .get(0)
                    .ok_or("no type")?,
            )?,
            atoms: Atoms::try_from(value)?,
            bonds: Bonds::try_from(value)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum AtomStereo {
    R,
    S,
    None,
}

impl FromStr for AtomStereo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::R),
            "S" => Ok(Self::S),
            "N" => Ok(Self::None),
            s => Err(format!("AtomStereo should be R, S or N, not {s}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Atoms<'s> {
    atom_id: Vec<&'s str>,
    symbol: Vec<&'s str>,
    charge: Vec<Option<i32>>,
    aromatic: Vec<bool>,
    leaving: Vec<bool>,
    stereo: Vec<AtomStereo>,
    x: Vec<Option<f32>>,
    y: Vec<Option<f32>>,
    z: Vec<Option<f32>>,
}

impl<'s> TryFrom<&HashMap<&'s str, Vec<&'s str>>> for Atoms<'s> {
    type Error = String;

    fn try_from(value: &HashMap<&'s str, Vec<&'s str>>) -> Result<Self, Self::Error> {
        let (x, y, z) = get_xyz(value)?;
        Ok(Self {
            atom_id: value
                .get("chem_comp_atom.atom_id")
                .ok_or("no atom ids")?
                .clone(),
            symbol: value
                .get("chem_comp_atom.type_symbol")
                .ok_or("no element symbols")?
                .clone(),
            charge: value
                .get("chem_comp_atom.charge")
                .ok_or("no formal charges")?
                .iter()
                .cloned()
                .map(|s| match s {
                    "?" => Ok(None),
                    s => s.parse().map(Some),
                })
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("charge of {value:?} failed to parse: {e}"))?,
            aromatic: value
                .get("chem_comp_atom.pdbx_aromatic_flag")
                .ok_or("no atom aromatic flags")?
                .iter()
                .cloned()
                .map(try_as_bool)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("aromatic of {value:?} failed to parse: {e}"))?,
            leaving: value
                .get("chem_comp_atom.pdbx_leaving_atom_flag")
                .ok_or("no leaving flags")?
                .iter()
                .cloned()
                .map(try_as_bool)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("leaving of {value:?} failed to parse: {e}"))?,
            stereo: value
                .get("chem_comp_atom.pdbx_stereo_config")
                .ok_or("no atom stereo flags")?
                .iter()
                .map(|&s| s.parse())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("stereo of {value:?} failed to parse: {e}"))?,
            x,
            y,
            z,
        })
    }
}

fn get_coords_with_key(
    map: &HashMap<&str, Vec<&str>>,
    key: &str,
) -> Result<Vec<Option<f32>>, String> {
    Ok(map
        .get(key)
        .ok_or(format!("no values for {key}"))?
        .iter()
        .cloned()
        .map(|s| match s {
            "?" => Ok(None),
            s => s.parse().map(Some),
        })
        .collect::<Result<Vec<Option<f32>>, _>>()
        .map_err(|e| format!("{key} failed to parse: {e}"))?)
}

/// Get the coordinates from a CCD datablock
///
/// Chooses between `"chem_comp_atom.model_Cartn_{xyz}"` and
/// `"chem_comp_atom.pdbx_model_Cartn_{xyz}_ideal"` depending on which provides
/// the "better" set of values.
fn get_xyz(
    map: &HashMap<&str, Vec<&str>>,
) -> Result<(Vec<Option<f32>>, Vec<Option<f32>>, Vec<Option<f32>>), String> {
    let x1 = get_coords_with_key(map, "chem_comp_atom.model_Cartn_x");
    let x2 = get_coords_with_key(map, "chem_comp_atom.pdbx_model_Cartn_x_ideal");
    let y1 = get_coords_with_key(map, "chem_comp_atom.model_Cartn_y");
    let y2 = get_coords_with_key(map, "chem_comp_atom.pdbx_model_Cartn_y_ideal");
    let z1 = get_coords_with_key(map, "chem_comp_atom.model_Cartn_z");
    let z2 = get_coords_with_key(map, "chem_comp_atom.pdbx_model_Cartn_z_ideal");
    match (x1, y1, z1, x2, y2, z2) {
        (Ok(x1), Ok(y1), Ok(z1), Ok(x2), Ok(y2), Ok(z2)) => {
            let x1_count = x1.iter().filter(|o| o.is_some()).count();
            let x2_count = x2.iter().filter(|o| o.is_some()).count();
            let y1_count = y1.iter().filter(|o| o.is_some()).count();
            let y2_count = y2.iter().filter(|o| o.is_some()).count();
            let z1_count = z1.iter().filter(|o| o.is_some()).count();
            let z2_count = z2.iter().filter(|o| o.is_some()).count();
            if x1_count.min(y1_count).min(z1_count) > x2_count.min(y2_count).min(z2_count) {
                Ok((x1, y1, z1))
            } else {
                Ok((x2, y2, z2))
            }
        }
        (Ok(x), Ok(y), Ok(z), _, _, _) | (_, _, _, Ok(x), Ok(y), Ok(z)) => Ok((x, y, z)),
        (Err(_), Err(_), Err(_), Err(_), Err(_), Err(_)) => Err("No coordinates".into()),
        _ => Err("Coordinates have fewer than 3 dimensions".into()),
    }
}

fn try_as_bool(value: &str) -> Result<bool, String> {
    match value {
        "Y" => Ok(true),
        "N" => Ok(false),
        s => Err(format!("bool should be Y or N, not {s}")),
    }
}

#[derive(Debug, Clone, PartialEq)]
enum BondStereo {
    E,
    Z,
    None,
}

impl FromStr for BondStereo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "E" => Ok(Self::E),
            "Z" => Ok(Self::Z),
            "N" => Ok(Self::None),
            s => Err(format!("BondStereo should be E, Z or N, not {s}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum BondOrder {
    Single,
    Double,
    Triple,
}

impl FromStr for BondOrder {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SING" => Ok(Self::Single),
            "DOUB" => Ok(Self::Double),
            "TRIP" => Ok(Self::Triple),
            s => Err(format!("Unknown bond order {s}")),
        }
    }
}

impl Into<u8> for BondOrder {
    fn into(self) -> u8 {
        match self {
            Self::Single => 1,
            Self::Double => 2,
            Self::Triple => 3,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Bonds<'s> {
    atom1: Vec<&'s str>,
    atom2: Vec<&'s str>,
    order: Vec<BondOrder>,
    aromatic: Vec<bool>,
    stereo: Vec<BondStereo>,
}

impl<'s> TryFrom<&HashMap<&'s str, Vec<&'s str>>> for Bonds<'s> {
    type Error = String;

    fn try_from(value: &HashMap<&'s str, Vec<&'s str>>) -> Result<Self, Self::Error> {
        Ok(Self {
            atom1: value
                .get("chem_comp_bond.atom_id_1")
                .unwrap_or(&Vec::new())
                .clone(),
            atom2: value
                .get("chem_comp_bond.atom_id_2")
                .unwrap_or(&Vec::new())
                .clone(),
            order: value
                .get("chem_comp_bond.value_order")
                .unwrap_or(&Vec::new())
                .iter()
                .cloned()
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("bond order of {value:?} failed to parse: {e}"))?,
            aromatic: value
                .get("chem_comp_bond.pdbx_aromatic_flag")
                .unwrap_or(&Vec::new())
                .iter()
                .cloned()
                .map(try_as_bool)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("bond aromatic of {value:?} failed to parse: {e}"))?,
            stereo: value
                .get("chem_comp_bond.pdbx_stereo_config")
                .unwrap_or(&Vec::new())
                .iter()
                .cloned()
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("bond stereo of {value:?} failed to parse: {e}"))?,
        })
    }
}
