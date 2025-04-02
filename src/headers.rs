use nom::branch::alt;
use nom::sequence::{delimited, preceded, terminated};

use crate::primitives::ws;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::{is_not, tag};

const PANEL: &str = "PANEL_FILE";
const LIBRARY: &str = "LIBRARY_FILE";
const BOARD: &str = "BOARD_FILE";

pub struct LibraryHeader {
    pub version: u32, // which IDF version, should be 3.0
    pub system_id: String,
    pub date: String, // I don't care about decomposing this for now
    pub file_version: u32,
}
pub struct BoardHeader {
    pub version: u32, // which IDF version, should be 3.0
    pub system_id: String,
    pub date: String, // I don't care about decomposing this for now
    pub file_version: u32,
    pub board_name: String,
    pub units: String,
}

fn header_start(input: &str) -> IResult<&str, &str> {
    ws(tag(".HEADER")).parse(input)
}
fn header_end(input: &str) -> IResult<&str, &str> {
    ws(tag(".END_HEADER")).parse(input)
}

fn board_name(input: &str) -> IResult<&str, &str> {
    terminated(is_not(" "), tag(" ")).parse(input)
}

fn header_metadata(input: &str) -> IResult<&str, (u32, String, String, u32)> {
    let (remaining, (_file_type, version, system_id, date, file_version)) = (
        ws(alt((tag(PANEL), tag(LIBRARY), tag(BOARD)))), // file type
        terminated(tag("3"), tag(".0")),                 // version
        delimited(tag(" \""), is_not("\""), tag("\"")),  // system id
        delimited(tag(" "), is_not(" "), tag(" ")),      // date
        ws(is_not("\n")),                                // file version
    )
        .parse(input)?;

    Ok((
        remaining,
        (
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
/// use idf_parser::headers::{parse_board_header, BoardHeader};
/// let input = ".HEADER
/// BOARD_FILE 3.0 \"Sample File Generator\" 10/22/96.16:02:44 1
/// sample_board THOU
/// .END_HEADER";
///
/// let (remaining, header) = parse_board_header(input).unwrap();
/// assert_eq!(header.units, "THOU");
/// ```
pub fn parse_board_header(input: &str) -> IResult<&str, BoardHeader> {
    let (remaining, (version, system_id, date, file_version)) =
        preceded(header_start, header_metadata).parse(input)?;

    let (remaining, (board_name, units)) =
        terminated(ws((board_name, alt((tag("THOU"), tag("MM"))))), header_end).parse(remaining)?;

    let header = BoardHeader {
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
    let (remaining, (version, system_id, date, file_version)) =
        delimited(header_start, header_metadata, header_end).parse(input)?;

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
        let (remaining, (version, system_id, date, file_version)) = header_metadata(input).unwrap();
        assert_eq!(remaining, "");
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

        let (remaining, header) = parse_board_header(input).unwrap();
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
