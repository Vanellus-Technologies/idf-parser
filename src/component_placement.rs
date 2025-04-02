use nom::branch::alt;

use crate::primitives::ws;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::{is_not, tag};
use nom::multi::many0;
use nom::number::complete::float;
use nom::sequence::delimited;

/// Represents a component placement in the IDF format.
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

/// Parses a single component placement from the input string.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=27
///
/// # Example
///
/// ```
/// use idf_parser::component_placement::{component_placement, ComponentPlacement};
/// let input = "cs13_a pn-cap C1\n4000.0 1000.0 100.0 0.0 TOP PLACED";
/// let (remaining, component_placement) = component_placement(input).unwrap();
/// ```
pub fn component_placement(input: &str) -> IResult<&str, ComponentPlacement> {
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
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=27
///
/// # Example
///
/// ```
/// use idf_parser::component_placement::{parse_component_placement_section, ComponentPlacement};
/// let input = ".PLACEMENT
/// cs13_a pn-cap C1
/// 4000.0 1000.0 100.0 0.0 TOP PLACED
/// .END_PLACEMENT";
///
/// let (remaining, component_placements) = parse_component_placement_section(input).unwrap();
/// ```
pub fn parse_component_placement_section(input: &str) -> IResult<&str, Vec<ComponentPlacement>> {
    delimited(
        ws(tag(".PLACEMENT")),          // section header
        ws(many0(component_placement)), // parse all component placements
        ws(tag(".END_PLACEMENT")),      // section footer
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
dip_14w pn-hs346-dip U4
2200.0 2500.0 0.0 270.0 TOP PLACED
.END_PLACEMENT";

        let (remaining, component_placements) = parse_component_placement_section(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(component_placements.len(), 4);
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
