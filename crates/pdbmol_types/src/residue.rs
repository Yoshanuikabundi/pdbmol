use std::borrow::Cow;

use bounded_static::ToStatic;

#[derive(ToStatic, Debug, Clone)]
pub struct LinkingBond<'s> {
    pub atom_name1: Cow<'s, str>,
    pub atom_name2: Cow<'s, str>,
    pub order: usize,
}

impl<'s> LinkingBond<'s> {
    pub fn new(
        atom_name1: &'s str,
        atom_name2: &'s str,
        order: usize,
    ) -> Self {
        Self {
            atom_name1: Cow::from(atom_name1),
            atom_name2: Cow::from(atom_name2),
            order,
        }
    }
}

#[derive(ToStatic, Debug, Clone)]
pub struct ResidueDefinition<'s> {
    pub id: Cow<'s, str>,
    pub linking_bonds: Vec<LinkingBond<'s>>,
}

impl<'s> ResidueDefinition<'s> {
    pub fn does_not_link(&self) -> bool {
        self.linking_bonds.is_empty()
    }
}
