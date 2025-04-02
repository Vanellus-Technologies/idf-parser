use crate::primitives::{Point, point, ws};
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::{is_not, tag};
use nom::multi::{many_m_n, many0};
use nom::number::complete::float;
use nom::sequence::{delimited, preceded, terminated};

struct ComponentProperties {
    capacitance: Option<f32>,                       // microfarads
    resistance: Option<f32>,                        // ohms
    tolerance: Option<f32>,                         // percent deviation
    operating_power: Option<f32>,                   // milliwatts
    maximum_power: Option<f32>,                     // milliwatts
    thermal_conductivity: Option<f32>,              // watt / meter °C
    junction_board_thermal_resistance: Option<f32>, // °C / watt
    junction_case_thermal_resistance: Option<f32>,  // °C / watt
}

/// Represents an electrical component in the IDF format.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=31
pub struct ElectricalComponent {
    pub geometry_name: String,
    part_number: String,
    units: String,
    height: f32,
    outline: Vec<Point>,
    properties: ComponentProperties,
}

/// Represents a mechanical component in the IDF format.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=34
pub struct MechanicalComponent {
    pub geometry_name: String,
    part_number: String,
    units: String,
    height: f32,
    outline: Vec<Point>,
}

fn get_component<'a>(input: &'a str, prop: &str) -> IResult<&'a str, Option<f32>> {
    let (remaining, capacitance) = many_m_n(
        0,
        1,
        preceded(tag(format!("PROP {}", prop).as_str()), ws(float)),
    )
    .parse(input)?;
    if capacitance.len() == 0 {
        Ok((remaining, None))
    } else {
        Ok((remaining, Some(capacitance[0])))
    }
}

/// Parses the properties of an electrical or mechanical component from the input string.
fn properties(input: &str) -> IResult<&str, ComponentProperties> {
    let (remaining, capacitance) = get_component(input, "CAPACITANCE")?;
    let (remaining, resistance) = get_component(remaining, "RESISTANCE")?;
    let (remaining, tolerance) = get_component(remaining, "TOLERANCE")?;
    let (remaining, operating_power) = get_component(remaining, "POWER_OPR")?;
    let (remaining, maximum_power) = get_component(remaining, "POWER_MAX")?;
    let (remaining, thermal_conductivity) = get_component(remaining, "THERM_COND")?;
    let (remaining, junction_board_thermal_resistance) = get_component(remaining, "THETA_JB")?;
    let (remaining, junction_case_thermal_resistance) = get_component(remaining, "THETA_JC")?;

    let component = ComponentProperties {
        capacitance,
        resistance,
        tolerance,
        operating_power,
        maximum_power,
        thermal_conductivity,
        junction_board_thermal_resistance,
        junction_case_thermal_resistance,
    };

    Ok((remaining, component))
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
    let (remaining, (geometry_name, part_number, units, height, outline, properties)) = (
        delimited(ws(tag(".ELECTRICAL")), is_not(" "), tag(" ")), // geometry name
        ws(is_not(" ")),                                          // part number
        ws(is_not(" ")),                                          // units
        ws(float),                                                // height
        many0(ws(point)),                                         // outline
        terminated(properties, tag(".END_ELECTRICAL")),           // outline
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
    let (remaining, (geometry_name, part_number, units, height, outline)) = (
        delimited(ws(tag(".MECHANICAL")), is_not(" "), tag(" ")), // geometry name
        ws(is_not(" ")),                                          // part number
        ws(is_not(" ")),                                          // units
        ws(float),                                                // height
        terminated(many0(ws(point)), tag(".END_MECHANICAL")),     // outline
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
    fn test_component_placement() {
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
.END_ELECTRICAL";
        let (remaining, component) = electrical_component(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(component.geometry_name, "cs13_a");
        assert_eq!(component.part_number, "pn-cap");
        assert_eq!(component.units, "THOU");
        assert_eq!(component.height, 150.0);
        assert_eq!(component.outline.len(), 13);
        assert_eq!(component.properties.capacitance, Some(100.0));
        assert_eq!(component.properties.tolerance, Some(5.0));
        assert_eq!(component.properties.resistance, None);
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
                label: 0,
                x: -55.0,
                y: 55.0,
                angle: 0.0
            }
        );
    }
}
