use std::collections::{BTreeMap, BTreeSet};

use pdbmol_pdb::datatypes::{AtomRecord, PdbParseErr, PdbRecord};
use pdbmol_types::geom::unitcell::{CrystallographicUnitCell, UnitCell};
use thiserror::Error;

/// Stores bonds as pairs of atom serial numbers
struct BondSet(BTreeSet<(i32, i32)>);

impl BondSet {
    pub fn insert(&mut self, a: i32, b: i32) {
        self.0.insert((a, b));
        self.0.insert((b, a));
    }

    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Iterate over all bonds as pairs with the lesser serial first.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (i32, i32)> + 'a {
        self.0.iter().filter(|(a, b)| a < b).copied()
    }

    /// Iterate over all bonds as pairs with the lesser serial first.
    pub fn into_iter(self) -> impl Iterator<Item = (i32, i32)> {
        self.0.into_iter().filter(|(a, b)| a < b)
    }

    /// Iterate over atom serial numbers bonded to the given serial number.
    pub fn bonded_to<'a>(&'a self, serial: i32) -> impl Iterator<Item = i32> + 'a {
        self.0
            .range((serial, i32::MIN)..=(serial, i32::MAX))
            .map(|(_, b)| *b)
    }
}

struct PdbTopology<S> {
    atoms: BTreeMap<i32, AtomRecord<S>>,
    bonds: BondSet,
    unit_cell: Option<CrystallographicUnitCell>,
}

#[derive(Error, Debug)]
enum MolFromPdbErr {
    #[error("serial {0} appears in two atom records")]
    DuplicateAtomSerial(i32),
}

impl<S> PdbTopology<S> {
    /// Read a `Molecule` representing many
    fn from_pdb_data(
        pdb: impl IntoIterator<Item = Result<PdbRecord<S>, PdbParseErr>>,
    ) -> Result<Self, MolFromPdbErr> {
        let mut atoms = BTreeMap::new();
        let mut bonds = BondSet::new();
        let mut unit_cell = None;

        for record in pdb {
            match record {
                Ok(PdbRecord::Atom(record) | PdbRecord::HetAtm(record)) => {
                    let serial = record.serial;
                    if atoms.insert(serial, record).is_some() {
                        return Err(MolFromPdbErr::DuplicateAtomSerial(serial));
                    }
                }
                Ok(PdbRecord::Conect {
                    parent: serial1,
                    bonds: serial2s,
                }) => {
                    for serial2 in serial2s {
                        bonds.insert(serial1, serial2);
                    }
                }
                Ok(PdbRecord::Cryst1 {
                    a,
                    b,
                    c,
                    alpha,
                    beta,
                    gamma,
                    ..
                }) => {
                    unit_cell = CrystallographicUnitCell::from_lengths_and_angles_deg(
                        a, b, c, alpha, beta, gamma,
                    )
                    .ok()
                }
                _ => unimplemented!(),
            }
        }

        Ok(Self {
            atoms,
            bonds,
            unit_cell,
        })
    }
}
