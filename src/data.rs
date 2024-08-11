use crate::parser::cif;
use once_cell::sync::Lazy;

pub static CCD: Lazy<Vec<(&str, Vec<cif::DataBlockItem>)>> = Lazy::new(|| {
    let ccd_str = include_str!("../data/ccd-20240406.cif");
    cif::parse(ccd_str).unwrap()
});
