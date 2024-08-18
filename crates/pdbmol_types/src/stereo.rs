use std::str::FromStr;

use bounded_static::ToStatic;

#[derive(Debug, Clone, PartialEq, ToStatic, Copy)]
pub enum AtomStereo {
    R,
    S,
    None,
}

impl FromStr for AtomStereo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::R),
            "S" => Ok(Self::S),
            "N" => Ok(Self::None),
            s => Err(format!("AtomStereo should be R, S or N, not {s}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, ToStatic, Copy)]
pub enum BondStereo {
    E,
    Z,
    None,
}

impl FromStr for BondStereo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "E" => Ok(Self::E),
            "Z" => Ok(Self::Z),
            "N" => Ok(Self::None),
            s => Err(format!("BondStereo should be E, Z or N, not {s}")),
        }
    }
}
