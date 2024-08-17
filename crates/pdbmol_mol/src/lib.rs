use std::collections::HashMap;

use pdbmol_types::{
    stereo::{AtomStereo, BondStereo},
    Element, ResidueDefinition,
};

pub mod pdb;

#[derive(Clone, Debug)]
pub enum MetadataValue {
    String(String),
    Float(f32),
    Int(i32),
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

type Metadata = HashMap<&'static str, MetadataValue>;

#[derive(Clone, Debug)]
pub struct Atom {
    element: Element,
    formal_charge: i8,
    metadata: Metadata,
    stereochemistry: AtomStereo,
    aromatic: Option<bool>,
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

    fn extend_with(
        &mut self,
        residue: &ResidueDefinition<'static>,
    ) -> Self {
        todo!()
    }
}

impl From<ResidueDefinition<'static>> for Molecule {
    fn from(value: ResidueDefinition<'static>) -> Self {
        Self::new().extend_with(&value)
    }
}
