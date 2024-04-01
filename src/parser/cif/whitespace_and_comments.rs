use super::charsets;
use winnow::{
    combinator::{alt, repeat},
    prelude::*,
};

fn comment<'s>(input: &mut &'s str) -> PResult<&'s str> {
    ('#', charsets::printchar0, charsets::eol)
        .recognize()
        .parse_next(input)
}

pub fn comments<'s>(input: &mut &'s str) -> PResult<&'s str> {
    repeat::<_, _, (), _, _>(1.., comment)
        .recognize()
        .parse_next(input)
}

fn tokenized_comments<'s>(input: &mut &'s str) -> PResult<&'s str> {
    (
        repeat::<_, _, (), _, _>(1.., charsets::whitespace).recognize(),
        comments,
    )
        .recognize()
        .parse_next(input)
}

pub fn whitespace<'s>(input: &mut &'s str) -> PResult<&'s str> {
    repeat::<_, _, (), _, _>(
        1..,
        alt((tokenized_comments.recognize(), charsets::whitespace)),
    )
    .recognize()
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment() {
        let mut stream = "# hello world # \n new line";

        let output = comment.parse_next(&mut stream);
        assert_eq!(stream, " new line");
        assert_eq!(output, Ok("# hello world # \n"));

        let output = comment.parse_next(&mut stream);
        assert!(output.is_err());

        let mut stream = "# comment without newline ends with stream";
        let output = comment.parse_next(&mut stream);
        assert_eq!(output, Ok("# comment without newline ends with stream"));
    }

    #[test]
    fn test_comments() {
        let mut stream = "# hello world # \n# new line, same comments\nThis line is important";

        let output = comments.parse_next(&mut stream);
        assert_eq!(output, Ok("# hello world # \n# new line, same comments\n"));

        let output = comments.parse_next(&mut stream);
        assert!(output.is_err());

        let mut stream = "# continuation comments with \n # whitespace after EOL are separate";
        let output = comments.parse_next(&mut stream);
        assert_eq!(output, Ok("# continuation comments with \n"));
        assert_eq!(stream, " # whitespace after EOL are separate");

        let output = comments.parse_next(&mut stream);
        assert!(output.is_err());

        let mut stream = "This line is important # and has a comment\n";
        let output = comments.parse_next(&mut stream);
        assert!(output.is_err());

        let mut stream = "# single line comment\ndata";
        let output = comments.parse_next(&mut stream);
        assert_eq!(output, Ok("# single line comment\n"));
        assert_eq!(stream, "data");
    }

    #[test]
    fn test_tokenized() {
        let mut stream = "  \t\n\n# hello world # \n# new line, same comments\ndata";

        let output = tokenized_comments.parse_next(&mut stream);
        assert_eq!(
            output,
            Ok("  \t\n\n# hello world # \n# new line, same comments\n")
        );
        assert_eq!(stream, "data");

        let output = comments.parse_next(&mut stream);
        assert!(output.is_err());
    }

    #[test]
    fn test_whitespace() {
        let mut stream = " asdf";
        let output = whitespace.parse_next(&mut stream);
        assert_eq!(output, Ok(" "));
        assert_eq!(stream, "asdf");

        let output = comments.parse_next(&mut stream);
        assert!(output.is_err());

        let mut stream = " \t\t    \n\n\n\n#hello there \n # hi # \n #\ndata";
        let output = whitespace.parse_next(&mut stream);
        assert!(output.is_ok());
        assert_eq!(stream, "data");
    }
}
