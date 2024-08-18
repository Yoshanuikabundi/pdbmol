use std::{borrow::Cow, collections::HashMap};

use bounded_static::ToStatic;

use crate::{
    stereo::{AtomStereo, BondStereo},
    Element,
};

#[derive(ToStatic, Debug, Clone)]
pub struct BondDefinition<'s> {
    pub atom_name1: Cow<'s, str>,
    pub atom_name2: Cow<'s, str>,
    pub order: usize,
    pub stereo: BondStereo,
    pub aromatic: bool,
}

impl<'s> BondDefinition<'s> {
    pub fn new(
        atom_name1: &'s str,
        atom_name2: &'s str,
        order: usize,
        stereo: BondStereo,
        aromatic: bool,
    ) -> Self {
        Self {
            atom_name1: Cow::from(atom_name1),
            atom_name2: Cow::from(atom_name2),
            order,
            stereo,
            aromatic,
        }
    }
}

#[derive(ToStatic, Debug, Clone)]
pub struct AtomDefinition {
    pub element: Element,
    pub charge: i8,
    pub leaving: bool,
    pub stereo: AtomStereo,
    pub aromatic: bool,
}

#[derive(ToStatic, Debug, Clone)]
pub struct ResidueDefinition<'s> {
    pub id: Cow<'s, str>,
    /// Bonds between this
    pub linking_bonds: Vec<BondDefinition<'s>>,
    pub atoms: HashMap<Cow<'s, str>, AtomDefinition>,
    pub bonds: Vec<BondDefinition<'s>>,
}

impl<'s> ResidueDefinition<'s> {
    pub fn does_not_link(&self) -> bool {
        self.linking_bonds.is_empty()
    }
}
