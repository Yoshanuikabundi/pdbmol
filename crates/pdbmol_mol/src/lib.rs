use std::collections::HashMap;

use pdbmol_types::{stereo::AtomStereo, stereo::BondStereo, Element};

enum MetadataValue {
    String(String),
    Float(f32),
    Int(i32),
}

struct Atom {
    element: Element,
    formal_charge: i8,
    metadata: HashMap<&'static str, MetadataValue>,
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
}
