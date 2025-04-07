use crate::primitives::{quote_string, ws};
use crate::{parse_section, ws_separated};
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::number::complete::float;
use nom::sequence::delimited;

/// A board or panel file note.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=26
///
/// This section contains notes for the design that can be displayed in the receiving system, to allow
/// the electrical and mechanical designers to communicate additional information about the design
/// entities beyond that conveyed by the entities themselves. Notes are not intended to be used for
/// rigorous translations of text such as for transferring manufacturing drawings. The association of a
/// note to its subject is inferred by its location.
#[derive(Debug, PartialEq, Clone, Default, PartialOrd)]
pub struct Note {
    pub x: f32,
    pub y: f32,
    pub text_height: f32,
    pub test_string_physical_length: f32,
    pub text: String,
}

fn note(input: &str) -> IResult<&str, Note> {
    let (remaining, (x, y, text_height, test_string_physical_length, text)) = ws_separated!((
        float,        // x
        float,        // y
        float,        // text height
        float,        // test string physical length
        quote_string  // text
    ))
    .parse(input)?;
    let note = Note {
        x,
        y,
        text_height,
        test_string_physical_length,
        text: text.to_string(),
    };
    Ok((remaining, note))
}

/// Parses a section of notes from the input string.
///
/// # Example
/// ```
/// use idf_parser::notes::parse_notes_section;
/// let input = ".NOTES
/// 3500.0 3300.0 75.0 2500.0 \"This component rotated 14 degrees\"
/// 400.0 4400.0 75.0 3200.0 \"Component height limited by enclosure latch\"
/// .END_NOTES";
///
/// let (_remaining, notes) = parse_notes_section(input).unwrap();
/// assert_eq!(notes.len(), 2);
/// ```
pub fn parse_notes_section(input: &str) -> IResult<&str, Vec<Note>> {
    parse_section!("NOTES", many1(ws(note))).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_note() {
        let input = "3500.0 3300.0 75.0 2500.0 \"This component rotated 14 degrees\"";

        let expected = Note {
            x: 3500.0,
            y: 3300.0,
            text_height: 75.0,
            test_string_physical_length: 2500.0,
            text: "This component rotated 14 degrees".to_string(),
        };
        let (_remaining, note) = note(input).unwrap();
        assert_eq!(note, expected);
    }
    #[test]
    fn test_notes_section() {
        let input = ".NOTES
3500.0 3300.0 75.0 2500.0 \"This component rotated 14 degrees\"
400.0 4400.0 75.0 3200.0 \"Component height limited by enclosure latch\"
1800.0 300.0 75.0 1700.0 \"Do not move connectors!\"
.END_NOTES";

        let expected = vec![
            Note {
                x: 3500.0,
                y: 3300.0,
                text_height: 75.0,
                test_string_physical_length: 2500.0,
                text: "This component rotated 14 degrees".to_string(),
            },
            Note {
                x: 400.0,
                y: 4400.0,
                text_height: 75.0,
                test_string_physical_length: 3200.0,
                text: "Component height limited by enclosure latch".to_string(),
            },
            Note {
                x: 1800.0,
                y: 300.0,
                text_height: 75.0,
                test_string_physical_length: 1700.0,
                text: "Do not move connectors!".to_string(),
            },
        ];
        let (_remaining, notes) = parse_notes_section(input).unwrap();
        assert_eq!(notes, expected);
    }

    #[test]
    fn test_notes_more() {
        let input = ".NOTES
1800.0 300.0 75.0 1700.0 \"Do not move connectors!\"
.END_NOTES";

        let expected = vec![Note {
            x: 1800.0,
            y: 300.0,
            text_height: 75.0,
            test_string_physical_length: 1700.0,
            text: "Do not move connectors!".to_string(),
        }];
        let (_remaining, notes) = parse_notes_section(input).unwrap();
        assert_eq!(notes, expected);
    }
}
