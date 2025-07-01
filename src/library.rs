use crate::components::{
    ElectricalComponent, MechanicalComponent, electrical_component, mechanical_component,
};
use crate::headers::{LibraryHeader, parse_library_header};
use nom::Parser;
use nom::multi::many0;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Library {
    pub header: LibraryHeader,
    pub electrical_components: Vec<ElectricalComponent>,
    pub mechanical_components: Vec<MechanicalComponent>,
}

/// Parses a library emp file which contains detail on electrical and mechanical components.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=29
pub fn parse_library(input: &str) -> Result<Library, nom::Err<nom::error::Error<&str>>> {
    // Sometimes mechanical components are first, sometimes electrical components are first.
    let (body, header) = parse_library_header.parse(input)?;

    // Check if body starts with ".ELECTRICAL" or ".MECHANICAL"
    let (remaining, (electrical_components, mechanical_components)) =
        if body.starts_with(".ELECTRICAL") {
            let (remaining, (electrical_components, mechanical_components)) =
                (many0(electrical_component), many0(mechanical_component)).parse(body)?;
            (remaining, (electrical_components, mechanical_components))
        } else if body.starts_with(".MECHANICAL") {
            let (remaining, (mechanical_components, electrical_components)) =
                (many0(mechanical_component), many0(electrical_component)).parse(body)?;
            (remaining, (electrical_components, mechanical_components))
        } else {
            return Err(nom::Err::Error(nom::error::Error::new(
                body,
                nom::error::ErrorKind::Tag,
            )));
        };

    let library = Library {
        header,
        electrical_components,
        mechanical_components,
    };

    // Check nothing is remaining
    if !remaining.is_empty() {
        Err(nom::Err::Error(nom::error::Error::new(
            remaining,
            nom::error::ErrorKind::Tag,
        )))
    } else {
        Ok(library)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;
    use std::collections::HashMap;
    #[test]
    fn test_library() {
        let input = ".HEADER
LIBRARY_FILE 3.0 \"Sample File Generator\" 10/22/96.16:41:37 1
.END_HEADER
.ELECTRICAL
cs13_a pn-cap THOU 150.0
0 -55.0 55.0 0.0
0 -55.0 55.0 0.0
PROP CAPACITANCE 100.0
PROP TOLERANCE 5.0
.END_ELECTRICAL
.ELECTRICAL
cc1210 pn-cc1210 THOU 67.0
0 -40.0 56.0 0.0
PROP CAPACITANCE 0.1
PROP TOLERANCE 5.0
.END_ELECTRICAL
.ELECTRICAL
conn_din24 connector THOU 435.0
0 -1400.0 -500.0 0.0
.END_ELECTRICAL
.ELECTRICAL
dip_14w pn-hs346-dip THOU 200.0
0 350.0 50.0 0.0
.END_ELECTRICAL
.ELECTRICAL
plcc_20 pn-pal16l8-plcc THOU 14.0
0 -200.0 240.0 0.0
0 -240.0 200.0 0.0
.END_ELECTRICAL";
        let library = parse_library(input).unwrap();
        assert_eq!(library.electrical_components.len(), 5);

        let header = LibraryHeader {
            version: 3,
            system_id: "Sample File Generator".to_string(),
            date: "10/22/96.16:41:37".to_string(),
            file_version: 1,
        };

        let electrical_components = vec![
            ElectricalComponent {
                geometry_name: "cs13_a".to_string(),
                part_number: "pn-cap".to_string(),
                units: "THOU".to_string(),
                height: 150.0,
                outline: vec![
                    Point {
                        loop_label: 0,
                        x: -55.0,
                        y: 55.0,
                        angle: 0.0,
                    },
                    Point {
                        loop_label: 0,
                        x: -55.0,
                        y: 55.0,
                        angle: 0.0,
                    },
                ],
                properties: {
                    let mut props = HashMap::new();
                    props.insert("CAPACITANCE".to_string(), 100.0);
                    props.insert("TOLERANCE".to_string(), 5.0);
                    props
                },
            },
            ElectricalComponent {
                geometry_name: "cc1210".to_string(),
                part_number: "pn-cc1210".to_string(),
                units: "THOU".to_string(),
                height: 67.0,
                outline: vec![Point {
                    loop_label: 0,
                    x: -40.0,
                    y: 56.0,
                    angle: 0.0,
                }],
                properties: {
                    let mut props = HashMap::new();
                    props.insert("CAPACITANCE".to_string(), 0.1);
                    props.insert("TOLERANCE".to_string(), 5.0);
                    props
                },
            },
            ElectricalComponent {
                geometry_name: "conn_din24".to_string(),
                part_number: "connector".to_string(),
                units: "THOU".to_string(),
                height: 435.0,
                outline: vec![Point {
                    loop_label: 0,
                    x: -1400.0,
                    y: -500.0,
                    angle: 0.0,
                }],
                properties: HashMap::new(),
            },
            ElectricalComponent {
                geometry_name: "dip_14w".to_string(),
                part_number: "pn-hs346-dip".to_string(),
                units: "THOU".to_string(),
                height: 200.0,
                outline: vec![Point {
                    loop_label: 0,
                    x: 350.0,
                    y: 50.0,
                    angle: 0.0,
                }],
                properties: HashMap::new(),
            },
            ElectricalComponent {
                geometry_name: "plcc_20".to_string(),
                part_number: "pn-pal16l8-plcc".to_string(),
                units: "THOU".to_string(),
                height: 14.0,
                outline: vec![
                    Point {
                        loop_label: 0,
                        x: -200.0,
                        y: 240.0,
                        angle: 0.0,
                    },
                    Point {
                        loop_label: 0,
                        x: -240.0,
                        y: 200.0,
                        angle: 0.0,
                    },
                ],
                properties: HashMap::new(),
            },
        ];

        let expected_library = Library {
            header,
            electrical_components,
            mechanical_components: vec![],
        };

        assert_eq!(library, expected_library);
    }
}
