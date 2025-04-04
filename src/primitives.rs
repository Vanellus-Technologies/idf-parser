use nom::character::complete::u32;
use nom::number::complete::float;
use nom::{Parser, character::complete::multispace0, error::ParseError, sequence::delimited};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, Output, Function>(
    inner: Function,
) -> impl Parser<&'a str, Output = Output, Error = nom::error::Error<&'a str>>
where
    Function: Parser<&'a str, Output = Output, Error = nom::error::Error<&'a str>>,
{
    delimited(multispace0, inner, multispace0)
}

/// Takes in a tuple of parsers with different return types
/// and returns a tuple of parsers each wrapped with `ws`.
///
/// # Example
/// ```
/// use nom::character::complete::u32;
/// use nom::number::complete::float;
/// use nom::Parser;
/// use idf_parser::ws_separated;
/// use idf_parser::primitives::ws;
///
/// let input = "0 100.0 200.0 45.0";
///
/// let (remaining, (label, x, y, angle)) = ws_separated!((u32, float, float, float)).parse(input).unwrap();
/// ```
#[macro_export]
macro_rules! ws_separated {
    (($($parser:expr),+)) => {
        ($(ws($parser)),+)
    };
}

/// Section parser
///
/// Takes a section delimited by `.section` and `.end_section` and applies the given parser to the
/// content of the section.
///
/// # Example
///
/// ```
/// use idf_parser::primitives::ws;
/// use idf_parser::parse_section;
/// use nom::Parser;
/// use nom::sequence::delimited;
/// use nom::bytes::complete::tag;
///
/// let input = ".SECTION
/// howdy!
/// .END_SECTION";
///
/// let (remaining, point) = parse_section!("SECTION", tag("howdy!")).parse(input).unwrap();
/// ```
#[macro_export]
macro_rules! parse_section {
    ($section:expr, $parser:expr) => {
        delimited(
            ws(tag(format!(".{}", $section).as_str())),
            $parser,
            ws(tag(format!(".END_{}", $section).as_str())),
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::bytes::complete::tag;

    #[test]
    fn test_ws() {
        let input = "\r0 \n\n\n100.0   200.0 \n45.0  ";
        let (remaining, (label, x, y, angle)) = (ws(u32), ws(float), ws(float), ws(float))
            .parse(input)
            .unwrap();
        assert_eq!(remaining, "");
        assert_eq!(label, 0);
        assert_eq!(x, 100.0);
        assert_eq!(y, 200.0);
        assert_eq!(angle, 45.0);
    }

    #[test]
    fn test_ws_separated() {
        let input = "0 100.0 200.0 45.0";
        let (remaining, (label, x, y, angle)) = ws_separated!((u32, float, float, float))
            .parse(input)
            .unwrap();
        assert_eq!(remaining, "");
        assert_eq!(label, 0);
        assert_eq!(x, 100.0);
        assert_eq!(y, 200.0);
        assert_eq!(angle, 45.0);
    }

    #[test]
    fn test_section() {
        let input = ".SECTION\n123 456\n.END_SECTION";
        let (remaining, ints) = parse_section!("SECTION", (ws(u32), ws(u32)))
            .parse(input)
            .unwrap();
        assert_eq!(remaining, "");
        assert_eq!(ints, (123, 456));
    }
}
