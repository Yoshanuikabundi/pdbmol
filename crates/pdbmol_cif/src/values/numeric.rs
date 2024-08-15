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

pub type Integer = i32;

fn integer(input: &mut &str) -> PResult<Integer> {
    (opt(one_of(('+', '-'))), unsigned_integer)
        .take()
        .parse_to()
        .parse_next(input)
}

fn exponent<'s>(input: &mut &'s str) -> PResult<&'s str> {
    preceded(one_of(('e', 'E')), integer)
        .take()
        .parse_next(input)
}

pub type Float = f32;

fn float(input: &mut &str) -> PResult<Float> {
    alt((
        (integer, exponent).take(),
        (
            opt(one_of(('+', '-'))),
            alt(((digit0, '.', digit1).take(), (digit1, '.').take())),
            opt(exponent),
        )
            .take(),
    ))
    .take()
    .parse_to()
    .parse_next(input)
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Number {
    Int(Integer),
    Float(Float),
}

fn number(input: &mut &str) -> PResult<Number> {
    alt((float.map(Number::Float), integer.map(Number::Int))).parse_next(input)
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Numeric {
    pub value: Number,
    pub esd: Option<UnsignedInteger>,
}

impl Numeric {
    pub fn parser(input: &mut &str) -> PResult<Self> {
        alt((
            (number, '(', unsigned_integer, ')')
                .map(|(value, _, esd, _)| Self { value, esd: Some(esd) }),
            number.map(|value| Self { value, esd: None }),
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
        assert_eq!(float.parse("5e3"), Ok(5e3));

        assert_eq!(float.parse("5.0e3"), Ok(5e3));

        assert_eq!(float.parse(".3e-3"), Ok(0.3e-3));

        assert_eq!(float.parse("8.5"), Ok(8.5));

        assert!(float.parse(".e9").is_err());

        assert!(float.parse("564").is_err());
    }

    #[test]
    fn test_number() {
        assert_eq!(number.parse("5e3"), Ok(Number::Float(5e3)));

        assert_eq!(number.parse("5"), Ok(Number::Int(5)));

        assert_eq!(number.parse("-5"), Ok(Number::Int(-5)));

        assert_eq!(number.parse("+354"), Ok(Number::Int(354)));

        assert_eq!(
            number.parse("+342.63547e-23"),
            Ok(Number::Float(342.63547e-23))
        );

        assert!(number.parse("hello").is_err());
    }

    #[test]
    fn test_numeric() {
        assert_eq!(
            Numeric::parser.parse("5e3"),
            Ok(Numeric { value: Number::Float(5e3), esd: None })
        );

        assert_eq!(
            Numeric::parser.parse("5"),
            Ok(Numeric { value: Number::Int(5), esd: None })
        );

        assert_eq!(
            Numeric::parser.parse("5."),
            Ok(Numeric { value: Number::Float(5.), esd: None })
        );

        assert_eq!(
            Numeric::parser.parse("-5"),
            Ok(Numeric { value: Number::Int(-5), esd: None })
        );

        assert_eq!(
            Numeric::parser.parse("+354"),
            Ok(Numeric { value: Number::Int(354), esd: None })
        );

        assert_eq!(
            Numeric::parser.parse("+342.63547e-23"),
            Ok(Numeric { value: Number::Float(342.63547e-23), esd: None })
        );

        assert_eq!(
            Numeric::parser.parse("7(2)"),
            Ok(Numeric { value: Number::Int(7), esd: Some(2) })
        );

        assert_eq!(
            Numeric::parser.parse("+342.63547e-23(54)"),
            Ok(Numeric { value: Number::Float(342.63547e-23), esd: Some(54) })
        );

        assert!(Numeric::parser.parse("7(2.)").is_err());

        assert!(Numeric::parser.parse("hello").is_err());
    }
}
