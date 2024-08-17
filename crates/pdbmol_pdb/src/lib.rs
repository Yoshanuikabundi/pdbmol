mod datatypes;
use bounded_static::IntoBoundedStatic;
use datatypes::PdbRecordParser;
pub use datatypes::{AtomRecord, PdbParseErr, PdbRecord};

pub fn load_from_file(
    path: impl AsRef<std::path::Path>
) -> std::io::Result<Vec<Result<PdbRecord<'static>, PdbParseErr>>> {
    let contents = std::fs::read_to_string(path)?;
    Ok(load(&contents))
}

pub fn parse<'s>(s: &'s str) -> impl Iterator<Item = Result<PdbRecord<'s>, PdbParseErr>> {
    PdbRecordParser::from_str(s)
}

pub fn load<'s>(s: &'s str) -> Vec<Result<PdbRecord<'static>, PdbParseErr>> {
    parse(s)
        .map(|result| result.map(IntoBoundedStatic::into_static))
        .collect()
}
