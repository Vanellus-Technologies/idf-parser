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
//! let board = parse_board_file("src/test_files/board.emn").unwrap();
//! let panel = parse_board_file("src/test_files/panel.emn").unwrap();
//! let library = parse_library_file("src/test_files/library.emp").unwrap();
//! ```

use crate::board::BoardPanel;
use crate::library::Library;

pub mod board;
pub mod component_placement;
pub mod components;
pub mod drilled_holes;
pub mod headers;
pub mod library;
pub mod notes;
mod outlines;
pub mod point;
pub mod primitives;
mod validation;

/// Take in the path a board or panel .emn file and return a Board struct.
pub fn parse_board_file(file_path: &str) -> Result<BoardPanel, String> {
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
pub fn parse_library_file(file_path: &str) -> Result<Library, String> {
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

/// Parse an optional panel file, library file, and 1 or more board files and validate them.
///
/// An assembly is either a single board and a library file, or a panel file,
/// 1 or more board files and a library file.
/// Here we parse all the provided files and check that all board and component references are valid.
fn parse_assembly(
    panel_file: Option<&str>,
    library_file: &str,
    board_files: Vec<&str>,
) -> Result<(Option<BoardPanel>, Library, Vec<BoardPanel>), String> {
    let mut boards = Vec::new();

    let panel = match panel_file {
        Some(file) => Some(parse_board_file(file)?),
        None => None,
    };

    let library = parse_library_file(library_file)?;

    for board_file in board_files {
        boards.push(parse_board_file(board_file)?);
    }

    for board in &boards {
        validation::library_references_valid(&library, board)?;
    }

    if let Some(panel) = &panel {
        validation::panel_references_valid(panel, &boards)?;
    }

    Ok((panel, library, boards))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_spec_board_file() {
        parse_board_file("src/test_files/board.emn").unwrap();
    }
    #[test]
    fn test_parse_isol_board_file() {
        parse_board_file("src/test_files/ISOL.emn").unwrap();
    }

    #[test]
    fn test_parse_beaglebone_board_file() {
        parse_board_file("src/test_files/beaglebone.emn").unwrap();
    }

    #[test]
    fn test_parse_ain_board_file() {
        parse_board_file("src/test_files/ain.emn").unwrap();
    }

    #[test]
    fn test_parse_esp_board_file() {
        parse_board_file("src/test_files/esp.emn").unwrap();
    }

    #[test]
    fn test_parse_library_file() {
        parse_library_file("src/test_files/library.emp").unwrap();
    }
    #[test]
    fn test_parse_isol_library_file() {
        parse_library_file("src/test_files/ISOL.emp").unwrap();
    }

    #[test]
    fn test_parse_beaglebone_library_file() {
        parse_library_file("src/test_files/beaglebone.emp").unwrap();
    }

    #[test]
    fn test_parse_ain_library_file() {
        parse_library_file("src/test_files/ain.emp").unwrap();
    }

    #[test]
    fn test_parse_esp_library_file() {
        parse_library_file("src/test_files/esp.emp").unwrap();
    }

    #[test]
    fn test_parse_assembly() {
        let panel = Some("src/test_files/panel.emn");
        let library = "src/test_files/library.emp";
        let boards = vec!["src/test_files/board.emn"];

        let result = parse_assembly(panel, library, boards.clone());
        assert!(result.is_ok());

        // This panel file references a board that doesn't exist
        let invalid_panel = Some("src/test_files/invalid_panel.emn");

        let result = parse_assembly(invalid_panel, library, boards);
        assert!(result.is_err());
    }
}
