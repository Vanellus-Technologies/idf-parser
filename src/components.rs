use crate::point::{Point, point};
use crate::primitives::{quote_string, ws};
use crate::{parse_section, ws_separated};
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::multi::many0;
use nom::number::complete::float;
use nom::sequence::delimited;
use std::collections::HashMap;

/// Parses the properties of an electrical component from the input string.
///
/// Represents the properties of an electrical component.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=33
///
/// Properties of an electrical component:
/// - capacitance: Capacitance in microfarads
/// - resistance: Resistance in ohms
/// - tolerance: Percent deviation
/// - power_opr: Operating power rating in milliwatts
/// - power_max: Maximum power rating in milliwatts
/// - thermal_cond: Thermal conductivity in watts per meter °C
/// - theta_jb: Junction to board thermal resistance in °C per watt
/// - theta_jc: Junction to case thermal resistance in °C per watt
/// - other: User-defined properties
fn electrical_properties(input: &str) -> IResult<&str, ElectricalProperties> {
    let (remaining, properties) = many0(electrical_property).parse(input)?;
    Ok((remaining, properties.into_iter().collect()))
}

/// Parse a single property entry for an electrical component.
fn electrical_property(input: &str) -> IResult<&str, (String, f32)> {
    let (remaining, (_prop_tag, prop_name, value)) =
        ws_separated!((tag("PROP"), is_not(" "), float)).parse(input)?;
    Ok((remaining, (prop_name.to_string(), value)))
}

/// Represent properties of an electrical component.
type ElectricalProperties = HashMap<String, f32>;

/// Represents an electrical component in the IDF format.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=31
#[derive(Debug, PartialEq, Clone, Default)]
pub struct ElectricalComponent {
    pub geometry_name: String,
    pub part_number: String,
    pub units: String,
    pub height: f32,
    pub outline: Vec<Point>,
    pub properties: ElectricalProperties,
}

/// Represents a mechanical component in the IDF format.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=34
#[derive(Debug, PartialEq, Clone, Default, PartialOrd)]
pub struct MechanicalComponent {
    pub geometry_name: String,
    pub part_number: String,
    pub units: String,
    pub height: f32,
    pub outline: Vec<Point>,
}

/// Parses an electrical component from the input string.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=31
///
/// # Example
///
/// ```
/// use idf_parser::components::electrical_component;
///
/// let input = ".ELECTRICAL
/// cs13_a pn-cap THOU 150.0
/// 0 -55.0 55.0 0.0
/// .END_ELECTRICAL";
///
/// let (remaining, component) = electrical_component(input).unwrap();
/// assert_eq!(component.geometry_name, "cs13_a");
/// ```
pub fn electrical_component(input: &str) -> IResult<&str, ElectricalComponent> {
    let (remaining, (geometry_name, part_number, units, height, outline, properties)) =
        parse_section!(
            "ELECTRICAL",
            ws_separated!((
                is_not(" "), // geometry name
                alt((
                    quote_string, // part number with quotes
                    is_not(" "),  // part number without quotes
                )),
                is_not(" "),           // units
                float,                 // height
                many0(ws(point)),      // outline
                electrical_properties  // electrical component properties
            ))
        )
        .parse(input)?;

    let electrical_component = ElectricalComponent {
        geometry_name: geometry_name.to_string(),
        part_number: part_number.to_string(),
        units: units.to_string(),
        height,
        outline,
        properties,
    };

    Ok((remaining, electrical_component))
}

/// Parses a mechanical component from the input string.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=34
/// # Example
///
/// ```
/// use idf_parser::components::mechanical_component;
///
/// let input = ".MECHANICAL
/// cs13_a pn-cap THOU 150.0
/// 0 -55.0 55.0 0.0
/// 0 -55.0 -55.0 0.0
/// .END_MECHANICAL";
///
/// let (remaining, component) = mechanical_component(input).unwrap();
/// assert_eq!(component.geometry_name, "cs13_a");
/// ```
pub fn mechanical_component(input: &str) -> IResult<&str, MechanicalComponent> {
    let (remaining, (geometry_name, part_number, units, height, outline)) = parse_section!(
        "MECHANICAL",
        ws_separated!((
            is_not(" "),      // geometry name
            is_not(" "),      // part number
            is_not(" "),      // units
            float,            // height
            many0(ws(point))  // outline
        ))
    )
    .parse(input)?;

    let mechanical_component = MechanicalComponent {
        geometry_name: geometry_name.to_string(),
        part_number: part_number.to_string(),
        units: units.to_string(),
        height,
        outline,
    };

    Ok((remaining, mechanical_component))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_electrical_component() {
        let input = ".ELECTRICAL
cs13_a pn-cap THOU 150.0
0 -55.0 55.0 0.0
0 -55.0 -55.0 0.0
0 135.0 -55.0 0.0
0 135.0 -80.0 0.0
0 565.0 -80.0 0.0
0 565.0 -55.0 0.0
0 755.0 -55.0 0.0
0 755.0 55.0 0.0
0 565.0 55.0 0.0
0 565.0 80.0 0.0
0 135.0 80.0 0.0
0 135.0 55.0 0.0
0 -55.0 55.0 0.0
PROP CAPACITANCE 100.0
PROP TOLERANCE 5.0
PROP RESISTANCE 122.0
PROP POWER_OPR 2.5
PROP POWER_MAX 9.12
PROP THERM_COND 0.0
PROP THETA_JB 0.2
PROP THETA_JC 5.1
.END_ELECTRICAL";
        let (remaining, component) = electrical_component(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(component.geometry_name, "cs13_a");
        assert_eq!(component.part_number, "pn-cap");
        assert_eq!(component.units, "THOU");
        assert_eq!(component.height, 150.0);
        assert_eq!(component.outline.len(), 13);
        assert_eq!(component.properties["CAPACITANCE"], 100.0);
        assert_eq!(component.properties["TOLERANCE"], 5.0);
        assert_eq!(component.properties["RESISTANCE"], 122.0);
        assert_eq!(component.properties["POWER_OPR"], 2.5);
        assert_eq!(component.properties["POWER_MAX"], 9.12);
        assert_eq!(component.properties["THERM_COND"], 0.0);
        assert_eq!(component.properties["THETA_JB"], 0.2);
        assert_eq!(component.properties["THETA_JC"], 5.1);
    }

    #[test]
    fn test_electrical_component_2() {
        let input = ".ELECTRICAL\r\nGLOB_FID_60R140  \"GLOB_FID_GLOB_FID_60R140_GLOB F\"  THOU         2.0\r\n0         0.0         0.0       0.000\r\n0        70.0         0.0     360.000\r\n.END_ELECTRICAL\r\n";

        let (remaining, component) = electrical_component(input).unwrap();

        let expected = ElectricalComponent {
            geometry_name: "GLOB_FID_60R140".to_string(),
            part_number: "GLOB_FID_GLOB_FID_60R140_GLOB F".to_string(),
            units: "THOU".to_string(),
            height: 2.0,
            outline: vec![
                Point {
                    loop_label: 0,
                    x: 0.0,
                    y: 0.0,
                    angle: 0.0,
                },
                Point {
                    loop_label: 0,
                    x: 70.0,
                    y: 0.0,
                    angle: 360.0,
                },
            ],
            properties: HashMap::new(),
        };
        assert_eq!(remaining, "");
        assert_eq!(component.geometry_name, expected.geometry_name);
    }
    #[test]
    fn test_mechanical_component() {
        let input = ".MECHANICAL
cs13_a pn-cap THOU 150.0
0 -55.0 55.0 0.0
0 -55.0 -55.0 0.0
0 135.0 -55.0 0.0
0 135.0 -80.0 0.0
0 565.0 -80.0 0.0
0 565.0 -55.0 0.0
.END_MECHANICAL";
        let (remaining, component) = mechanical_component(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(component.geometry_name, "cs13_a");
        assert_eq!(component.part_number, "pn-cap");
        assert_eq!(component.units, "THOU");
        assert_eq!(component.height, 150.0);
        assert_eq!(component.outline.len(), 6);
        assert_eq!(
            component.outline[0],
            Point {
                loop_label: 0,
                x: -55.0,
                y: 55.0,
                angle: 0.0
            }
        );
        assert_eq!(
            component.outline[4],
            Point {
                loop_label: 0,
                x: 565.0,
                y: -80.0,
                angle: 0.0
            }
        );
    }
}
