use bounded_static::IntoBoundedStatic;

pub mod parser;
// pub mod topology;

use parser::{PdbRecord, PdbRecordParseError};
// use topology::{PdbTopology, PdbTopologyError};

/// Load records from a file into memory
pub fn load_records_from_file(
    path: impl AsRef<std::path::Path>
) -> std::io::Result<Vec<Result<PdbRecord<'static>, PdbRecordParseError>>> {
    let contents = std::fs::read_to_string(path)?;
    Ok(load_records(&contents))
}

/// Stream records from a string
pub fn stream_records<'s>(
    s: &'s str
) -> impl Iterator<Item = Result<PdbRecord<'s>, PdbRecordParseError>> {
    s.lines().map(PdbRecord::try_from)
}

/// Load records from a string into a vector
pub fn load_records<'s>(s: &'s str) -> Vec<Result<PdbRecord<'static>, PdbRecordParseError>> {
    stream_records(s)
        .map(|result| result.map(IntoBoundedStatic::into_static))
        .collect()
}

// /// Load a PDB from a string into a PdbTopology object
// pub fn load<'s, 'd>(
//     s: &'s str,
//     residue_database: &HashMap<&'d str, ResidueDefinition<'d>>,
// ) -> Result<PdbTopology<'s, 'd>, PdbTopologyError> {
//     let records = stream_records(s);
//     PdbTopology::<'s, 'd>::from_records(records, residue_database)
// }

// /// Create a PdbTopology object from a file
// pub fn load_from_file<'d>(
//     path: impl AsRef<std::path::Path>,
//     residue_database: &HashMap<&'d str, ResidueDefinition<'d>>,
// ) -> Result<PdbTopology<'static, 'd>, PdbTopologyError> {
//     let contents = std::fs::read_to_string(path)?;
//     load(&contents, residue_database).map(IntoBoundedStatic::into_static)
// }
