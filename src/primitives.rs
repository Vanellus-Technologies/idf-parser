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
pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
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
