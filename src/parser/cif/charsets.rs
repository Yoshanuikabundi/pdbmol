use winnow::{
    ascii::line_ending,
    combinator::{alt, eof, repeat},
    prelude::*,
    token::one_of,
};

pub fn ordinary_char(input: &mut &str) -> PResult<char> {
    one_of(('!', '%'..='&', '('..=':', '<'..='Z', '\\', '^', '`'..='~')).parse_next(input)
}

pub fn nonblank_char(input: &mut &str) -> PResult<char> {
    one_of('!'..='~').parse_next(input)
}

pub fn text_lead_char(input: &mut &str) -> PResult<char> {
    one_of(('\t', ' ', '!'..=':', '<'..='~')).parse_next(input)
}

pub fn any_print_char(input: &mut &str) -> PResult<char> {
    one_of(('\t', ' ', '!'..='~')).parse_next(input)
}

pub fn printchar0<'s>(input: &mut &'s str) -> PResult<&'s str> {
    repeat::<_, _, (), _, _>(0.., any_print_char)
        .recognize()
        .parse_next(input)
}

pub fn printchar1<'s>(input: &mut &'s str) -> PResult<&'s str> {
    repeat::<_, _, (), _, _>(1.., any_print_char)
        .recognize()
        .parse_next(input)
}

pub fn nonblank0<'s>(input: &mut &'s str) -> PResult<&'s str> {
    repeat::<_, _, (), _, _>(0.., nonblank_char)
        .recognize()
        .parse_next(input)
}

pub fn nonblank1<'s>(input: &mut &'s str) -> PResult<&'s str> {
    repeat::<_, _, (), _, _>(1.., nonblank_char)
        .recognize()
        .parse_next(input)
}

pub fn eol<'s>(input: &mut &'s str) -> PResult<&'s str> {
    alt((line_ending, eof)).parse_next(input)
}

pub fn whitespace<'s>(input: &mut &'s str) -> PResult<&'s str> {
    alt((' '.recognize(), '\t'.recognize(), eol.recognize())).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ORDINARY_CHARS: [char; 86] = [
        '!', '%', '&', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3', '4', '5', '6',
        '7', '8', '9', ':', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
        'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '\\',
        '^', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~',
    ];

    #[test]
    fn test_ordinary_char() {
        for c in (0..=256).map(|b| char::from_u32(b).unwrap()) {
            let mut buf = [0; 4];
            let mut stream = &*c.encode_utf8(&mut buf);

            let output = ordinary_char.parse_next(&mut stream);
            if ORDINARY_CHARS.contains(&c) {
                assert_eq!(stream, "");
                assert_eq!(output, Ok(c));
            } else {
                assert!(output.is_err())
            }
        }
    }

    #[test]
    fn test_nonblank_char() {
        for c in (0..=256).map(|b| char::from_u32(b).unwrap()) {
            let mut buf = [0; 4];
            let mut stream = &*c.encode_utf8(&mut buf);

            let output = nonblank_char.parse_next(&mut stream);
            if ORDINARY_CHARS.contains(&c) || ['"', '#', '$', '\'', '_', ';', '[', ']'].contains(&c)
            {
                assert_eq!(stream, "");
                assert_eq!(output, Ok(c));
            } else {
                assert!(output.is_err())
            }
        }
    }

    #[test]
    fn test_text_lead_char() {
        for c in (0..=256).map(|b| char::from_u32(b).unwrap()) {
            let mut buf = [0; 4];
            let mut stream = &*c.encode_utf8(&mut buf);

            let output = text_lead_char.parse_next(&mut stream);
            if ORDINARY_CHARS.contains(&c)
                || ['"', '#', '$', '\'', '_', ' ', '\t', '[', ']'].contains(&c)
            {
                assert_eq!(stream, "");
                assert_eq!(output, Ok(c));
            } else {
                assert!(output.is_err())
            }
        }
    }

    #[test]
    fn test_any_print_char() {
        for c in (0..=256).map(|b| char::from_u32(b).unwrap()) {
            let mut buf = [0; 4];
            let mut stream = &*c.encode_utf8(&mut buf);

            let output = any_print_char.parse_next(&mut stream);
            if ORDINARY_CHARS.contains(&c)
                || ['"', '#', '$', '\'', '_', ' ', '\t', ';', '[', ']'].contains(&c)
            {
                assert_eq!(stream, "");
                assert_eq!(output, Ok(c));
            } else {
                assert!(output.is_err())
            }
        }
    }
}
