use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;
use pdbmol_pdb::{AtomRecord, PdbParseErr, PdbRecord};
use pdbmol_types::{
    geom::lattice::{representations::CrystallographicUnitCell, UnitCell},
    Element, ResidueDefinition,
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

impl<'s> PdbTopology<'s> {
    /// Construct a `Molecule` for each residue in the PDB file.
    ///
    /// The constructed Molecule may include atoms that are not present in the
    /// PDB file.
    fn construct_expected_molecules(
        &self,
        residue_database: HashMap<&'s str, ResidueDefinition<'s>>,
    ) -> Result<Vec<Molecule>, MolFromPdbErr> {
        let mut this_chain_id = ' ';
        let mut molecules: Vec<Molecule> = vec![Molecule::new()];

        fn start_new_molecule(molecules: &mut Vec<Molecule>) -> &mut Molecule {
            if molecules.is_empty() | !molecules.last().unwrap().is_empty() {
                molecules.push(Molecule::new());
            }
            molecules.last_mut().unwrap()
        }

        let mut this_molecule: &mut Molecule = start_new_molecule(&mut molecules);

        for (res_name, _, _, chain_id, terminated) in self.residues() {
            if chain_id != this_chain_id {
                this_molecule = start_new_molecule(&mut molecules);
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
                this_molecule = start_new_molecule(&mut molecules);
            } else {
                this_molecule.extend_with(residue);
            }

            if terminated {
                this_molecule = start_new_molecule(&mut molecules);
            }
        }

        Ok(molecules)
    }
}

impl<'s> PdbTopology<'s> {
    /// Iterate over each residue by its identifiers.
    ///
    /// ```rust
    /// # use pdbmol_mol::pdb::PdbTopology
    /// # let pdbtopology = PdbTopology::default()
    /// for (res_name, res_seq, i_code, chain_id, terminated) in pdbtopology.residues() {
    ///     println!(
    ///         "{res_name}#{res_seq}^{i_code}:{chain_id} is {}",
    ///         if terminated {"terminated"} else {"not terminated"}
    ///     )
    /// }
    pub fn residues<'a: 's>(
        &'a self
    ) -> impl Iterator<Item = (&'s str, i32, char, char, bool)> + 'a {
        self.atoms
            .values()
            .map(|atom| {
                (
                    atom.record.res_name.as_ref(),
                    atom.record.res_seq,
                    atom.record.i_code,
                    atom.record.chain_id,
                    atom.terminated,
                )
            })
            .dedup()
            .map(|(res_name, res_seq, i_code, chain_id, ter)| {
                (res_name, res_seq, i_code, chain_id, ter.is_some())
            })
    }

    /// Iterate over each atom by its identifiers.
    ///
    /// ```rust
    /// # use pdbmol_mol::pdb::PdbTopology
    /// # let pdbtopology = PdbTopology::default()
    /// for (
    ///     atom_name,
    ///     atom_seq,
    ///     res_name,
    ///     res_seq,
    ///     i_code,
    ///     chain_id,
    ///     terminated,
    ///     element,
    ///     charge,
    ///     xyz
    /// ) in pdbtopology.atoms() {
    ///     # "
    ///     ...
    ///     # "
    /// }
    pub fn atoms<'a: 's>(
        &'a self
    ) -> impl Iterator<
        Item = (
            &'s str,
            i32,
            &'s str,
            i32,
            char,
            char,
            Option<i32>,
            Element,
            i8,
            [f32; 3],
        ),
    > + 'a {
        self.atoms.values().map(|atom| {
            (
                atom.record.name.as_ref(),
                atom.record.serial,
                atom.record.res_name.as_ref(),
                atom.record.res_seq,
                atom.record.i_code,
                atom.record.chain_id,
                atom.terminated,
                atom.record.element,
                atom.record.charge,
                [atom.record.x, atom.record.y, atom.record.z],
            )
        })
    }
}

impl<'s> PdbTopology<'s> {
    pub fn from_pdb_data(
        pdb: impl IntoIterator<Item = Result<PdbRecord<'s>, PdbParseErr>>
    ) -> Result<Self, MolFromPdbErr> {
        let mut atoms: BTreeMap<i32, PdbAtom<'s>> = BTreeMap::new();
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
