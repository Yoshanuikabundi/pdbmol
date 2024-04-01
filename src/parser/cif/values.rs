mod numeric;
mod strings;
pub use numeric::Numeric;
use strings::*;
use winnow::combinator::alt;
use winnow::prelude::*;

use super::charsets::nonblank1;
use super::whitespace_and_comments::whitespace;

// TODO: Split Numeric into uint/int/float here
#[derive(Clone, Debug)]
pub enum Value<'a> {
    Numeric(Numeric),
    String(&'a str),
    Inapplicable,
    Unknown,
}

fn numeric<'s>(input: &mut &'s str) -> PResult<Value<'s>> {
    Numeric::parser.map(Value::Numeric).parse_next(input)
}

/// This parser must only be called immediately after an EOL
pub fn eol_value<'s>(input: &mut &'s str) -> PResult<Value<'s>> {
    alt((
        eol_string.map(Value::String),
        numeric,
        '.'.map(|_| Value::Inapplicable),
        '?'.map(|_| Value::Unknown),
    ))
    .parse_next(input)
}

/// This parser must only be called immediately after a non-EOL character
pub fn noteol_value<'s>(input: &mut &'s str) -> PResult<Value<'s>> {
    alt((
        noteol_string.map(Value::String),
        numeric,
        '.'.map(|_| Value::Inapplicable),
        '?'.map(|_| Value::Unknown),
    ))
    .parse_next(input)
}

pub fn whitespace_value<'s>(input: &mut &'s str) -> PResult<(&'s str, Value<'s>)> {
    alt((
        (
            whitespace.verify(|s: &str| s.chars().last() == Some('\n')),
            eol_value,
        ),
        (whitespace, noteol_value),
    ))
    .parse_next(input)
}

pub fn tag<'s>(input: &mut &'s str) -> PResult<&'s str> {
    ('_'.recognize(), nonblank1).recognize().parse_next(input)
}
