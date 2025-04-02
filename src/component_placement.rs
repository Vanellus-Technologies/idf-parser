use nom::branch::alt;

use crate::idf_v3::primitives::ws;
use nom::bytes::complete::{is_not, tag};
use nom::multi::many0;
use nom::number::complete::float;
use nom::sequence::delimited;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub struct ComponentPlacement {
    package_name: String,
    part_number: String,
    reference_designator: String, // Any (Component instance ref designator), NOREFDES, BOARD
    x: f32,
    y: f32,
    mounting_offset: f32,     // >= 0 Mounting offset from board surface
    rotation: f32,            // degrees
    board_side: String,       // "TOP" or "BOTTOM"
    placement_status: String, // "PLACED", "UNPLACED", "ECAD", "MCAD"
}

fn component_placement(input: &str) -> IResult<&str, ComponentPlacement> {
    let (
        remaining,
        (
            package_name,
            part_number,
            reference_designator,
            x,
            y,
            mounting_offset,
            rotation,
            board_side,
            placement_status,
        ),
    ) = (
        ws(is_not(" ")),                      // package name
        ws(is_not(" ")),                      // part number
        ws(is_not("\n")),                     // reference designator
        ws(float),                            // x coordinate
        ws(float),                            // y coordinate
        ws(float),                            // mounting offset
        ws(float),                            // rotation
        ws(alt((tag("TOP"), tag("BOTTOM")))), // board side
        ws(alt((
            tag("PLACED"),
            tag("UNPLACED"),
            tag("ECAD"),
            tag("MCAD"),
        ))), // placement status
    )
        .parse(input)?;

    let component_placement = ComponentPlacement {
        package_name: package_name.to_string(),
        part_number: part_number.to_string(),
        reference_designator: reference_designator.to_string(),
        x,
        y,
        mounting_offset,
        rotation,
        board_side: board_side.to_string(),
        placement_status: placement_status.to_string(),
    };

    Ok((remaining, component_placement))
}

/// Parses a section of component placements from the input string.
/// 
/// The section starts with `.PLACEMENT` and ends with `.END_PLACEMENT`.
pub fn component_placement_section(input: &str) -> IResult<&str, Vec<ComponentPlacement>> {
    delimited(
        ws(tag(".PLACEMENT")),              // section header
        ws(many0(component_placement)), // parse all component placements
        ws(tag(".END_PLACEMENT")),          // section footer
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_component_placement() {
        let input = "cs13_a pn-cap C1\n4000.0 1000.0 100.0 0.0 TOP PLACED";
        let expected = ComponentPlacement {
            package_name: "cs13_a".to_string(),
            part_number: "pn-cap".to_string(),
            reference_designator: "C1".to_string(),
            x: 4000.0,
            y: 1000.0,
            mounting_offset: 100.0,
            rotation: 0.0,
            board_side: "TOP".to_string(),
            placement_status: "PLACED".to_string(),
        };
        let result = component_placement(input);
        // assert!(result.is_ok());
        let (_, component_placement) = result.unwrap();

        assert_eq!(component_placement, expected);
    }
    #[test]
    fn test_component_placement_section() {
        let input = ".PLACEMENT
cs13_a pn-cap C1
4000.0 1000.0 100.0 0.0 TOP PLACED
cc1210 pn-cc1210 C2
3000.0 3500.0 0.0 0.0 TOP PLACED
cc1210 pn-cc1210 C3
3200.0 1800.0 0.0 0.0 BOTTOM PLACED
cc1210 pn-cc1210 C4
1400.0 2300.0 0.0 270.0 TOP PLACED
cc1210 pn-cc1210 C5
1799.5 3518.1 0.0 0.0 BOTTOM PLACED
conn_din24 connector J1
1800.0 100.0 0.0 0.0 TOP MCAD
conn_din24 connector J2
4400.0 100.0 0.0 0.0 TOP MCAD
plcc_20 pn-pal16l8-plcc U1
1800.0 3200.0 0.0 0.0 BOTTOM ECAD
plcc_20 pn-pal16l8-plcc U2
3200.0 1800.0 0.0 0.0 TOP PLACED
dip_14w pn-hs346-dip U3
3000.0 3300.0 0.0 14.0 TOP PLACED
dip_14w pn-hs346-dip U4
2200.0 2500.0 0.0 270.0 TOP PLACED
.END_PLACEMENT";

        let (remaining, component_placements) = component_placement_section(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(component_placements.len(), 11);
        assert_eq!(component_placements[0].package_name, "cs13_a");
        assert_eq!(component_placements[0].part_number, "pn-cap");
        assert_eq!(component_placements[0].reference_designator, "C1");
        assert_eq!(component_placements[0].x, 4000.0);
        assert_eq!(component_placements[0].y, 1000.0);
        assert_eq!(component_placements[0].mounting_offset, 100.0);
        assert_eq!(component_placements[0].rotation, 0.0);
        assert_eq!(component_placements[0].board_side, "TOP");
        assert_eq!(component_placements[0].placement_status, "PLACED");
    }
}
