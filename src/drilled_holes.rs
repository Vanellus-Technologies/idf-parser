use nom::branch::alt;
use nom::sequence::delimited;

use crate::primitives::{owner, ws};
use crate::ws_separated;
use nom::bytes::complete::{is_not, tag};
use nom::multi::many1;
use nom::number::complete::float;
use nom::IResult;
use nom::Parser;

/// Represents a drilled hole in the IDF format.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=25
pub struct Hole {
    pub diameter: f32,
    pub x: f32,                  // absolute x coordinate
    pub y: f32,                  // absolute y coordinate
    pub plating_style: String, // PTH: Plated (conducting) through hole, NPTH: Non-plated (non-conducting) through hole
    pub associated_part: String, // BOARD, NOREFDES, PANEL, Reference designator
    pub hole_type: String,     // PIN, VIA, MTG, TOOL, Other
    pub owner: String,         // The owner of the hole ECAD, MCAD, UNOWNED
}

/// Parses a single drilled hole from the input string.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=25
///
/// # Example
///
/// ```
/// use idf_parser::drilled_holes::{drilled_hole, Hole};
/// let input = "30.0 1600.0 100.0 PTH J1 PIN ECAD";
///
/// let (remaining, hole) = drilled_hole(input).unwrap();
/// assert_eq!(hole.x, 1600.0);
/// ```
pub fn drilled_hole(input: &str) -> IResult<&str, Hole> {
    let (remaining, (diameter, x, y, plating_style, associated_part, hole_type, owner)) =
        ws_separated!((
            float,                                                  // diameter
            float,                                                  // x coordinate
            float,                                                  // y coordinate
            alt((tag("PTH"), tag("NPTH"))),                         // plating style
            is_not(" "),                                            // associated part
            alt((tag("PIN"), tag("VIA"), tag("MTG"), tag("TOOL"))), // hole type TODO add support user defined hole types
            owner
        ))
        .parse(input)?;

    let hole = Hole {
        diameter,
        x,
        y,
        plating_style: plating_style.to_string(),
        associated_part: associated_part.to_string(),
        hole_type: hole_type.to_string(),
        owner: owner.to_string(),
    };
    Ok((remaining, hole))
}

/// Parses a section of drilled holes from the input string.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=25
///
/// # Example
///
/// ```
/// use idf_parser::drilled_holes::{parse_drilled_holes_section, Hole};
///         let input = ".DRILLED_HOLES
/// 30.0 1800.0 100.0 PTH J1 PIN ECAD
/// 30.0 1700.0 100.0 PTH J1 PIN ECAD
/// 30.0 1600.0 100.0 PTH J1 PIN ECAD
/// 93.0 0.0 4800.0 NPTH BOARD TOOL MCAD
/// 93.0 0.0 0.0 PTH BOARD MTG UNOWNED
/// .END_DRILLED_HOLES";
///
/// let (remaining, holes) = parse_drilled_holes_section(input).unwrap();
/// // assert_eq!(holes[0].owner, "ECAD");
/// ```
pub fn parse_drilled_holes_section(input: &str) -> IResult<&str, Vec<Hole>> {
    delimited(
        ws(tag(".DRILLED_HOLES\n")),
        many1(drilled_hole),
        ws(tag(".END_DRILLED_HOLES")),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_drilled_hole() {
        let input = "30.0 1800.0 100.0 PTH J1 PIN ECAD";

        let (remaining, hole) = drilled_hole(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(hole.diameter, 30.0);
        assert_eq!(hole.x, 1800.0);
        assert_eq!(hole.y, 100.0);
        assert_eq!(hole.plating_style, "PTH");
        assert_eq!(hole.associated_part, "J1");
        assert_eq!(hole.hole_type, "PIN");
        assert_eq!(hole.owner, "ECAD");
    }

    #[test]
    fn test_drilled_holes_section() {
        let input = ".DRILLED_HOLES
30.0 1800.0 100.0 PTH J1 PIN ECAD
30.0 1700.0 100.0 PTH J1 PIN ECAD
30.0 1600.0 100.0 PTH J1 PIN ECAD
93.0 0.0 4800.0 NPTH BOARD TOOL MCAD
93.0 0.0 0.0 PTH NOREFDES MTG UNOWNED
123.0 0.0 0.0 PTH NOREFDES VIA UNOWNED
.END_DRILLED_HOLES";
        let (remaining, holes) = parse_drilled_holes_section(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(holes.len(), 6);
        assert_eq!(holes[0].diameter, 30.0);
        assert_eq!(holes[0].x, 1800.0);
        assert_eq!(holes[0].y, 100.0);
        assert_eq!(holes[0].plating_style, "PTH");
        assert_eq!(holes[0].associated_part, "J1");
        assert_eq!(holes[0].hole_type, "PIN");
        assert_eq!(holes[0].owner, "ECAD");
        assert_eq!(holes[1].plating_style, "PTH");
        assert_eq!(holes[3].plating_style, "NPTH");
        assert_eq!(holes[4].associated_part, "NOREFDES");
        assert_eq!(holes[5].hole_type, "VIA");
    }
}
