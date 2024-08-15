use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

use itertools::Itertools;
use pdbmol_pdb::datatypes::{AtomRecord, PdbParseErr, PdbRecord};
use pdbmol_types::{
    geom::unitcell::{CrystallographicUnitCell, UnitCell},
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
pub struct PdbAtom<S> {
    pub record: AtomRecord<S>,
    pub terminated: Option<i32>,
}

impl<S> From<AtomRecord<S>> for PdbAtom<S> {
    fn from(value: AtomRecord<S>) -> Self {
        Self {
            record: value,
            terminated: None,
        }
    }
}

pub struct PdbTopology<S> {
    atoms: BTreeMap<i32, PdbAtom<S>>,
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
        let mut current_chain = ' ';
        let mut chains: Vec<Molecule> = Vec::new();
        let mut molecule: Molecule = Molecule::new();

        for (res_name, chain_id, terminated) in self.residues_with_chain_and_ter() {
            if chain_id != current_chain {
                if !molecule.is_empty() {
                    chains.push(molecule);
                }
                molecule = Molecule::new();
            }
            current_chain = chain_id;

            // Eventually, we want to try and put something together from CONECT
            // records and formal charges and possibly a user-provided list of
            // unnamed residues, but for now a residue that's not in the
            // database just raises an error.
            let residue = residue_database
                .get(&res_name)
                .ok_or(MolFromPdbErr::UnknownResidue)?;

            // Handle residues that do not link to their neighbours (eg water)
            if residue.does_not_link() {
                if !molecule.is_empty() {
                    chains.push(molecule);
                }
                chains.push(Molecule::from(residue));
                molecule = Molecule::new();
            } else {
                molecule.extend_with(residue);
            }

            if terminated & !molecule.is_empty() {
                chains.push(molecule);
                molecule = Molecule::new();
            }
        }

        Ok(chains)
    }
}

impl<S: Eq + Clone> PdbTopology<S> {
    pub fn res_names<'a>(&'a self) -> impl Iterator<Item = S> + 'a {
        self.residues_with_chain_and_ter()
            .map(|(res_name, _, _)| res_name)
    }

    /// Iterate over the names of each residue with each residue's chain ID and terminated record.
    pub fn residues_with_chain_and_ter<'a>(&'a self) -> impl Iterator<Item = (S, char, bool)> + 'a {
        self.atoms
            .values()
            .map(|atom| {
                (
                    atom.record.chain_id,
                    atom.record.res_seq,
                    atom.record.res_name.clone(),
                    atom.record.i_code,
                    atom.terminated,
                )
            })
            .dedup()
            .map(|(chain_id, _res_seq, res_name, _i_code, ter)| (res_name, chain_id, ter.is_some()))
    }
}

impl<S: Eq> PdbTopology<S> {
    pub fn from_pdb_data(
        pdb: impl IntoIterator<Item = Result<PdbRecord<S>, PdbParseErr>>,
    ) -> Result<Self, MolFromPdbErr> {
        let mut atoms: BTreeMap<i32, PdbAtom<S>> = BTreeMap::new();
        let mut bonds = BondSet::new();
        let mut unit_cell = None;

        for record in pdb {
            match record? {
                PdbRecord::Atom(record) | PdbRecord::HetAtm(record) => {
                    let serial = record.serial;
                    if atoms.insert(serial, record.into()).is_some() {
                        return Err(MolFromPdbErr::DuplicateAtomSerial(serial));
                    }
                }
                PdbRecord::Conect {
                    parent: serial1,
                    bonds: serial2s,
                } => {
                    for serial2 in serial2s {
                        bonds.insert(serial1, serial2);
                    }
                }
                PdbRecord::Cryst1 {
                    a,
                    b,
                    c,
                    alpha,
                    beta,
                    gamma,
                    ..
                } => {
                    if unit_cell.is_some() {
                        return Err(MolFromPdbErr::DuplicateUnitCellRecords);
                    }
                    unit_cell = CrystallographicUnitCell::from_lengths_and_angles_deg(
                        a, b, c, alpha, beta, gamma,
                    )
                    .ok()
                }
                PdbRecord::Ter {
                    serial,
                    res_name,
                    chain_id,
                    res_seq,
                    i_code,
                } => {
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

        Ok(Self {
            atoms,
            bonds,
            unit_cell,
        })
    }
}
