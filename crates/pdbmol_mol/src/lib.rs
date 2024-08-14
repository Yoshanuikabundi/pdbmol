use std::collections::HashMap;

use pdbmol_ccdparse::datatypes::Residue;
use pdbmol_pdb::datatypes::{AtomRecord, PdbParseErr, PdbRecord};
use pdbmol_types::{stereo::AtomStereo, stereo::BondStereo, Element};

mod pdb;

enum MetadataValue {
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

struct Atom {
    element: Element,
    formal_charge: i8,
    metadata: Metadata,
    stereochemistry: AtomStereo,
    aromatic: Option<bool>,
}

struct Bond {
    atom1: usize,
    atom2: usize,
    bond_order: u8,
    stereochemistry: BondStereo,
    aromatic: Option<bool>,
}

struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
    metadata: Metadata,
}
