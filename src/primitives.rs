use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::number::complete::float;
use nom::sequence::preceded;
use nom::{
    character::complete::multispace0, error::ParseError, sequence::delimited, IResult, Parser,
};

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

/// units
pub const MM: &str = "MM"; // millimeters
pub const THOU: &str = "THOU"; // mils (thousandths of an inch)

#[derive(Debug, PartialEq)]
pub struct Point {
    pub label: u32, // 0 for counter-clockwise, 1 for clockwise
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

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

pub fn units(input: &str) -> IResult<&str, &str> {
    alt((tag(MM), tag(THOU))).parse(input)
}