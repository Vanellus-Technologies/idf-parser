use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::number::complete::float;
use nom::sequence::preceded;
use nom::{
    character::complete::multispace0, error::ParseError, sequence::delimited, IResult, Parser,
};

/// Determine the owner of an element.
pub fn owner(input: &str) -> IResult<&str, &str> {
    alt((tag("ECAD"), tag("MCAD"), tag("UNOWNED"))).parse(input)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, O, F>(
    inner: F,
) -> impl Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>
where
    F: Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>,
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

/// A macro which takes  in a parser and a tuple of sub_parsers,
/// repeatedly apply parser to the tuple of sub_parsers, and return a tuple of parsers
#[macro_export]
macro_rules! wrap {
    ($parser_a:expr, ($($sub_parsers:expr),+)) => {
        ($($parser_a($sub_parsers)),+)
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
/// use idf_parser::primitives::{point, ws};
/// use idf_parser::section;
/// use nom::Parser;
/// use nom::sequence::delimited;
/// use nom::bytes::complete::tag;
///
/// let input = ".SECTION
/// 0 100.0 200.0 45.0
/// .END_SECTION";
///
/// let (remaining, point) = section!("SECTION", point).parse(input).unwrap();
/// ```
#[macro_export]
macro_rules! section {
    ($section:expr, $parser:expr) => {
        delimited(
            ws(tag(format!(".{}", $section).as_str())),
            $parser,
            ws(tag(format!(".END_{}", $section).as_str())),
        )
    };
}

/// Represents a point in a loop.
#[derive(Debug, PartialEq)]
pub struct Point {
    /// The label of the point, 0 for counter-clockwise, 1 for clockwise.
    pub label: u32,
    /// The x coordinate of the point.
    pub x: f32,
    /// The y coordinate of the point.
    pub y: f32,
    /// 0 for a straight line, between 0 and 360 for an arc, 360 for a full circle.
    pub angle: f32,
}

/// Parses a point from the input string.
///
/// # Example
/// ```
/// use idf_parser::primitives::{point, Point};
/// let input = "0 100.0 200.0 45.0";
///
/// let (remaining, point) = point(input).unwrap();
/// assert_eq!(point, Point { label: 0, x: 100.0, y: 200.0, angle: 45.0 });
/// ```
pub fn point(input: &str) -> IResult<&str, Point> {
    let (remaining, (label, x, y, angle)) = (
        u32,
        preceded(tag(" "), float),
        preceded(tag(" "), float),
        preceded(tag(" "), float),
    )
        .parse(input)?;
    let point = Point { label, x, y, angle };
    Ok((remaining, point))
}

/// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let input = "0 100.0 200.0 45.0";
        let (remaining, point) = point(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            point,
            Point {
                label: 0,
                x: 100.0,
                y: 200.0,
                angle: 45.0
            }
        );
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
        let input = ".SECTION\n0 100.0 200.0 45.0\n.END_SECTION";
        let (remaining, point) = section!("SECTION", point).parse(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            point,
            Point {
                label: 0,
                x: 100.0,
                y: 200.0,
                angle: 45.0
            }
        );
    }
    #[test]
    fn test_wrap() {
        let input = "0 100.0 200.0 45.0";
        let (remaining, (label, x, y, angle)) =
            wrap!(ws, (u32, float, float, float)).parse(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(label, 0);
        assert_eq!(x, 100.0);
        assert_eq!(y, 200.0);
        assert_eq!(angle, 45.0);
    }
}
