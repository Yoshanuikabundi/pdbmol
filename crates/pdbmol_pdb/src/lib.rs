pub mod datatypes;
use datatypes::{PdbParseErr, PdbRecord, PdbRecordParser};

pub fn load(
    path: impl AsRef<std::path::Path>,
) -> std::io::Result<Vec<Result<PdbRecord, PdbParseErr>>> {
    let contents = std::fs::read_to_string(path)?;
    Ok(parse(&contents))
}

pub fn parse(s: &str) -> Vec<Result<PdbRecord, PdbParseErr>> {
    PdbRecordParser::from_str(s)
        .map(|result| result.map(|record| record.into()).map_err(Into::into))
        .collect()
}
