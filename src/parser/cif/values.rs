mod numeric;
mod strings;
use numeric::Number;
pub use numeric::Numeric;
use strings::*;
use winnow::combinator::{alt, peek, preceded, terminated};
use winnow::prelude::*;

use super::charsets::nonblank1;
use super::whitespace_and_comments::whitespace;

// TODO: Split Numeric into uint/int/float here
// TODO: Store all values as strings and let the schema decode type
#[derive(Clone, Debug, PartialEq)]
pub enum Value<'a> {
    Numeric(Numeric),
    String(&'a str),
    Inapplicable,
    Unknown,
}

impl<'a> Value<'a> {
    /// Get the value if it is a string
    pub fn try_as_str(&self) -> Result<&'a str, &'static str> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err("Not a string"),
        }
    }

    /// Get the value if it is an integer
    pub fn try_as_int(&self) -> Result<numeric::Integer, &'static str> {
        match self {
            Self::Numeric(Numeric {
                value: Number::Int(i),
                esd: None,
            }) => Ok(*i),
            _ => Err("Not an int"),
        }
    }

    /// Get the value if it is an integer
    pub fn try_as_float(&self) -> Result<numeric::Float, &'static str> {
        match self {
            Self::Numeric(Numeric {
                value: Number::Float(i),
                esd: None,
            }) => Ok(*i),
            _ => Err("Not a float"),
        }
    }
}

fn numeric<'s>(input: &mut &'s str) -> PResult<Value<'s>> {
    Numeric::parser.map(Value::Numeric).parse_next(input)
}

fn eol_agnostic_value<'s>(input: &mut &'s str) -> PResult<Value<'s>> {
    alt((
        terminated('.', peek(whitespace)).map(|_| Value::Inapplicable),
        terminated('?', peek(whitespace)).map(|_| Value::Unknown),
        terminated(numeric, peek(whitespace)),
    ))
    .parse_next(input)
}

/// This parser must only be called immediately after an EOL
///
/// In contrast to the spec, Numeric, Inapplicable and Unknown values match only
/// when followed by whitespace.
pub fn eol_value<'s>(input: &mut &'s str) -> PResult<&'s str> {
    alt((eol_agnostic_value.take(), eol_string)).parse_next(input)
}

/// This parser must only be called immediately after a non-EOL character
///
/// In contrast to the spec, Numeric, Inapplicable and Unknown values match only
/// when followed by whitespace.
pub fn noteol_value<'s>(input: &mut &'s str) -> PResult<&'s str> {
    alt((eol_agnostic_value.take(), noteol_string)).parse_next(input)
}

/// This parser must only be called immediately after an EOL
///
/// In contrast to the spec, Numeric, Inapplicable and Unknown values match only
/// when followed by whitespace.
fn eol_value_parsed<'s>(input: &mut &'s str) -> PResult<Value<'s>> {
    alt((eol_agnostic_value, eol_string.map(Value::String))).parse_next(input)
}

/// This parser must only be called immediately after a non-EOL character
///
/// In contrast to the spec, Numeric, Inapplicable and Unknown values match only
/// when followed by whitespace.
fn noteol_value_parsed<'s>(input: &mut &'s str) -> PResult<Value<'s>> {
    alt((eol_agnostic_value, noteol_string.map(Value::String))).parse_next(input)
}

/// In contrast to the spec, Numeric, Inapplicable and Unknown values match only
/// when followed by whitespace.
pub fn whitespace_value<'s>(input: &mut &'s str) -> PResult<&'s str> {
    alt((
        preceded(whitespace.verify(|s: &str| s.ends_with('\n')), eol_value),
        preceded(whitespace, noteol_value),
    ))
    .parse_next(input)
}

/// In contrast to the spec, Numeric, Inapplicable and Unknown values match only
/// when followed by whitespace.
fn whitespace_value_parsed<'s>(input: &mut &'s str) -> PResult<Value<'s>> {
    alt((
        preceded(
            whitespace.verify(|s: &str| s.ends_with('\n')),
            eol_value_parsed,
        ),
        preceded(whitespace, noteol_value_parsed),
    ))
    .parse_next(input)
}

pub fn tag<'s>(input: &mut &'s str) -> PResult<&'s str> {
    preceded('_', nonblank1).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::cif::charsets::eol;
    use crate::parser::cif::values::numeric::Number;
    use winnow::combinator::preceded;

    #[test]
    fn test_eol_value() {
        let mut stream = "\n.";
        let output = preceded(eol, eol_value_parsed).parse_next(&mut stream);
        assert_eq!(output, Ok(Value::Inapplicable));

        let mut stream = "\n?";
        let output = preceded(eol, eol_value_parsed).parse_next(&mut stream);
        assert_eq!(output, Ok(Value::Unknown));

        let mut stream = "\n7.452323";
        let output = preceded(eol, eol_value_parsed).parse_next(&mut stream);
        assert_eq!(
            output,
            Ok(Value::Numeric(Numeric {
                value: Number::Float(7.452323),
                esd: None
            }))
        );

        let mut stream = "\n'hello there'";
        let output = preceded(eol, eol_value_parsed).parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String("hello there")));

        let mut stream = "\nUnquotedString";
        let output = preceded(eol, eol_value_parsed).parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String("UnquotedString")));

        let mut stream = "\n.UnquotedString";
        let output = preceded(eol, eol_value_parsed).parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String(".UnquotedString")));

        let mut stream = "\n?UnquotedString";
        let output = preceded(eol, eol_value_parsed).parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String("?UnquotedString")));
    }

    #[test]
    fn test_noteol_value() {
        let mut stream = " .";
        let output = preceded(whitespace, noteol_value_parsed).parse_next(&mut stream);
        assert_eq!(output, Ok(Value::Inapplicable));

        let mut stream = " ?";
        let output = preceded(whitespace, noteol_value_parsed).parse_next(&mut stream);
        assert_eq!(output, Ok(Value::Unknown));

        let mut stream = " 7.452323";
        let output = preceded(whitespace, noteol_value_parsed).parse_next(&mut stream);
        assert_eq!(
            output,
            Ok(Value::Numeric(Numeric {
                value: Number::Float(7.452323),
                esd: None
            }))
        );

        let mut stream = " 'hello there'";
        let output = preceded(whitespace, noteol_value_parsed).parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String("hello there")));

        let mut stream = " ;UnquotedString";
        let output = preceded(whitespace, noteol_value_parsed).parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String(";UnquotedString")));

        let mut stream = " UnquotedString";
        let output = preceded(whitespace, noteol_value_parsed).parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String("UnquotedString")));

        let mut stream = " .UnquotedString";
        let output = preceded(whitespace, noteol_value_parsed).parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String(".UnquotedString")));

        let mut stream = " ?UnquotedString";
        let output = preceded(whitespace, noteol_value_parsed).parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String("?UnquotedString")));
    }

    #[test]
    fn test_whitespace_value() {
        let mut stream = "\n.";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(output, Ok(Value::Inapplicable));

        let mut stream = "\n?";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(output, Ok(Value::Unknown));

        let mut stream = "\n7.452323";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(
            output,
            Ok(Value::Numeric(Numeric {
                value: Number::Float(7.452323),
                esd: None
            }))
        );

        let mut stream = "\n'hello there'";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String("hello there")));

        let mut stream = "\nUnquotedString";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String("UnquotedString")));

        let mut stream = "\n.UnquotedString";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String(".UnquotedString")));

        let mut stream = "\n?UnquotedString";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String("?UnquotedString")));

        let mut stream = " .";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(output, Ok(Value::Inapplicable));

        let mut stream = " ?";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(output, Ok(Value::Unknown));

        let mut stream = " 7.452323";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(
            output,
            Ok(Value::Numeric(Numeric {
                value: Number::Float(7.452323),
                esd: None
            }))
        );

        let mut stream = " 'hello there'";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String("hello there")));

        let mut stream = " UnquotedString";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String("UnquotedString")));

        let mut stream = " .UnquotedString";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String(".UnquotedString")));

        let mut stream = " ?UnquotedString";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String("?UnquotedString")));

        let mut stream = " ;UnquotedString";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String(";UnquotedString")));

        let mut stream = " 1999-07-08";
        let output = whitespace_value_parsed.parse_next(&mut stream);
        assert_eq!(output, Ok(Value::String("1999-07-08")));
    }
}
