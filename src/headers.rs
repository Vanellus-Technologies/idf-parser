use nom::branch::alt;
use nom::sequence::{delimited, terminated};

use crate::primitives::ws;
use crate::{parse_section, ws_separated};
use nom::Err::Error;
use nom::Parser;
use nom::bytes::complete::{is_not, tag};
use nom::error::ErrorKind;
use nom::{IResult, error};

#[derive(PartialEq, Debug, Clone)]
pub struct LibraryHeader {
    pub version: u32, // which IDF version, should be 3.0
    pub system_id: String,
    pub date: String, // We don't decompose this for now
    pub file_version: u32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct BoardPanelHeader {
    pub file_type: String, // BOARD_FILE or PANEL_FILE
    pub version: u32,      // which IDF version, should be 3.0
    pub system_id: String,
    pub date: String, // We don't decompose this for now
    pub file_version: u32,
    pub board_name: String,
    pub units: String,
}

/// Parses the first line of the header section.
fn header_metadata(input: &str) -> IResult<&str, (String, u32, String, String, u32)> {
    let (remaining, (file_type, version, system_id, date, file_version)) = (
        ws(alt((
            tag("PANEL_FILE"),
            tag("LIBRARY_FILE"),
            tag("BOARD_FILE"),
        ))), // file type
        terminated(tag("3"), tag(".0")),                // version
        delimited(tag(" \""), is_not("\""), tag("\"")), // system id
        delimited(tag(" "), is_not(" "), tag(" ")),     // date
        ws(is_not("\n")),                               // file version
    )
        .parse(input)?;

    Ok((
        remaining,
        (
            file_type.to_string(),
            version.parse::<u32>().unwrap(),
            system_id.to_string(),
            date.to_string(),
            file_version.parse::<u32>().unwrap(),
        ),
    ))
}

/// Parses the header of a board or panel emn file.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=8
///
/// # Example
///
/// ```
/// use idf_parser::headers::{parse_board_or_panel_header, BoardPanelHeader};
/// let input = ".HEADER
/// BOARD_FILE 3.0 \"Sample File Generator\" 10/22/96.16:02:44 1
/// sample_board THOU
/// .END_HEADER";
///
/// let (remaining, header) = parse_board_or_panel_header(input).unwrap();
/// assert_eq!(header.units, "THOU");
/// ```
pub fn parse_board_or_panel_header(input: &str) -> IResult<&str, BoardPanelHeader> {
    let (remaining, (metadata, (board_name, units))) = parse_section!(
        "HEADER",
        (
            header_metadata,
            ws_separated!((is_not(" "), alt((tag("THOU"), tag("MM"))))),
        )
    )
    .parse(input)?;

    let (file_type, version, system_id, date, file_version) = metadata;

    let header = BoardPanelHeader {
        file_type,
        version,
        system_id,
        date,
        file_version,
        board_name: board_name.to_string(),
        units: units.to_string(),
    };
    Ok((remaining, header))
}

/// Parses the header of a library emn file.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=29
///
/// # Example
///
/// ```
/// use idf_parser::headers::{parse_library_header, LibraryHeader};
///
/// let input = ".HEADER
/// LIBRARY_FILE 3.0 \"Sample File Generator\" 10/22/96.16:41:37 1
/// .END_HEADER\n";
///
/// let (remaining, header) = parse_library_header(input).unwrap();
/// assert_eq!(header.date, "10/22/96.16:41:37");
/// ```
pub fn parse_library_header(input: &str) -> IResult<&str, LibraryHeader> {
    let (remaining, (file_type, version, system_id, date, file_version)) =
        parse_section!("HEADER", header_metadata).parse(input)?;

    if file_type != "LIBRARY_FILE" {
        return Err(Error(error::Error::new(input, ErrorKind::Tag)));
    }

    let header = LibraryHeader {
        version,
        system_id,
        date,
        file_version,
    };
    Ok((remaining, header))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_header_metadata() {
        let input = "BOARD_FILE 3.0 \"Sample File Generator\" 10/22/96.16:02:44 1\n";
        let (remaining, (file_type, version, system_id, date, file_version)) =
            header_metadata(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(file_type, "BOARD_FILE");
        assert_eq!(version, 3);
        assert_eq!(system_id, "Sample File Generator");
        assert_eq!(date, "10/22/96.16:02:44");
        assert_eq!(file_version, 1);
    }

    #[test]
    fn test_parse_board_header() {
        let input = ".HEADER
BOARD_FILE 3.0 \"Sample File Generator\" 10/22/96.16:02:44 1
sample_board THOU
.END_HEADER\n other nonsense";

        let (remaining, header) = parse_board_or_panel_header(input).unwrap();
        assert_eq!(remaining, "other nonsense");
        assert_eq!(header.version, 3);
        assert_eq!(header.system_id, "Sample File Generator");
        assert_eq!(header.date, "10/22/96.16:02:44");
        assert_eq!(header.file_version, 1);
        assert_eq!(header.board_name, "sample_board");
        assert_eq!(header.units, "THOU");
    }
    #[test]
    fn test_parse_library_header() {
        let input = ".HEADER
LIBRARY_FILE 3.0 \"Sample File Generator\" 10/22/96.16:41:37 1
.END_HEADER\n";

        let (remaining, header) = parse_library_header(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(header.version, 3);
        assert_eq!(header.system_id, "Sample File Generator");
        assert_eq!(header.date, "10/22/96.16:41:37");
        assert_eq!(header.file_version, 1);
    }
}
