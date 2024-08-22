use std::{borrow::Cow, fmt::Display, num::ParseIntError, str::FromStr};

use arrayvec::ArrayVec;
use bounded_static::{IntoBoundedStatic, ToBoundedStatic};
use pdbmol_types::Element;

use crate::parser::PdbRecordParseError;

#[derive(Clone, Debug, PartialEq)]
pub struct ConectBonds(ArrayVec<i32, 4>);

impl IntoBoundedStatic for ConectBonds {
    type Static = ConectBonds;

    fn into_static(self) -> Self::Static {
        self
    }
}

impl ToBoundedStatic for ConectBonds {
    type Static = ConectBonds;

    fn to_static(&self) -> Self::Static {
        self.clone()
    }
}

impl FromStr for ConectBonds {
    type Err = ParseIntError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut bonds = ArrayVec::new();
        let mut bond;
        loop {
            (bond, s) = if s.len() <= 5 { (s, "") } else { s.split_at(5) };

            bonds.push(bond.trim().parse()?);

            if s.is_empty() {
                break;
            }
        }
        Ok(Self(bonds))
    }
}

impl Display for ConectBonds {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for bond in &self.0 {
            write!(f, "{bond: >5}")?;
        }
        Ok(())
    }
}

pub trait ParseFromPdb<'s> {
    fn parse_from_pdb(s: &'s str) -> Result<Self, PdbRecordParseError>
    where
        Self: Sized;
}

impl<'s> ParseFromPdb<'s> for &'s str {
    fn parse_from_pdb(s: &'s str) -> Result<Self, PdbRecordParseError> {
        Ok(s)
    }
}

impl<'s> ParseFromPdb<'s> for Cow<'s, str> {
    fn parse_from_pdb(s: &'s str) -> Result<Self, PdbRecordParseError> {
        Ok(Cow::Borrowed(s))
    }
}

impl<'s, T: ParseFromPdb<'s>> ParseFromPdb<'s> for Option<T> {
    fn parse_from_pdb(s: &'s str) -> Result<Self, PdbRecordParseError>
    where
        Self: Sized,
    {
        if s.trim().is_empty() {
            Ok(None)
        } else {
            Some(T::parse_from_pdb(s)).transpose()
        }
    }
}

impl<'s> ParseFromPdb<'s> for Element {
    fn parse_from_pdb(s: &'s str) -> Result<Self, PdbRecordParseError> {
        Element::from_uncased_symbol(s.trim())
            .ok_or(PdbRecordParseError::UnknownElement(s.to_string()))
    }
}

macro_rules! impl_parse_from_fromstr {
    ($($t:ty),*) => {
        $(
            impl<'s> ParseFromPdb<'s> for $t {
                fn parse_from_pdb(s: &'s str) -> Result<Self, PdbRecordParseError> {
                    Ok(s.trim().parse()?)
                }
            }
        )*
    }
}

impl_parse_from_fromstr!(
    usize,
    u8,
    u16,
    u32,
    u64,
    u128,
    isize,
    i8,
    i16,
    i32,
    i64,
    i128,
    f32,
    f64,
    char,
    ConectBonds
);
