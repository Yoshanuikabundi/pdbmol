use std::{borrow::Cow, collections::HashMap};

use super::{PdbRecord, PdbRecordParseError};
use bounded_static::{IntoBoundedStatic, ToBoundedStatic, ToStatic};
use itertools::Itertools;
use pdbmol_types::{
    geom::lattice::{representations::CrystallographicUnitCell, UnitCell},
    residue::BondDefinition,
    stereo::AtomStereo,
    Element, ResidueDefinition,
};
use thiserror::Error;

type BondSet = pdbmol_types::BondSet<i32>;

#[derive(Error, Debug)]
pub enum PdbTopologyError {
    #[error("serial {0} appears in two atom records")]
    DuplicateAtomSerial(i32),
    #[error("error encountered while parsing pdb records: {0}")]
    PdbRecordParseError(#[from] PdbRecordParseError),
    #[error("pdb file has two CRYST1 records")]
    DuplicateUnitCellRecords,
    #[error("TER record appeared before first ATOM or HETATM record")]
    TerBeforeAtoms,
    #[error("atom name {atom_name} could not be found in residue definition {res_name}")]
    AtomMissingFromResidueDefinition { atom_name: String, res_name: String },
    #[error("atom name {name} found twice in residue {res_name}#{res_seq}")]
    DuplicateAtomNameInResidue {
        name: String,
        res_name: String,
        res_seq: i32,
    },
    #[error(
        "atom {name} in residue {res_name}#{res_seq} should be {}, not {}",
        expected.symbol(),
        found.symbol()
    )]
    AtomElementMismatch {
        name: String,
        res_name: String,
        res_seq: i32,
        expected: Element,
        found: Element,
    },
    #[error(
        "atom {name} in residue {res_name}#{res_seq} should have charge {expected}, not {found}"
    )]
    AtomChargeMismatch {
        name: String,
        res_name: String,
        res_seq: i32,
        expected: i8,
        found: i8,
    },
    #[error("error opening PDB file: {0}")]
    FileOpenError(#[from] std::io::Error),
}

#[derive(Debug, ToStatic)]
pub struct PdbAtom<'s> {
    pub name: Cow<'s, str>,
    pub element: Element,
    pub charge: i8,
    /// Empty if atom is in residue DB but not PDB file
    pub serial: Vec<i32>,
    /// Empty if atom is in residue DB but not PDB file
    pub alt_loc: Vec<char>,
    /// Empty if atom is in residue DB but not PDB file
    pub xyz: Vec<[f32; 3]>,
    /// `None` represents an atom from an unknown ligand
    pub leaving: Option<bool>,
    /// `None` represents an atom from an unknown ligand
    pub stereo: Option<AtomStereo>,
    /// `None` represents an atom from an unknown ligand
    pub aromatic: Option<bool>,
}

#[derive(Debug, ToStatic)]
pub struct PdbResidue<'s, 'd> {
    pub atoms: Vec<PdbAtom<'s>>,
    pub chain_id: char,
    pub res_name: Cow<'s, str>,
    pub res_seq: i32,
    pub i_code: char,
    pub terminated: bool,
    /// `None` represents an unknown residue
    pub definition: Option<ResidueDefinition<'d>>,
}

impl PdbResidue<'_, '_> {
    fn in_same_chain(
        &self,
        second: &Self,
    ) -> bool {
        !(self.terminated | (self.chain_id != second.chain_id))
    }
}

#[derive(Debug, Default)]
pub struct PdbTopology<'s, 'd> {
    pub residues: Vec<PdbResidue<'s, 'd>>,
    pub conects: BondSet,
    pub unit_cell: Option<CrystallographicUnitCell>,
}

impl<'s, 'd> ToBoundedStatic for PdbTopology<'s, 'd> {
    type Static = PdbTopology<'static, 'static>;

    fn to_static(&self) -> Self::Static {
        let Self { residues, conects: bonds, unit_cell } = self;

        PdbTopology {
            residues: residues.to_static(),
            conects: bonds.clone(),
            unit_cell: *unit_cell,
        }
    }
}

impl<'s, 'd> IntoBoundedStatic for PdbTopology<'s, 'd> {
    type Static = PdbTopology<'static, 'static>;

    fn into_static(self) -> Self::Static {
        let Self { residues, conects: bonds, unit_cell } = self;

        PdbTopology {
            residues: residues.into_static(),
            conects: bonds,
            unit_cell,
        }
    }
}

impl<'s, 'd> PdbTopology<'s, 'd> {
    pub fn from_records(
        records: impl IntoIterator<Item = Result<PdbRecord<'s>, PdbRecordParseError>>,
        residue_database: &HashMap<&'d str, ResidueDefinition<'d>>,
    ) -> Result<Self, PdbTopologyError> {
        let mut topology = Self::default();
        topology.load_atoms_from_records(records, residue_database)?;
        Ok(topology)
    }

    fn add_bond(
        &mut self,
        a: i32,
        b: i32,
    ) {
        self.conects.insert(a, b);
    }

    fn add_atom(
        &mut self,
        record: AtomRecord<'s>,
        residue_database: &HashMap<&'d str, ResidueDefinition<'d>>,
    ) -> Result<(), PdbTopologyError> {
        let AtomRecord {
            serial,
            name,
            alt_loc,
            res_name,
            chain_id,
            res_seq,
            i_code,
            x,
            y,
            z,
            element,
            charge,
            ..
        } = record;

        let residue = match self.residues.last_mut() {
            Some(residue)
                if (residue.res_name == res_name)
                    & (residue.chain_id == chain_id)
                    & (residue.res_seq == res_seq)
                    & (residue.i_code == i_code)
                    & !residue.terminated =>
            {
                residue
            }
            _ => {
                let residue_definition = residue_database.get(res_name.as_ref());
                self.start_next_residue(
                    chain_id,
                    res_name.clone(),
                    res_seq,
                    i_code,
                    residue_definition.cloned(),
                )
            }
        };

        if let Some(db_residue) = &residue.definition {
            let db_atom = db_residue
                .atoms
                .get(&name)
                .ok_or_else(|| PdbTopologyError::AtomMissingFromResidueDefinition {
                    atom_name: name.to_string(),
                    res_name: res_name.to_string(),
                })?;

            if let Some(duplicate_atom) = residue
                .atoms
                .iter_mut()
                .find(|record| record.name == name)
            {
                if duplicate_atom
                    .alt_loc
                    .contains(&alt_loc)
                {
                    Err(PdbTopologyError::DuplicateAtomNameInResidue {
                        name: name.into_owned(),
                        res_name: res_name.into_owned(),
                        res_seq,
                    })
                } else {
                    duplicate_atom.alt_loc.push(alt_loc);
                    duplicate_atom.xyz.push([x, y, z]);
                    Ok(())
                }
            } else if db_atom.element != element {
                Err(PdbTopologyError::AtomElementMismatch {
                    name: name.into_owned(),
                    res_name: res_name.into_owned(),
                    res_seq,
                    expected: db_atom.element,
                    found: element,
                })
            } else if db_atom.charge != charge {
                Err(PdbTopologyError::AtomChargeMismatch {
                    name: name.into_owned(),
                    res_name: res_name.into_owned(),
                    res_seq,
                    expected: db_atom.charge,
                    found: charge,
                })
            } else {
                residue.atoms.push(PdbAtom {
                    name,
                    element,
                    charge,
                    serial: vec![serial],
                    alt_loc: vec![alt_loc],
                    xyz: vec![[x, y, z]],
                    leaving: Some(db_atom.leaving),
                    stereo: Some(db_atom.stereo),
                    aromatic: Some(db_atom.aromatic),
                });

                Ok(())
            }
        } else {
            residue.atoms.push(PdbAtom {
                name,
                element,
                charge,
                serial: vec![serial],
                alt_loc: vec![alt_loc],
                xyz: vec![[x, y, z]],
                leaving: None,
                stereo: None,
                aromatic: None,
            });

            Ok(())
        }
    }

    fn terminate_residue(&mut self) -> Result<(), PdbTopologyError> {
        self.residues
            .last_mut()
            .ok_or(PdbTopologyError::TerBeforeAtoms)?
            .terminated = true;
        Ok(())
    }

    pub fn atoms(&self) -> impl Iterator<Item = &PdbAtom<'s>> {
        self.residues
            .iter()
            .flat_map(|residue| &residue.atoms)
    }

    pub fn atoms_with_residues<'a>(
        &'a self
    ) -> impl Iterator<Item = (&'a PdbResidue<'s, 'd>, &'a PdbAtom<'s>)> {
        self.residues
            .iter()
            .flat_map(|residue| {
                residue
                    .atoms
                    .iter()
                    .map(|atom| {
                        let residue: &'a PdbResidue<'s, 'd> = residue;
                        let atom: &'a PdbAtom<'s> = atom;
                        (residue, atom)
                    })
                    .inspect(|_| ())
            })
    }

    fn start_next_residue(
        &mut self,
        chain_id: char,
        res_name: Cow<'s, str>,
        res_seq: i32,
        i_code: char,
        definition: Option<ResidueDefinition<'d>>,
    ) -> &mut PdbResidue<'s, 'd> {
        self.complete_last_residue();

        let residue = PdbResidue {
            atoms: Vec::new(),
            chain_id,
            res_name,
            res_seq,
            i_code,
            terminated: false,
            definition,
        };

        self.residues.push(residue);
        self.residues.last_mut().unwrap()
    }

    /// Add any atoms from the residue definition that aren't yet in the residue
    /// and then add bonds
    fn complete_last_residue(&mut self) {
        if let Some(last_residue) = self.residues.last_mut() {
            if let Some(last_residue_def) = &last_residue.definition {
                let last_residue_atom_names = last_residue
                    .atoms
                    .iter()
                    .map(|record| record.name.clone())
                    .collect_vec();

                for (db_atom_name, db_atom) in &last_residue_def.atoms {
                    if !last_residue_atom_names.contains(db_atom_name) {
                        last_residue.atoms.push(PdbAtom {
                            serial: vec![],
                            name: db_atom_name.to_static(),
                            alt_loc: vec![],
                            element: db_atom.element,
                            charge: db_atom.charge,
                            xyz: vec![],
                            leaving: Some(db_atom.leaving),
                            stereo: Some(db_atom.stereo),
                            aromatic: Some(db_atom.aromatic),
                        })
                    }
                }
            }
        }
        self.link_last_two_residues();
    }

    /// Removes leaving atoms from the linkage between the last two residues
    fn link_last_two_residues<'a>(&'a mut self) {
        if let [.., prev_res, last_res] = &mut self.residues[..] {
            if !prev_res.in_same_chain(last_res) {
                return;
            }

            if let (Some(db_prev_res), Some(db_last_res)) =
                (&prev_res.definition, &last_res.definition)
            {
                for BondDefinition { atom_name1, atom_name2, .. } in db_prev_res
                    .linking_type
                    .bonds(&db_last_res.linking_type)
                {
                    let leaving_atoms = db_prev_res.linked_leaving_atoms(atom_name1);
                    prev_res
                        .atoms
                        .retain(|PdbAtom { name, .. }| !leaving_atoms.contains(name.as_ref()));

                    let leaving_atoms = db_last_res.linked_leaving_atoms(atom_name2);
                    last_res
                        .atoms
                        .retain(|PdbAtom { name, .. }| !leaving_atoms.contains(name.as_ref()));
                }
            }
        }
    }

    fn load_atoms_from_records(
        &mut self,
        records: impl IntoIterator<Item = Result<PdbRecord<'s>, PdbRecordParseError>>,
        residue_database: &HashMap<&'d str, ResidueDefinition<'d>>,
    ) -> Result<(), PdbTopologyError> {
        for record in records {
            match record? {
                PdbRecord::Atom(record) | PdbRecord::HetAtm(record) => {
                    self.add_atom(record, residue_database)?
                }
                PdbRecord::Conect { parent: serial1, bonds: serial2s } => {
                    for serial2 in serial2s {
                        self.add_bond(serial1, serial2);
                    }
                }
                PdbRecord::Cryst1 { a, b, c, alpha, beta, gamma, .. } => {
                    if self.unit_cell.is_some() {
                        return Err(PdbTopologyError::DuplicateUnitCellRecords);
                    }
                    self.unit_cell = CrystallographicUnitCell::from_lengths_and_angles_deg(
                        a, b, c, alpha, beta, gamma,
                    )
                    .ok()
                }
                PdbRecord::Link | PdbRecord::SsBond { .. } => todo!(),
                PdbRecord::Ter { .. } => self.terminate_residue()?,
                _ => continue,
            }
        }
        self.complete_last_residue();
        Ok(())
    }
}
