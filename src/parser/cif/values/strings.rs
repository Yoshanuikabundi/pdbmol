use winnow::{
    combinator::{alt, delimited, opt, peek, repeat},
    prelude::*,
};

use super::super::{charsets, whitespace};

/// This parser must only be called immediately after an EOL
fn eol_text_field<'s>(input: &mut &'s str) -> PResult<&'s str> {
    delimited(
        ';',
        (
            charsets::printchar0,
            charsets::eol,
            repeat::<_, _, (), _, _>(
                0..,
                (
                    opt((charsets::text_lead_char, charsets::printchar0)),
                    charsets::eol,
                ),
            ),
        )
            .recognize(),
        ';',
    )
    .parse_next(input)
}

fn double_quoted_string<'s>(input: &mut &'s str) -> PResult<&'s str> {
    (delimited('"', charsets::printchar0, '"'), peek(whitespace))
        .map(|(s, _)| s)
        .parse_next(input)
}

fn single_quoted_string<'s>(input: &mut &'s str) -> PResult<&'s str> {
    (
        delimited('\'', charsets::printchar0, '\''),
        peek(whitespace),
    )
        .map(|(s, _)| s)
        .parse_next(input)
}

/// This parser must only be called immediately after an EOL
fn eol_unquoted_string<'s>(input: &mut &'s str) -> PResult<&'s str> {
    (charsets::ordinary_char, charsets::nonblank0)
        .recognize()
        .parse_next(input)
}

/// This parser must only be called immediately after a non-EOL character
fn noteol_unquoted_string<'s>(input: &mut &'s str) -> PResult<&'s str> {
    (alt((charsets::ordinary_char, ';')), charsets::nonblank0)
        .recognize()
        .parse_next(input)
}

/// This parser must only be called immediately after an EOL
pub fn eol_string<'s>(input: &mut &'s str) -> PResult<&'s str> {
    alt((
        eol_unquoted_string,
        eol_text_field,
        single_quoted_string,
        double_quoted_string,
    ))
    .parse_next(input)
}

/// This parser must only be called immediately after a non-EOL character
pub fn noteol_string<'s>(input: &mut &'s str) -> PResult<&'s str> {
    alt((
        noteol_unquoted_string,
        single_quoted_string,
        double_quoted_string,
    ))
    .parse_next(input)
}
