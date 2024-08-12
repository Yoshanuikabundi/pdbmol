mod charsets;
mod reserved;
mod structure;
mod values;
mod whitespace_and_comments;

use std::collections::HashMap;

pub use structure::{cif, DataBlockItem};
pub use values::{Numeric, Value};
use whitespace_and_comments::{comments, whitespace};

use winnow::{
    error::{ContextError, ParseError, ParserError},
    prelude::*,
};

pub type ParsedDataBlock<'s> = HashMap<&'s str, Vec<&'s str>>;
pub type ParsedCif<'s> = HashMap<&'s str, ParsedDataBlock<'s>>;

pub fn parse(mut s: &str) -> Result<ParsedCif, ParseError<&str, ContextError>> {
    Ok(cif
        .parse(&mut s)?
        .into_iter()
        .map(|(datablock_heading, datablock)| {
            (
                datablock_heading,
                datablock
                    .into_iter()
                    .map(
                        |datablockitem| -> Box<dyn Iterator<Item = (&str, Vec<&str>)>> {
                            match datablockitem {
                                DataBlockItem::DataItems(map) => {
                                    Box::new(map.into_iter().map(|(key, value)| (key, vec![value])))
                                }
                                DataBlockItem::Table(map) => Box::new(map.into_iter()),
                                DataBlockItem::SaveFrame(_) => unimplemented!(),
                            }
                        },
                    )
                    .flatten()
                    .collect(),
            )
        })
        .collect())
}

// TODO: Implement https://www.iucr.org/resources/cif/spec/version1.1/cifsyntax#restrictions
// TODO: Implement https://www.iucr.org/resources/cif/spec/version1.1/semantics

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn load_ccd() {
    //     let ccd = std::fs::read_to_string("data/ccd-20240406.cif").unwrap();
    //     parse(&ccd).unwrap();
    // }
}
