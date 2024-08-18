use std::collections::{BTreeMap, HashMap};

use pdbmol_pdb::topology::{PdbAtom, PdbTopology};
use pdbmol_types::{
    residue::{BondDefinition, LinkingType},
    stereo::{AtomStereo, BondStereo},
    Element,
};

#[derive(Clone, Debug)]
pub enum MetadataValue {
    String(String),
    Float(f32),
    Int(i32),
    List(Vec<MetadataValue>),
}

impl From<String> for MetadataValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for MetadataValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<char> for MetadataValue {
    fn from(value: char) -> Self {
        Self::String(value.to_string())
    }
}

impl From<f32> for MetadataValue {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<i32> for MetadataValue {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl<I, T> From<I> for MetadataValue
where
    I: IntoIterator<Item = T>,
    T: Into<MetadataValue>,
{
    fn from(value: I) -> Self {
        Self::List(
            value
                .into_iter()
                .map(Into::into)
                .collect(),
        )
    }
}

type Metadata = HashMap<&'static str, MetadataValue>;

#[derive(Clone, Debug)]
pub struct Atom {
    name: String,
    element: Element,
    formal_charge: i8,
    metadata: Metadata,
    stereochemistry: Option<AtomStereo>,
    aromatic: Option<bool>,
}

impl From<PdbAtom<'_>> for Atom {
    fn from(value: PdbAtom<'_>) -> Self {
        let PdbAtom {
            name,
            element,
            charge,
            serial,
            alt_loc,
            xyz,
            leaving,
            stereo,
            aromatic,
        } = value;

        Self {
            name: name.into_owned(),
            element,
            formal_charge: charge,
            metadata: Metadata::from_iter([
                ("leaving", leaving.into()),
                ("alt_loc", alt_loc.into()),
                ("atom_serial", serial.into()),
                ("coords", xyz.into()),
            ]),
            stereochemistry: stereo,
            aromatic,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Bond {
    atom1: usize,
    atom2: usize,
    bond_order: u8,
    stereochemistry: BondStereo,
    aromatic: Option<bool>,
}

#[derive(Clone, Default, Debug)]
pub struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
    metadata: Metadata,
}

impl Molecule {
    fn new() -> Self {
        Self::default()
    }

    /// A molecule is empty iff it has no atoms.
    fn is_empty(&self) -> bool {
        self.atoms.is_empty()
    }
}

impl<'s, 'd> From<PdbTopology<'s, 'd>> for Molecule {
    fn from(value: PdbTopology) -> Self {
        let mut atoms = vec![];
        let mut bonds = vec![];
        let mut atom_index = 0;
        let mut prev_res_linkage: Option<LinkingType> = None;
        let mut prev_res_atoms = HashMap::new();

        for residue in value.residues {
            let mut this_res_atoms = HashMap::new();

            for atom in residue.atoms {
                let mut mol_atom: Atom = atom.into();
                mol_atom.metadata.extend([
                    ("residue_name", residue.res_name.as_ref().into()),
                    ("residue_number", residue.res_seq.into()),
                    ("insertion_code", residue.i_code.into()),
                    ("chain_id", residue.chain_id.into()),
                ]);

                this_res_atoms.insert(mol_atom.name.clone(), atom_index);
                atoms.push(mol_atom);
                atom_index += 1;
            }

            if let Some(db_res) = &residue.definition {
                if let Some(prev_linkage) = prev_res_linkage {
                    for BondDefinition { atom_name1, atom_name2, order, stereo, aromatic } in
                        prev_linkage.bonds(&db_res.linking_type)
                    {
                        bonds.push(Bond {
                            atom1: prev_res_atoms[atom_name1.as_ref()],
                            atom2: this_res_atoms[atom_name2.as_ref()],
                            bond_order: *order,
                            stereochemistry: *stereo,
                            aromatic: Some(*aromatic),
                        })
                    }
                }

                for BondDefinition { atom_name1, atom_name2, order, stereo, aromatic } in
                    &db_res.bonds
                {
                    bonds.push(Bond {
                        atom1: this_res_atoms[atom_name1.as_ref()],
                        atom2: this_res_atoms[atom_name2.as_ref()],
                        bond_order: *order,
                        stereochemistry: *stereo,
                        aromatic: Some(*aromatic),
                    })
                }
            };

            prev_res_atoms.clear();
            prev_res_atoms.extend(this_res_atoms);
            prev_res_linkage = residue
                .definition
                .map(|d| d.linking_type);
        }

        Molecule { atoms, bonds, metadata: Metadata::new() }
    }
}
