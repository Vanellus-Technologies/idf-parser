use nom::branch::alt;

use crate::primitives::ws;
use crate::{parse_section, ws_separated};
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::not_line_ending;
use nom::multi::many0;
use nom::number::complete::float;
use nom::sequence::delimited;

/// Represents a component placement in the IDF format.
#[derive(Debug, PartialEq, Clone)]
pub struct ComponentPlacement {
    pub package_name: String,
    pub part_number: String,
    pub reference_designator: String, // Any (Component instance ref designator), NOREFDES, BOARD
    pub x: f32,
    pub y: f32,
    pub mounting_offset: f32,     // >= 0 Mounting offset from board surface
    pub rotation_angle: f32,      // degrees
    pub board_side: String,       // "TOP" or "BOTTOM"
    pub placement_status: String, // "PLACED", "UNPLACED", "ECAD", "MCAD"
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
            rotation_angle,
            board_side,
            placement_status,
        ),
    ) = ws_separated!((
        is_not(" "),                                                      // package name
        is_not(" "),                                                      // part number
        not_line_ending,                                                     // reference designator
        float,                                                            // x coordinate
        float,                                                            // y coordinate
        float,                                                            // mounting offset
        float,                                                            // rotation
        alt((tag("TOP"), tag("BOTTOM"))),                                 // board side
        alt((tag("PLACED"), tag("UNPLACED"), tag("ECAD"), tag("MCAD"),))  // placement status
    ))
    .parse(input)?;

    let component_placement = ComponentPlacement {
        package_name: package_name.to_string(),
        part_number: part_number.to_string(),
        reference_designator: reference_designator.to_string(),
        x,
        y,
        mounting_offset,
        rotation_angle,
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
    parse_section!("PLACEMENT", ws(many0(component_placement))).parse(input)
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
            rotation_angle: 0.0,
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
3000.0 3500.0 0.0 0.0 TOP UNPLACED
cc1210 pn-cc1210 C3
3200.0 1800.0 0.0 0.0 BOTTOM MCAD
dip_14w pn-hs346-dip U4
2200.0 2500.0 0.0 270.0 TOP ECAD
.END_PLACEMENT";

        let expected = vec![
            ComponentPlacement {
                package_name: "cs13_a".to_string(),
                part_number: "pn-cap".to_string(),
                reference_designator: "C1".to_string(),
                x: 4000.0,
                y: 1000.0,
                mounting_offset: 100.0,
                rotation_angle: 0.0,
                board_side: "TOP".to_string(),
                placement_status: "PLACED".to_string(),
            },
            ComponentPlacement {
                package_name: "cc1210".to_string(),
                part_number: "pn-cc1210".to_string(),
                reference_designator: "C2".to_string(),
                x: 3000.0,
                y: 3500.0,
                mounting_offset: 0.0,
                rotation_angle: 0.0,
                board_side: "TOP".to_string(),
                placement_status: "UNPLACED".to_string(),
            },
            ComponentPlacement {
                package_name: "cc1210".to_string(),
                part_number: "pn-cc1210".to_string(),
                reference_designator: "C3".to_string(),
                x: 3200.0,
                y: 1800.0,
                mounting_offset: 0.0,
                rotation_angle: 0.0,
                board_side: "BOTTOM".to_string(),
                placement_status: "MCAD".to_string(),
            },
            ComponentPlacement {
                package_name: "dip_14w".to_string(),
                part_number: "pn-hs346-dip".to_string(),
                reference_designator: "U4".to_string(),
                x: 2200.0,
                y: 2500.0,
                mounting_offset: 0.0,
                rotation_angle: 270.0,
                board_side: "TOP".to_string(),
                placement_status: "ECAD".to_string(),
            },
        ];

        let (remaining, component_placements) = parse_component_placement_section(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(component_placements, expected);
    }
    #[test]
    fn test_invalid_component_placement() {
        // Invalid input with extra text
        let input = "cs13_a pn-cap C1\n4000.0 1000.0 100.0 hi hi 0.0 TOP PLACED\n";
        let result = component_placement(input);
        assert!(!result.is_ok());
    }
}
