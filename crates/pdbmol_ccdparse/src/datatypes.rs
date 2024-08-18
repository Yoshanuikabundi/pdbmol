//! <https://mmcif.wwpdb.org/dictionaries/mmcif_pdbx_v50.dic/Categories/chem_comp.html>

use std::{borrow::Cow, collections::HashMap, fmt::Debug, str::FromStr};

use pdbmol_cif::ParsedDataBlock;
use pdbmol_types::{
    residue::{AtomDefinition, BondDefinition, LinkingType},
    stereo::{AtomStereo, BondStereo},
    Element, ResidueDefinition,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CcdResidue<'s> {
    /// Residue ID code
    ///
    /// `_chem_comp.id` in CCD
    pub id: &'s str,

    /// Chemical name
    ///
    /// `_chem_comp.name` in CCD
    pub name: &'s str,

    /// Three-character residue ID code
    ///
    /// `_chem_comp.type` in CCD
    pub linking_type: LinkingType,

    pub atoms: Atoms<'s>,
    pub bonds: Bonds<'s>,
}

impl<'s> From<CcdResidue<'s>> for ResidueDefinition<'s> {
    fn from(value: CcdResidue<'s>) -> Self {
        let CcdResidue { id, linking_type, atoms, bonds, .. } = value;
        Self {
            id: Cow::from(id),
            linking_type,
            atoms: atoms.into(),
            bonds: bonds.into(),
        }
    }
}

impl<'s> From<Atoms<'s>> for HashMap<Cow<'s, str>, AtomDefinition> {
    fn from(value: Atoms<'s>) -> Self {
        let Atoms {
            symbol, charge, leaving, aromatic, stereo, atom_id, ..
        } = value;
        symbol
            .into_iter()
            .map(Element::from_uncased_symbol)
            .map(Option::unwrap)
            .zip(charge.into_iter().map(Option::unwrap))
            .zip(leaving)
            .zip(aromatic)
            .zip(stereo)
            .map(
                |((((element, charge), leaving), aromatic), stereo)| AtomDefinition {
                    element,
                    charge: charge.try_into().unwrap(),
                    leaving,
                    stereo,
                    aromatic,
                },
            )
            .zip(atom_id)
            .map(|(def, name)| (Cow::from(name), def))
            .collect()
    }
}

impl<'s> From<Bonds<'s>> for Vec<BondDefinition<'s>> {
    fn from(value: Bonds<'s>) -> Self {
        let Bonds { atom1, atom2, order, aromatic, stereo } = value;
        atom1
            .into_iter()
            .zip(atom2)
            .zip(order)
            .zip(aromatic)
            .zip(stereo)
            .map(
                |((((atom1, atom2), order), aromatic), stereo)| BondDefinition {
                    atom_name1: Cow::from(atom1),
                    atom_name2: Cow::from(atom2),
                    order: match order {
                        BondOrder::Single => 1,
                        BondOrder::Double => 2,
                        BondOrder::Triple => 3,
                    },
                    stereo,
                    aromatic,
                },
            )
            .collect()
    }
}

impl<'s> TryFrom<&ParsedDataBlock<'s>> for CcdResidue<'s> {
    type Error = String;

    fn try_from(value: &ParsedDataBlock<'s>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value
                .get("chem_comp.id")
                .ok_or("no id")?
                .first()
                .ok_or("no id")?,
            name: value
                .get("chem_comp.name")
                .ok_or("no name")?
                .first()
                .ok_or("no name")?,
            linking_type: LinkingType::from_str(
                value
                    .get("chem_comp.type")
                    .ok_or("no type")?
                    .first()
                    .ok_or("no type")?,
            )?,
            atoms: Atoms::try_from(value)?,
            bonds: Bonds::try_from(value)?,
        })
    }
}

#[derive(Clone, PartialEq)]
pub struct Atoms<'s> {
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

impl<'s> Debug for Atoms<'s> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if f.alternate() {
            writeln!(f, "Atoms {{")?;
            writeln!(f, "    atom_id: {:?},", self.atom_id)?;
            writeln!(f, "    symbol: {:?},", self.symbol)?;
            writeln!(f, "    charge: {:?},", self.charge)?;
            writeln!(f, "    aromatic: {:?},", self.aromatic)?;
            writeln!(f, "    leaving: {:?},", self.leaving)?;
            writeln!(f, "    stereo: {:?},", self.stereo)?;
            writeln!(f, "    x: {:?},", self.x)?;
            writeln!(f, "    y: {:?},", self.y)?;
            writeln!(f, "    z: {:?},", self.z)?;
            writeln!(f, "}}")?;
        } else {
            write!(f, "Atoms {{")?;
            write!(f, " atom_id: {:?},", self.atom_id)?;
            write!(f, " symbol: {:?},", self.symbol)?;
            write!(f, " charge: {:?},", self.charge)?;
            write!(f, " aromatic: {:?},", self.aromatic)?;
            write!(f, " leaving: {:?},", self.leaving)?;
            write!(f, " stereo: {:?},", self.stereo)?;
            write!(f, " x: {:?},", self.x)?;
            write!(f, " y: {:?},", self.y)?;
            write!(f, " z: {:?},", self.z)?;
            write!(f, "}}")?;
        }
        Ok(())
    }
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
    map.get(key)
        .ok_or(format!("no values for {key}"))?
        .iter()
        .cloned()
        .map(|s| match s {
            "?" => Ok(None),
            s => s.parse().map(Some),
        })
        .collect::<Result<Vec<Option<f32>>, _>>()
        .map_err(|e| format!("{key} failed to parse: {e}"))
}

/// Get the coordinates from a CCD datablock
///
/// Chooses between `"chem_comp_atom.model_Cartn_{xyz}"` and
/// `"chem_comp_atom.pdbx_model_Cartn_{xyz}_ideal"` depending on which provides
/// the "better" set of values.
fn get_xyz(
    map: &HashMap<&str, Vec<&str>>
) -> Result<(Vec<Option<f32>>, Vec<Option<f32>>, Vec<Option<f32>>), String> {
    let x1 = get_coords_with_key(map, "chem_comp_atom.model_Cartn_x");
    let x2 = get_coords_with_key(map, "chem_comp_atom.pdbx_model_Cartn_x_ideal");
    let y1 = get_coords_with_key(map, "chem_comp_atom.model_Cartn_y");
    let y2 = get_coords_with_key(map, "chem_comp_atom.pdbx_model_Cartn_y_ideal");
    let z1 = get_coords_with_key(map, "chem_comp_atom.model_Cartn_z");
    let z2 = get_coords_with_key(map, "chem_comp_atom.pdbx_model_Cartn_z_ideal");
    match (x1, y1, z1, x2, y2, z2) {
        (Ok(x1), Ok(y1), Ok(z1), Ok(x2), Ok(y2), Ok(z2)) => {
            let x1_count = x1
                .iter()
                .filter(|o| o.is_some())
                .count();
            let x2_count = x2
                .iter()
                .filter(|o| o.is_some())
                .count();
            let y1_count = y1
                .iter()
                .filter(|o| o.is_some())
                .count();
            let y2_count = y2
                .iter()
                .filter(|o| o.is_some())
                .count();
            let z1_count = z1
                .iter()
                .filter(|o| o.is_some())
                .count();
            let z2_count = z2
                .iter()
                .filter(|o| o.is_some())
                .count();
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
pub enum BondOrder {
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

impl From<BondOrder> for u8 {
    fn from(value: BondOrder) -> Self {
        match value {
            BondOrder::Single => 1,
            BondOrder::Double => 2,
            BondOrder::Triple => 3,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Bonds<'s> {
    atom1: Vec<&'s str>,
    atom2: Vec<&'s str>,
    order: Vec<BondOrder>,
    aromatic: Vec<bool>,
    stereo: Vec<BondStereo>,
}

impl<'s> Debug for Bonds<'s> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if f.alternate() {
            writeln!(f, "Bonds {{")?;
            writeln!(f, "    atom1: {:?},", self.atom1)?;
            writeln!(f, "    atom2: {:?},", self.atom2)?;
            writeln!(f, "    order: {:?},", self.order)?;
            writeln!(f, "    aromatic: {:?},", self.aromatic)?;
            writeln!(f, "    stereo: {:?},", self.stereo)?;
            write!(f, "}}")?;
        } else {
            write!(f, "Bonds {{")?;
            write!(f, " atom1: {:?},", self.atom1)?;
            write!(f, " atom2: {:?},", self.atom2)?;
            write!(f, " order: {:?},", self.order)?;
            write!(f, " aromatic: {:?},", self.aromatic)?;
            write!(f, " stereo: {:?},", self.stereo)?;
            write!(f, "}}")?;
        }
        Ok(())
    }
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
