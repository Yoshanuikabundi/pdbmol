//! https://mmcif.wwpdb.org/dictionaries/mmcif_pdbx_v50.dic/Categories/chem_comp.html

use std::collections::HashMap;

use crate::parser::cif::{DataBlockItem, Value};

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

impl TryFrom<&str> for LinkingType {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
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
            s => Err(format!("{s} is not a string")),
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

impl<'s> TryFrom<&Vec<DataBlockItem<'s>>> for Residue<'s> {
    type Error = String;

    fn try_from(value: &Vec<DataBlockItem<'s>>) -> Result<Self, Self::Error> {
        let atom_table = match value.get(1) {
            Some(DataBlockItem::Table(atom_table)) => atom_table,
            Some(_) => Err("second data block item isn't atom table")?,
            None => Err("residue definition has only one data block item")?,
        };
        let atoms = Atoms::try_from(atom_table)?;

        let bond_table = match value.get(2) {
            Some(DataBlockItem::Table(bond_table)) => bond_table,
            Some(_) => Err("third data block item isn't bond table")?,
            None => Err("residue definition has only two data block item")?,
        };
        let bonds = Bonds::try_from(bond_table)?;

        match value.get(0) {
            Some(DataBlockItem::DataItems(hash_map)) => Ok(Self {
                id: hash_map.get("chem_comp.id").ok_or("no id")?.try_as_str()?,
                name: hash_map
                    .get("chem_comp.name")
                    .ok_or("no name")?
                    .try_as_str()?,
                linking_type: LinkingType::try_from(
                    hash_map
                        .get("chem_comp.type")
                        .ok_or("no type")?
                        .try_as_str()?,
                )?,
                atoms,
                bonds,
            }),
            Some(d) => Err(format!("first data block item {d:#?} isn't chem_comp")),
            None => Err("empty residue definition".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum AtomStereo {
    R,
    S,
    None,
}

impl<'s> TryFrom<&Value<'s>> for AtomStereo {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value.try_as_str() {
            Ok("R") => Ok(Self::R),
            Ok("S") => Ok(Self::S),
            Ok("N") => Ok(Self::None),
            Ok(s) => Err(format!("AtomStereo should be R, S or N, not {s}")),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Atoms<'s> {
    atom_id: Vec<&'s str>,
    symbol: Vec<&'s str>,
    charge: Vec<i32>,
    aromatic: Vec<bool>,
    leaving: Vec<bool>,
    stereo: Vec<AtomStereo>,
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
}

impl<'s> TryFrom<&HashMap<&'s str, Vec<Value<'s>>>> for Atoms<'s> {
    type Error = String;

    fn try_from(value: &HashMap<&'s str, Vec<Value<'s>>>) -> Result<Self, Self::Error> {
        Ok(Self {
            atom_id: value
                .get("chem_comp_atom.atom_id")
                .ok_or("no atom ids")?
                .iter()
                .map(Value::try_as_str)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("id of {value:?} shat the bed: {e}"))?,
            symbol: value
                .get("chem_comp_atom.type_symbol")
                .ok_or("no element symbols")?
                .iter()
                .map(Value::try_as_str)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("symbol of {value:?} shat the bed: {e}"))?,
            charge: value
                .get("chem_comp_atom.charge")
                .ok_or("no formal charges")?
                .iter()
                .map(Value::try_as_int)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("charge of {value:?} shat the bed: {e}"))?,
            aromatic: value
                .get("chem_comp_atom.pdbx_aromatic_flag")
                .ok_or("no atom aromatic flags")?
                .iter()
                .map(try_as_bool)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("aromatic of {value:?} shat the bed: {e}"))?,
            leaving: value
                .get("chem_comp_atom.pdbx_leaving_atom_flag")
                .ok_or("no leaving flags")?
                .iter()
                .map(try_as_bool)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("leaving of {value:?} shat the bed: {e}"))?,
            stereo: value
                .get("chem_comp_atom.pdbx_stereo_config")
                .ok_or("no atom stereo flags")?
                .iter()
                .map(AtomStereo::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("stereo of {value:?} shat the bed: {e}"))?,
            x: value
                .get("chem_comp_atom.pdbx_model_Cartn_x_ideal")
                .ok_or("no ideal x values")?
                .iter()
                .map(Value::try_as_float)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("x of {value:?} shat the bed: {e}"))?,
            y: value
                .get("chem_comp_atom.pdbx_model_Cartn_y_ideal")
                .ok_or("no ideal y values")?
                .iter()
                .map(Value::try_as_float)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("y of {value:?} shat the bed: {e}"))?,
            z: value
                .get("chem_comp_atom.pdbx_model_Cartn_z_ideal")
                .ok_or("no ideal z values")?
                .iter()
                .map(Value::try_as_float)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("z of {value:?} shat the bed: {e}"))?,
        })
    }
}

fn try_as_bool(value: &Value) -> Result<bool, String> {
    match value.try_as_str() {
        Ok("Y") => Ok(true),
        Ok("N") => Ok(false),
        Ok(s) => Err(format!("bool should be Y or N, not {s}")),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Debug, Clone, PartialEq)]
enum BondStereo {
    E,
    Z,
    None,
}

impl<'s> TryFrom<&Value<'s>> for BondStereo {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value.try_as_str() {
            Ok("E") => Ok(Self::E),
            Ok("Z") => Ok(Self::Z),
            Ok("N") => Ok(Self::None),
            Ok(s) => Err(format!("BondStereo should be E, Z or N, not {s}")),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum BondOrder {
    Single,
    Double,
    Triple,
    Quadruple,
}

impl<'s> TryFrom<&Value<'s>> for BondOrder {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value.try_as_str() {
            Ok("SING") => Ok(Self::Single),
            Ok("DOUB") => Ok(Self::Double),
            Ok("TRIP") => Ok(Self::Triple),
            Ok("QUAD") => Ok(Self::Quadruple),
            Ok(s) => Err(format!("Unknown bond order {s}")),
            Err(e) => Err(e.to_string()),
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

impl<'s> TryFrom<&HashMap<&'s str, Vec<Value<'s>>>> for Bonds<'s> {
    type Error = String;

    fn try_from(value: &HashMap<&'s str, Vec<Value<'s>>>) -> Result<Self, Self::Error> {
        Ok(Self {
            atom1: value
                .get("chem_comp_bond.atom_id_1")
                .ok_or("no atom1 ids")?
                .iter()
                .map(Value::try_as_str)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("bond atom1 of {value:?} shat the bed: {e}"))?,
            atom2: value
                .get("chem_comp_bond.atom_id_2")
                .ok_or("no atom2 ids")?
                .iter()
                .map(Value::try_as_str)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("bond atom2 of {value:?} shat the bed: {e}"))?,
            order: value
                .get("chem_comp_bond.value_order")
                .ok_or("no bond orders")?
                .iter()
                .map(BondOrder::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("bond order of {value:?} shat the bed: {e}"))?,
            aromatic: value
                .get("chem_comp_bond.pdbx_aromatic_flag")
                .ok_or("no bond aromatic flags")?
                .iter()
                .map(try_as_bool)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("bond aromatic of {value:?} shat the bed: {e}"))?,
            stereo: value
                .get("chem_comp_bond.pdbx_stereo_config")
                .ok_or("no bond stereo flags")?
                .iter()
                .map(BondStereo::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("bond stereo of {value:?} shat the bed: {e}"))?,
        })
    }
}
