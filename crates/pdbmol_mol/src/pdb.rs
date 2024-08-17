use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    ops::RangeInclusive,
};

use itertools::Itertools;
use pdbmol_pdb::datatypes::{AtomRecord, PdbParseErr, PdbRecord};
use pdbmol_types::{
    geom::lattice::{representations::CrystallographicUnitCell, UnitCell},
    ResidueDefinition,
};
use thiserror::Error;

type BondSet = pdbmol_types::BondSet<i32>;

#[derive(Error, Debug)]
pub enum MolFromPdbErr {
    #[error("serial {0} appears in two atom records")]
    DuplicateAtomSerial(i32),
    #[error("error encountered while parsing pdb: {0}")]
    PdbRecordParseError(#[from] PdbParseErr),
    #[error("pdb file has two CRYST1 records")]
    DuplicateUnitCellRecords,
    #[error("TER record does not correspond to immediately prior residue")]
    TerRecordMismatch,
    #[error("a residue in the PDB file could not be found in the residue database")]
    UnknownResidue,
}

#[derive(Debug, Clone)]
pub struct PdbAtom<'s> {
    pub record: AtomRecord<'s>,
    pub terminated: Option<i32>,
}

impl<'s> From<AtomRecord<'s>> for PdbAtom<'s> {
    fn from(value: AtomRecord<'s>) -> Self {
        Self { record: value, terminated: None }
    }
}

#[derive(Debug, Default)]
pub struct PdbTopology<'s> {
    atoms: BTreeMap<i32, PdbAtom<'s>>,
    bonds: BondSet,
    unit_cell: Option<CrystallographicUnitCell>,
}

use super::Molecule;

impl<S: Eq + Clone + Hash> PdbTopology<S> {
    /// Construct a `Molecule` for each residue in the PDB file.
    ///
    /// The constructed Molecule may include atoms that are not present in the
    /// PDB file.
    fn construct_expected_molecules(
        &self,
        residue_database: HashMap<S, ResidueDefinition<S>>,
    ) -> Result<Vec<Molecule>, MolFromPdbErr> {
        let mut molecules: BTreeMap<RangeInclusive<i32>, Molecule> = BTreeMap::new();
        let mut this_chain_id = ' ';
        let mut this_molecule: Molecule = Molecule::new();

        for (res_name, res_seq, i_code, chain_id, terminated) in self.residues() {
            if chain_id != this_chain_id {
                if !this_molecule.is_empty() {
                    molecules.push(this_molecule);
                }
                this_molecule = Molecule::new();
            }
            this_chain_id = chain_id;

            // Eventually, we want to try and put something together from CONECT
            // records and formal charges and possibly a user-provided list of
            // unnamed residues, but for now a residue that's not in the
            // database just raises an error.
            let residue = residue_database
                .get(&res_name)
                .ok_or(MolFromPdbErr::UnknownResidue)?;

            // Handle residues that do not link to their neighbours (eg water)
            if residue.does_not_link() {
                if !this_molecule.is_empty() {
                    molecules.push(this_molecule);
                }
                molecules.push(Molecule::from(residue));
                this_molecule = Molecule::new();
            } else {
                this_molecule.extend_with(residue.to_owned());
            }

            if terminated & !this_molecule.is_empty() {
                molecules.push(this_molecule);
                this_molecule = Molecule::new();
            }
        }

        Ok(molecules)
    }
}

impl<'s> PdbTopology<'s> {
    pub fn res_names<'a>(&'a self) -> impl Iterator<Item = Cow<'s, str>> + 'a {
        self.residues()
            .map(|(res_name, ..)| res_name)
    }

    /// Iterate over each residue by its identifiers.
    ///
    /// ```rust
    /// # use pdbmol_mol::pdb::PdbTopology
    /// # let pdbtopology = PdbTopology::default()
    /// for (res_name, res_seq, i_code, chain_id, terminated) in pdbtopology.residues {
    ///     println!(
    ///         "{res_name}#{res_seq}^{i_code}:{chain_id} is {}",
    ///         if terminated {"terminated"} else {"not terminated"}
    ///     )
    /// }
    pub fn residues<'a>(&'a self) -> impl Iterator<Item = (S, i32, char, char, bool)> + 'a {
        self.atoms
            .values()
            .map(|atom| {
                (
                    atom.record.res_name.clone(),
                    atom.record.res_seq,
                    atom.record.i_code,
                    atom.record.chain_id,
                    atom.terminated,
                )
            })
            .dedup()
    }
}

impl<S: Eq> PdbTopology<S> {
    pub fn from_pdb_data(
        pdb: impl IntoIterator<Item = Result<PdbRecord<S>, PdbParseErr>>
    ) -> Result<Self, MolFromPdbErr> {
        let mut atoms: BTreeMap<i32, PdbAtom<S>> = BTreeMap::new();
        let mut bonds = BondSet::new();
        let mut unit_cell = None;

        for record in pdb {
            match record? {
                PdbRecord::Atom(record) | PdbRecord::HetAtm(record) => {
                    let serial = record.serial;
                    if atoms
                        .insert(serial, record.into())
                        .is_some()
                    {
                        return Err(MolFromPdbErr::DuplicateAtomSerial(serial));
                    }
                }
                PdbRecord::Conect { parent: serial1, bonds: serial2s } => {
                    for serial2 in serial2s {
                        bonds.insert(serial1, serial2);
                    }
                }
                PdbRecord::Cryst1 { a, b, c, alpha, beta, gamma, .. } => {
                    if unit_cell.is_some() {
                        return Err(MolFromPdbErr::DuplicateUnitCellRecords);
                    }
                    unit_cell = CrystallographicUnitCell::from_lengths_and_angles_deg(
                        a, b, c, alpha, beta, gamma,
                    )
                    .ok()
                }
                PdbRecord::Ter { serial, res_name, chain_id, res_seq, i_code } => {
                    let mut no_terminated_residue = true;
                    for atom in atoms.values_mut().rev() {
                        if (atom.record.res_name == res_name)
                            & (atom.record.chain_id == chain_id)
                            & (atom.record.res_seq == res_seq)
                            & (atom.record.i_code == i_code)
                        {
                            atom.terminated = Some(serial);
                            no_terminated_residue = false;
                        } else {
                            break;
                        }
                    }
                    if no_terminated_residue {
                        return Err(MolFromPdbErr::TerRecordMismatch);
                    }
                }
                _ => continue,
            }
        }

        Ok(Self { atoms, bonds, unit_cell })
    }
}
