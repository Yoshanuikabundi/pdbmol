use winnow::{
    combinator::{alt, delimited, opt, peek, repeat, repeat_till},
    error::StrContext,
    prelude::*,
};

use crate::reserved::is_not_reserved;

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
                )
                    .verify(|(_, eol)| !eol.is_empty()), // Prevent infinite loop on EOF
            ),
        )
            .take(),
        ';',
    )
    .parse_next(input)
}

fn double_quoted_string<'s>(input: &mut &'s str) -> PResult<&'s str> {
    delimited(
        '"',
        repeat_till::<_, _, (), _, _, _, _>(0.., charsets::any_print_char, peek(('"', whitespace)))
            .take(),
        '"',
    )
    .context(StrContext::Label("double quoted string"))
    .parse_next(input)
}

fn single_quoted_string<'s>(input: &mut &'s str) -> PResult<&'s str> {
    delimited(
        '\'',
        repeat_till::<_, _, (), _, _, _, _>(
            0..,
            charsets::any_print_char,
            peek(('\'', whitespace)),
        )
        .take(),
        '\'',
    )
    .context(StrContext::Label("single quoted string"))
    .parse_next(input)
}

/// This parser must only be called immediately after an EOL
fn eol_unquoted_string<'s>(input: &mut &'s str) -> PResult<&'s str> {
    (charsets::ordinary_char, charsets::nonblank0)
        .take()
        .verify(is_not_reserved)
        .context(StrContext::Label("unquoted string"))
        .parse_next(input)
}

/// This parser must only be called immediately after a non-EOL character
fn noteol_unquoted_string<'s>(input: &mut &'s str) -> PResult<&'s str> {
    (alt((charsets::ordinary_char, ';')), charsets::nonblank0)
        .take()
        .verify(is_not_reserved)
        .context(StrContext::Label("unquoted string"))
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
    .context(StrContext::Label("eol string"))
    .parse_next(input)
}

/// This parser must only be called immediately after a non-EOL character
pub fn noteol_string<'s>(input: &mut &'s str) -> PResult<&'s str> {
    alt((
        noteol_unquoted_string,
        single_quoted_string,
        double_quoted_string,
    ))
    .context(StrContext::Label("noteol string"))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use winnow::combinator::terminated;

    #[test]
    fn test_eol_text_field() {
        let mut stream = "\n;this is a text field\n;";

        charsets::eol.parse_next(&mut stream).unwrap();
        let output = eol_text_field.parse(stream);
        assert_eq!(output, Ok("this is a text field\n"));

        let mut stream = "\n;this is a text field;";

        charsets::eol.parse_next(&mut stream).unwrap();
        assert!(eol_text_field.parse(stream).is_err());

        let mut stream = "\n;this is a text field;\nit has multiple lines\n;";

        charsets::eol.parse_next(&mut stream).unwrap();
        let output = eol_text_field.parse(stream);
        assert_eq!(output, Ok("this is a text field;\nit has multiple lines\n"));
    }

    #[test]
    fn test_double_quoted_string() {
        let mut stream = r#""This is a double quoted string" "#;
        let output = double_quoted_string.parse_next(&mut stream);
        assert_eq!(output, Ok("This is a double quoted string"));
        assert_eq!(stream, " ");

        let mut stream = r#""Double quoted strings must be proceeded by whitespace"lol"#;
        let output = double_quoted_string.parse_next(&mut stream);
        assert!(output.is_err());

        let mut stream = r#""Double quoted strings may include 'single quotes'" "#;
        let output = terminated(double_quoted_string, " ").parse(&mut stream);
        assert_eq!(
            output,
            Ok(r#"Double quoted strings may include 'single quotes'"#)
        );

        let mut stream = r#""Double quoted strings may include '"' as long as it is not followed by whitespace" "#;
        let output = terminated(double_quoted_string, " ").parse(&mut stream);
        assert_eq!(
            output,
            Ok(
                r#"Double quoted strings may include '"' as long as it is not followed by whitespace"#
            )
        );

        let mut stream = r#""Double quoted strings may be proceeded by EOF""#;
        let output = double_quoted_string.parse_next(&mut stream);
        assert_eq!(
            output,
            Ok(r#"Double quoted strings may be proceeded by EOF"#)
        );
    }

    #[test]
    fn test_single_quoted_string() {
        let mut stream = r#"'This is a single quoted string' "#;
        let output = single_quoted_string.parse_next(&mut stream);
        assert_eq!(output, Ok("This is a single quoted string"));
        assert_eq!(stream, " ");

        let mut stream = r#"'Single quoted strings must be proceeded by whitespace'lol"#;
        let output = single_quoted_string.parse_next(&mut stream);
        assert!(output.is_err());

        let mut stream = r#"'Single quoted strings may include "double quotes"' "#;
        let output = terminated(single_quoted_string, " ").parse(&mut stream);
        assert_eq!(
            output,
            Ok(r#"Single quoted strings may include "double quotes""#)
        );

        let mut stream = r#"'Single quoted strings may include "'" as long as it is not followed by whitespace' "#;
        let output = terminated(single_quoted_string, " ").parse(&mut stream);
        assert_eq!(
            output,
            Ok(
                r#"Single quoted strings may include "'" as long as it is not followed by whitespace"#
            )
        );

        let mut stream = r#"'Single quoted strings may be proceeded by EOF'"#;
        let output = single_quoted_string.parse_next(&mut stream);
        assert_eq!(
            output,
            Ok(r#"Single quoted strings may be proceeded by EOF"#)
        );
    }
}
