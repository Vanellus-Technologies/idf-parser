use nom::branch::alt;
use nom::sequence::{delimited, preceded, terminated};

use nom::bytes::complete::{is_not, tag};
use nom::IResult;
use nom::Parser;
use crate::idf_v3::primitives;

const PANEL: &str = "PANEL_FILE";
const LIBRARY: &str = "LIBRARY_FILE";
const BOARD: &str = "BOARD_FILE";

pub struct LibraryHeader {
    version: u32, // which IDF version, should be 3.0
    system_id: String,
    date: String, // I don't care about decomposing this for now
    file_version: u32,
}
pub struct BoardHeader {
    version: u32, // which IDF version, should be 3.0
    system_id: String,
    date: String, // I don't care about decomposing this for now
    file_version: u32,
    board_name: String,
    units: String,
}

fn header_start(input: &str) -> IResult<&str, &str> {
    tag(".HEADER\n")(input)
}
fn header_end(input: &str) -> IResult<&str, &str> {
    tag("\n.END_HEADER\n")(input)
}

fn board_name(input: &str) -> IResult<&str, &str> {
    terminated(is_not(" "), tag(" ")).parse(input)
}

fn header_metadata(input: &str) -> IResult<&str, (u32, String, String, u32)> {
    let (remaining, (_file_type, version, system_id, date, file_version)) = (
        alt((tag(PANEL), tag(LIBRARY), tag(BOARD))), // file type
        delimited(tag(" "), tag("3"), tag(".0")),    // version
        delimited(tag(" \""), is_not("\""), tag("\"")), // system id
        delimited(tag(" "), is_not(" "), tag(" ")),  // date
        is_not("\n"),                                // file version
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

pub fn board_header(input: &str) -> IResult<&str, BoardHeader> {
    let (remaining, (version, system_id, date, file_version)) =
        preceded(header_start, header_metadata).parse(input)?;

    let (remaining, (board_name, units)) =
        delimited(tag("\n"), (board_name, primitives::units), header_end).parse(remaining)?;

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

pub fn library_header(input: &str) -> IResult<&str, LibraryHeader> {
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
        let (remaining, (version, system_id, date, file_version)) =
            header_metadata(input).unwrap();
        assert_eq!(remaining, "\n");
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

        let (remaining, header) = board_header(input).unwrap();
        assert_eq!(remaining, " other nonsense");
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

        let (remaining, header) = library_header(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(header.version, 3);
        assert_eq!(header.system_id, "Sample File Generator");
        assert_eq!(header.date, "10/22/96.16:41:37");
        assert_eq!(header.file_version, 1);
    }
}
