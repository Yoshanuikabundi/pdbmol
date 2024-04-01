use winnow::{
    ascii::{digit0, digit1},
    combinator::{alt, opt, preceded},
    prelude::*,
    token::one_of,
};

type UnsignedInteger = u32;

fn unsigned_integer(input: &mut &str) -> PResult<UnsignedInteger> {
    digit1.parse_to().parse_next(input)
}

type Integer = i32;

fn integer(input: &mut &str) -> PResult<Integer> {
    (opt(one_of(('+', '-'))), unsigned_integer)
        .recognize()
        .parse_to()
        .parse_next(input)
}

fn exponent<'s>(input: &mut &'s str) -> PResult<&'s str> {
    preceded(one_of(('e', 'E')), integer)
        .recognize()
        .parse_next(input)
}

type Float = f64;

fn float(input: &mut &str) -> PResult<Float> {
    alt((
        (integer, exponent).recognize(),
        (
            opt(one_of(('+', '-'))),
            alt(((digit0, '.', digit1).recognize(), (digit1, '.').recognize())),
            opt(exponent),
        )
            .recognize(),
    ))
    .recognize()
    .parse_to()
    .parse_next(input)
}

// TODO: Retain uint/int typing distinction
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Number {
    Int(Integer),
    Float(Float),
}

fn number(input: &mut &str) -> PResult<Number> {
    alt((integer.map(Number::Int), float.map(Number::Float))).parse_next(input)
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Numeric {
    pub value: Number,
    pub esd: Option<UnsignedInteger>,
}

impl Numeric {
    pub fn parser(input: &mut &str) -> PResult<Self> {
        alt((
            number.map(|value| Self { value, esd: None }),
            (number, '(', unsigned_integer, ')').map(|(value, _, esd, _)| Self {
                value,
                esd: Some(esd),
            }),
        ))
        .parse_next(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned_integer() {
        let mut stream = "1345-hello34 3";

        let output = unsigned_integer.parse_next(&mut stream);
        assert_eq!(stream, "-hello34 3");
        assert_eq!(output, Ok(1345));

        let output = unsigned_integer.parse_next(&mut stream);
        assert!(output.is_err());

        let mut stream = "98 6";
        let output = unsigned_integer.parse_next(&mut stream);
        assert_eq!(stream, " 6");
        assert_eq!(output, Ok(98));

        let mut stream = "-12";
        let output = unsigned_integer.parse_next(&mut stream);
        assert!(output.is_err());

        let mut stream = "e12";
        let output = unsigned_integer.parse_next(&mut stream);
        assert!(output.is_err());
    }

    #[test]
    fn test_integer() {
        let mut stream = "1345-25hi";

        let output = integer.parse_next(&mut stream);
        assert_eq!(stream, "-25hi");
        assert_eq!(output, Ok(1345));

        let output = integer.parse_next(&mut stream);
        assert_eq!(stream, "hi");
        assert_eq!(output, Ok(-25));

        let output = integer.parse_next(&mut stream);
        assert!(output.is_err());

        let mut stream = "98 6";
        let output = integer.parse_next(&mut stream);
        assert_eq!(stream, " 6");
        assert_eq!(output, Ok(98));

        let mut stream = "+12";
        let output = integer.parse_next(&mut stream);
        assert_eq!(output, Ok(12));
    }

    #[test]
    fn test_exponent() {
        let mut stream = "e34-";
        let output = exponent.parse_next(&mut stream);
        assert_eq!(stream, "-");
        assert_eq!(output, Ok("e34"));

        let mut stream = "E+87 ";
        let output = exponent.parse_next(&mut stream);
        assert_eq!(stream, " ");
        assert_eq!(output, Ok("E+87"));

        let mut stream = "e-2465634 ";
        let output = exponent.parse_next(&mut stream);
        assert_eq!(stream, " ");
        assert_eq!(output, Ok("e-2465634"));
    }

    #[test]
    fn test_float() {
        let mut stream = "5e3";
        let output = float.parse_next(&mut stream);
        assert_eq!(stream, "");
        assert_eq!(output, Ok(5e3));

        let mut stream = "5.0e3";
        let output = float.parse_next(&mut stream);
        assert_eq!(stream, "");
        assert_eq!(output, Ok(5e3));

        let mut stream = ".3e-3";
        let output = float.parse_next(&mut stream);
        assert_eq!(stream, "");
        assert_eq!(output, Ok(0.3e-3));

        let mut stream = "8.5";
        let output = float.parse_next(&mut stream);
        assert_eq!(stream, "");
        assert_eq!(output, Ok(8.5));

        let mut stream = ".e9";
        let output = float.parse_next(&mut stream);
        assert!(output.is_err());

        let mut stream = "564";
        let output = float.parse_next(&mut stream);
        assert!(output.is_err());
    }
}
