use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    str::FromStr,
};

use bounded_static::{IntoBoundedStatic, ToBoundedStatic, ToStatic};

use crate::{
    stereo::{AtomStereo, BondStereo},
    Element,
};

#[derive(ToStatic, Debug, Clone, PartialEq)]
pub struct BondDefinition<'s> {
    pub atom_name1: Cow<'s, str>,
    pub atom_name2: Cow<'s, str>,
    pub order: u8,
    pub stereo: BondStereo,
    pub aromatic: bool,
}

impl<'s> BondDefinition<'s> {
    pub const fn new(
        atom_name1: &'s str,
        atom_name2: &'s str,
        order: u8,
        stereo: BondStereo,
        aromatic: bool,
    ) -> Self {
        Self {
            atom_name1: Cow::Borrowed(atom_name1),
            atom_name2: Cow::Borrowed(atom_name2),
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
    /// Bonds between this residue and the next
    pub linking_type: LinkingType,
    pub atoms: HashMap<Cow<'s, str>, AtomDefinition>,
    pub bonds: Vec<BondDefinition<'s>>,
}

impl<'s> ResidueDefinition<'s> {
    fn leaving_atoms(&self) -> impl Iterator<Item = &str> {
        self.atoms
            .iter()
            .filter_map(|(name, atom)| atom.leaving.then_some(name.as_ref()))
    }

    fn linked_leaving_bonds<'a, 'b: 'a>(
        &'a self,
        center: &'b str,
    ) -> impl Iterator<Item = [&'a str; 2]> {
        let leavers: HashSet<_> = self.leaving_atoms().collect();

        self.bonds
            .iter()
            .filter_map(move |BondDefinition { atom_name1, atom_name2, .. }| {
                let atom2 = atom_name2.as_ref();
                let atom1 = atom_name1.as_ref();
                (((atom1 == center) & leavers.contains(atom2))
                    | ((atom2 == center) & leavers.contains(atom1))
                    | (leavers.contains(atom1) & leavers.contains(atom2)))
                .then_some([atom1, atom2])
            })
    }

    pub fn linked_leaving_atoms(
        &self,
        center: &str,
    ) -> HashSet<String> {
        let bonds: Vec<_> = self
            .linked_leaving_bonds(center)
            .collect();
        let mut linked = HashSet::new();
        let mut prev_linked_len = 0;
        loop {
            for [atom1, atom2] in &bonds {
                if linked.contains(*atom1) | (*atom1 == center) {
                    linked.insert(atom2.to_string());
                }
                if linked.contains(*atom2) | (*atom2 == center) {
                    linked.insert(atom1.to_string());
                }
            }
            if prev_linked_len == linked.len() {
                break;
            }
            prev_linked_len = linked.len();
        }
        linked
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum LinkingType {
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
    Other(&'static [BondDefinition<'static>]),
    PeptideLinking,
    PeptideLike,
    Saccharide,
}

impl IntoBoundedStatic for LinkingType {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }
}

impl ToBoundedStatic for LinkingType {
    type Static = Self;

    fn to_static(&self) -> Self::Static {
        *self
    }
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
            "other" => Ok(Self::Other(&[])),
            "peptide linking" => Ok(Self::PeptideLinking),
            "peptide-like" => Ok(Self::PeptideLike),
            "saccharide" => Ok(Self::Saccharide),
            s => Err(format!("{s} is not a known linking type")),
        }
    }
}

const PEPTIDE_BOND: &[BondDefinition<'static>] =
    &[BondDefinition::new("C", "N", 1, BondStereo::None, false)];

impl LinkingType {
    pub fn bonds(
        &self,
        other: &Self,
    ) -> &'static [BondDefinition<'static>] {
        use LinkingType::*;
        match (self, other) {
            (Other(lbonds), Other(rbonds)) if lbonds == rbonds => lbonds,
            (NonPolymer, _) | (_, NonPolymer) => &[],
            (
                DPeptideNh3AminoTerminus
                | DPeptideLinking
                | LPeptideNh3AminoTerminus
                | LPeptideLinking,
                DPeptideCoohCarboxyTerminus
                | DPeptideLinking
                | LPeptideCoohCarboxyTerminus
                | LPeptideLinking,
            ) => PEPTIDE_BOND,
            _ => &[],
        }
    }
}
