use nom::branch::alt;
use nom::sequence::{delimited, terminated};

use crate::idf_v3::primitives::{owner, ws};
use nom::bytes::complete::{is_not, tag};
use nom::multi::many1;
use nom::number::complete::float;
use nom::IResult;
use nom::Parser;

pub struct Hole {
    diameter: f32,
    x: f32,                  // absolute x coordinate
    y: f32,                  // absolute y coordinate
    plating_style: String, // PTH: Plated (conducting) through hole, NPTH: Non-plated (non-conducting) through hole
    associated_part: String, // BOARD, NOREFDES, PANEL, Reference designator
    hole_type: String,     // PIN, VIA, MTG, TOOL, Other
    owner: String,         // The owner of the hole ECAD, MCAD, UNOWNED
}

fn drilled_hole(input: &str) -> IResult<&str, Hole> {
    let (remaining, (diameter, x, y, plating_style, associated_part, hole_type, owner)) = (
        terminated(float, tag(" ")),                          // diameter
        terminated(float, tag(" ")),                          // x coordinate
        terminated(float, tag(" ")),                          // y coordinate
        terminated(alt((tag("PTH"), tag("NPTH"))), tag(" ")), // plating style
        terminated(is_not(" "), tag(" ")),                    // associated part
        terminated(
            alt((tag("PIN"), tag("VIA"), tag("MTG"), tag("TOOL"))),
            tag(" "),
        ), // hole type TODO add support user defined hole types
        owner,
    )
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

pub fn drilled_holes_section(input: &str) -> IResult<&str, Vec<Hole>> {
    delimited(
        ws(tag(".DRILLED_HOLES\n")),
        many1(terminated(drilled_hole, tag("\n"))),
        ws(tag(".END_DRILLED_HOLES")),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
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
93.0 0.0 0.0 PTH BOARD MTG UNOWNED
.END_DRILLED_HOLES";
        let (remaining, holes) = drilled_holes_section(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(holes.len(), 5);
        assert_eq!(holes[0].diameter, 30.0);
        assert_eq!(holes[0].x, 1800.0);
        assert_eq!(holes[0].y, 100.0);
        assert_eq!(holes[0].plating_style, "PTH");
        assert_eq!(holes[0].associated_part, "J1");
        assert_eq!(holes[0].hole_type, "PIN");
        assert_eq!(holes[0].owner, "ECAD");
    }
}
