use crate::board::BoardPanel;
use crate::library::Library;
use std::collections::HashSet;

/// Validate that a Library struct contains all the components referenced in board struct.
///
/// A board file can reference a number of components which have properties and outlines defined in
/// the library file. To confirm that the board file is valid, we need to check that every
/// referenced component is defined in the library file.
///
/// For example this placement section of a board file references component "cs13_a".
/// ```text
/// .PLACEMENT
/// cs13_a pn-cap C1
/// 4000.0 1000.0 100.0 0.0 TOP PLACED
/// .END_PLACEMENT
/// ```
///
/// We thus expect there to be a corresponding ".ELECTRICAL"
/// or ".MECHANICAL" section in the library file which defines the component, for example:
/// ```text
/// .ELECTRICAL
/// cs13_a pn-cc1210 THOU 67.0
/// 0 -40.0 56.0 0.0
/// PROP CAPACITANCE 0.1
/// .END_ELECTRICAL
/// ```
pub(crate) fn library_references_valid(
    library: &Library,
    board: &BoardPanel,
) -> Result<(), String> {
    let mut board_components = HashSet::new();

    // Collect all component references from the board
    for component in board.component_placements.iter() {
        if component.reference_designator == "BOARD" {
            continue;
        }
        board_components.insert(component.package_name.clone());
    }

    let mut library_components = HashSet::new();
    // Collect all component references from the library
    for component in library.electrical_components.iter() {
        library_components.insert(component.geometry_name.clone());
    }
    for component in library.mechanical_components.iter() {
        library_components.insert(component.geometry_name.clone());
    }

    // Check if all board components are present in the library
    for component in board_components.iter() {
        if !library_components.contains(component) {
            return Err(format!(
                "Component {} referenced in board not found in library.",
                component
            ));
        }
    }

    Ok(())
}

/// Validate that all the boards referenced in a panel file are present.
///
/// Any entry in .PLACEMENT section of the panel file with a reference designator of "BOARD" is
/// considered a reference to another board. The package name of that entry is the name of the
/// board being referenced.
///
/// For example this placement section of a panel file references board "board1".
/// ```text
/// .PLACEMENT
/// board1 pn-board BOARD
/// 1700.0 3300.0 0.0 0.0 TOP MCAD
/// .END_PLACEMENT
/// ```
///
/// We thus expect there to be a corresponding ".HEADER" section in on of the board files which
/// defines the board, for example:
/// ```text
/// .HEADER
/// BOARD_FILE 3.0 "Sample File Generator" 10/22/96.16:02:44 1
/// board1 THOU
/// .END_HEADER
/// ```
pub(crate) fn panel_references_valid(
    panel: &BoardPanel,
    boards: &[BoardPanel],
) -> Result<(), String> {
    let mut referenced_boards = HashSet::new();

    // Collect all board references from the panel
    for placement in panel.component_placements.iter() {
        if placement.reference_designator == "BOARD" {
            referenced_boards.insert(placement.package_name.clone());
        }
    }

    // Check if all referenced boards are present
    for ref_board in referenced_boards.iter() {
        if !boards
            .iter()
            .any(|board| board.header.board_name == *ref_board)
        {
            return Err(format!(
                "Board {} referenced in panel not found.",
                ref_board
            ));
        }
    }

    Ok(())
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::component_placement::ComponentPlacement;
    use crate::parse_board_file;
    use crate::parse_library_file;

    #[test]
    fn test_library_references_valid() {
        let library = parse_library_file("src/test_files/library.emp").unwrap();
        let board = parse_board_file("src/test_files/board.emn").unwrap();

        let result = library_references_valid(&library, &board);

        result.unwrap();

        // Add dummy component to board struct
        let dummy_component = ComponentPlacement {
            package_name: "dummy_component".to_string(),
            part_number: "dummy_part".to_string(),
            reference_designator: "DUMMY".to_string(),
            x: 0.0,
            y: 0.0,
            mounting_offset: 0.0,
            rotation_angle: 0.0,
            board_side: "TOP".to_string(),
            placement_status: "PLACED".to_string(),
        };
        let mut board = board.clone();
        board.component_placements.push(dummy_component);

        // Test with dummy component
        let result = library_references_valid(&library, &board);
        assert!(result.is_err());
    }
    #[test]
    fn test_panel_references_valid() {
        let panel = parse_board_file("src/test_files/panel.emn").unwrap();
        let boards = vec![parse_board_file("src/test_files/board.emn").unwrap()];

        let result = panel_references_valid(&panel, &boards);

        result.unwrap();

        // Add dummy component placement to panel struct
        let dummy_component = ComponentPlacement {
            package_name: "dummy_board".to_string(),
            part_number: "dummy_part".to_string(),
            reference_designator: "BOARD".to_string(),
            x: 0.0,
            y: 0.0,
            mounting_offset: 0.0,
            rotation_angle: 0.0,
            board_side: "TOP".to_string(),
            placement_status: "PLACED".to_string(),
        };
        let mut panel = panel.clone();
        panel.component_placements.push(dummy_component);
        // Test with dummy component
        let result = panel_references_valid(&panel, &boards);
        assert!(result.is_err());
    }
}
