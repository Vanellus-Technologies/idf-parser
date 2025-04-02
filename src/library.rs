use crate::components::{
    ElectricalComponent, MechanicalComponent, electrical_component, mechanical_component,
};
use crate::headers::{LibraryHeader, library_header};
use nom::multi::many0;
use nom::{IResult, Parser};

pub struct Library {
    header: LibraryHeader,
    electrical_components: Vec<ElectricalComponent>,
    mechanical_components: Vec<MechanicalComponent>,
}

/// Parses a library emp file which contains detail on electrical and mechanical components.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=29
pub(crate) fn parse_library(input: &str) -> IResult<&str, Library> {
    let (remaining, (header, electrical_components, mechanical_components)) = (
        library_header,
        many0(electrical_component),
        many0(mechanical_component),
    )
        .parse(input)?;

    let library = Library {
        header,
        electrical_components,
        mechanical_components,
    };

    Ok((remaining, library))
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let (remaining, library) = parse_library(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(library.electrical_components.len(), 5);
    }
}
