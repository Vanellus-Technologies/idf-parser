//! The crate parses [IDF (Intermediate Data Format)](https://en.wikipedia.org/wiki/Intermediate_Data_Format) files, specifically the IDF 3.0 format used for PCB design data exchange.
//! It can parse board and panel .emn files, as well as library .emp files.
//!
//! [The IDF V3 specification.](http://www.simplifiedsolutionsinc.com/images/idf_v30_spec.pdf)
//!
//! # Example
//! ```
//! use idf_parser::parse_board_file;
//! use idf_parser::parse_library_file;
//!
//! let board = parse_board_file("path/to/board.emn").unwrap();
//! let library = parse_library_file("path/to/library.emp").unwrap();
//! ```
//!
//!

pub mod board;
pub mod component_placement;
pub mod components;
pub mod drilled_holes;
pub mod headers;
mod library;
pub mod notes;
mod outlines;
pub mod primitives;

/// Take in the path a board or panel .emn file and return a Board struct.
pub fn parse_board_file(file_path: &str) -> Result<board::Board, String> {
    if !file_path.ends_with(".emn") {
        return Err("Board and panel files must end with .emn.".to_string());
    }

    let file = std::fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let (remaining, board) = board::board(&file).map_err(|e| e.to_string())?;
    if !remaining.is_empty() {
        return Err(format!("Unparsed data remaining: {}", remaining));
    }
    Ok(board)
}

pub fn parse_library_file(file_path: &str) -> Result<library::Library, String> {
    if !file_path.ends_with(".emp") {
        return Err("Library files must end with .emp.".to_string());
    }

    let file = std::fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let (remaining, library) = library::library(&file).map_err(|e| e.to_string())?;
    if !remaining.is_empty() {
        return Err(format!("Unparsed data remaining: {}", remaining));
    }
    Ok(library)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_board_file() {
        let file_path = "src/board.emn";
        let board = parse_board_file(file_path).unwrap();
    }
    #[test]
    fn test_parse_library_file() {
        let file_path = "src/library.emp";
        let library = parse_library_file(file_path).unwrap();
    }
}
