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
//! let board = parse_board_file("src/board.emn").unwrap();
//! let panel = parse_board_file("src/panel.emn").unwrap();
//! let library = parse_library_file("src/library.emp").unwrap();
//! ```

pub mod board;
pub mod component_placement;
pub mod components;
pub mod drilled_holes;
pub mod headers;
mod library;
pub mod notes;
mod outlines;
pub mod primitives;
pub mod point;

/// Take in the path a board or panel .emn file and return a Board struct.
pub fn parse_board_file(file_path: &str) -> Result<board::BoardPanel, String> {
    if !file_path.ends_with(".emn") {
        return Err("Board and panel files must end with .emn.".to_string());
    }
    let file = std::fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let result = board::parse_board_or_panel(&file);

    match result {
        Ok(board) => Ok(board),
        Err(e) => Err(format!("Failed to parse board file: {}", e)),
    }
}

/// Take in the path a library .emp file and return a Library struct.
pub fn parse_library_file(file_path: &str) -> Result<library::Library, String> {
    if !file_path.ends_with(".emp") {
        return Err("Library files must end with .emp.".to_string());
    }

    let file = std::fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let result = library::parse_library(&file);

    match result {
        Ok(library) => Ok(library),
        Err(e) => Err(format!("Failed to parse library file: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_board_file() {
        parse_board_file("src/board.emn").unwrap();
    }
    #[test]
    fn test_parse_library_file() {
        parse_library_file("src/library.emp").unwrap();
    }
}
