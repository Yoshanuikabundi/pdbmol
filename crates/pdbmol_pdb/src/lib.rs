use std::error::Error;

pub mod datatypes;
use datatypes::PdbRecord;

pub fn parse(s: &str) -> Result<Vec<PdbRecord>, Box<dyn Error>> {
    s.lines().map(PdbRecord::try_from).collect()
}
