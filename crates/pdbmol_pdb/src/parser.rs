use std::{
    char::ParseCharError,
    num::{ParseFloatError, ParseIntError},
};
use thiserror::Error;

mod records;
mod types;
pub use records::PdbRecord;
pub use types::ConectBonds;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PdbRecordParseError {
    #[error("record type {0} is unknown")]
    UnknownRecordType(String),
    #[error("record expected, found empty line")]
    EmptyLine,
    #[error("expected a {expected} record, found {found}")]
    UnexpectedRecord {
        expected: &'static str,
        found: String,
    },
    #[error("SEQRES record declared {expected} residues, but contained {found}")]
    SeqresResnameCountMismatch { expected: usize, found: usize },
    #[error("encountered unexpected end of file")]
    UnexpectedEof,
    #[error("couldn't parse int: {0}")]
    CouldNotParseInt(#[from] ParseIntError),
    #[error("couldn't parse float: {0}")]
    CouldNotParseFloat(#[from] ParseFloatError),
    #[error("couldn't parse char: {0}")]
    CouldNotParseChar(#[from] ParseCharError),
    #[error("couldn't parse charge: {0}")]
    CouldNotParseCharge(String),
    #[error("Line {0:?} too short to include essential data")]
    LineTooShort(String),
    #[error("Element symbol {0} is unknown")]
    UnknownElement(String),
}
