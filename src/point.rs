use crate::primitives::ws;
use crate::ws_separated;

use nom::character::complete::u32;
use nom::number::complete::float;
use nom::{IResult, Parser};

/// Represents a point which exists as part of 2D loop of points which describe an outline of a
/// component or board section.
///
/// Used repeatedly in the IDF format to represent points in a loop.
/// First mention here:
/// http://www.simplifiedsolutionsinc.com/images/idf_v30_spec.pdf#page=10 in Record 3
#[derive(Debug, PartialEq, Clone, Default, PartialOrd)]
pub struct Point {
    /// The label of the loop the point exist in, 0 for counter-clockwise, 1 for clockwise.
    pub loop_label: u32,
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
/// use idf_parser::point::{point, Point};
/// let input = "0 100.0 200.0 45.0";
///
/// let (remaining, point) = point(input).unwrap();
/// assert_eq!(point, Point { loop_label: 0, x: 100.0, y: 200.0, angle: 45.0 });
/// ```
pub fn point(input: &str) -> IResult<&str, Point> {
    let (remaining, (loop_label, x, y, angle)) =
        ws_separated!((u32, float, float, float)).parse(input)?;
    let point = Point {
        loop_label,
        x,
        y,
        angle,
    };
    Ok((remaining, point))
}

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
                loop_label: 0,
                x: 100.0,
                y: 200.0,
                angle: 45.0
            }
        );
    }
}
