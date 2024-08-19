use std::{borrow::Cow, fmt::Display, num::ParseIntError, str::FromStr};

use arrayvec::ArrayVec;
use bounded_static::{IntoBoundedStatic, ToBoundedStatic, ToStatic};
use pdbmol_types::Element;

use crate::parser::PdbRecordParseError;

#[derive(Clone, Debug, PartialEq)]
pub struct ConectBonds(ArrayVec<i32, 4>);

#[derive(Clone, Debug, PartialEq, ToStatic)]
pub struct VerbatimStr<'s>(pub Cow<'s, str>);

#[derive(Clone, Debug, PartialEq, ToStatic)]
pub struct PrecisionFloat<const P: usize>(f32);

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
        Ok(s.trim())
    }
}

impl<'s> ParseFromPdb<'s> for Cow<'s, str> {
    fn parse_from_pdb(s: &'s str) -> Result<Self, PdbRecordParseError> {
        Ok(Cow::Borrowed(s.trim()))
    }
}

impl<'s> ParseFromPdb<'s> for VerbatimStr<'s> {
    fn parse_from_pdb(s: &'s str) -> Result<Self, PdbRecordParseError> {
        Ok(VerbatimStr(Cow::Borrowed(s)))
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

impl<'s, const P: usize> ParseFromPdb<'s> for PrecisionFloat<P> {
    fn parse_from_pdb(s: &'s str) -> Result<Self, PdbRecordParseError> {
        Ok(Self(s.trim().parse()?))
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

pub trait WriteToPdb {
    fn write_to_pdb(
        self,
        width: usize,
    ) -> String;
}

impl WriteToPdb for &Option<char> {
    fn write_to_pdb(
        self,
        width: usize,
    ) -> String {
        if let Some(value) = self {
            format!("{value: >width$}")
        } else {
            format!("{: >width$}", ' ')
        }
    }
}

impl WriteToPdb for &VerbatimStr<'_> {
    fn write_to_pdb(
        self,
        _width: usize,
    ) -> String {
        self.0.to_string()
    }
}

impl<const P: usize> WriteToPdb for PrecisionFloat<P> {
    fn write_to_pdb(
        self,
        width: usize,
    ) -> String {
        format!("{: >width$.P$}", self.0)
    }
}

impl<const P: usize> WriteToPdb for &PrecisionFloat<P> {
    fn write_to_pdb(
        self,
        width: usize,
    ) -> String {
        format!("{: >width$.P$}", self.0)
    }
}

macro_rules! impl_write_from_display {
    ($($t:ty),*) => {
        $(
            impl WriteToPdb for $t {
                fn write_to_pdb(
                    self,
                    width: usize,
                ) -> String {
                    format!("{self: >width$}")
                }
            }

            impl WriteToPdb for &$t {
                fn write_to_pdb(
                    self,
                    width: usize,
                ) -> String {
                    format!("{self: >width$}")
                }
            }
        )*
    }
}

impl_write_from_display!(
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
    char,
    ConectBonds,
    &str,
    Element,
    Cow<'_, str>
);
