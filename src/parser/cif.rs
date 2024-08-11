mod charsets;
mod reserved;
mod structure;
mod values;
mod whitespace_and_comments;

pub use structure::{cif, DataBlockItem};
pub use values::{Numeric, Value};
use whitespace_and_comments::{comments, whitespace};

use winnow::prelude::*;

pub fn parse(mut s: &str) -> PResult<Vec<(&str, Vec<DataBlockItem>)>> {
    cif.parse_next(&mut s)
}

// TODO: Implement https://www.iucr.org/resources/cif/spec/version1.1/cifsyntax#restrictions
// TODO: Implement https://www.iucr.org/resources/cif/spec/version1.1/semantics

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_ccd() {
        let ccd = std::fs::read_to_string("data/ccd-20240406.cif").unwrap();
        parse(&ccd).unwrap();
    }
}
