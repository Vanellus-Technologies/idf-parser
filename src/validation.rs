use crate::board::BoardPanel;
use crate::library::Library;
use std::collections::HashSet;

/// Validate that a Library struct contains all the components referenced in board struct.
pub(crate) fn library_references_valid(
    library: &Library,
    board: &BoardPanel,
) -> Result<(), String> {
    let mut board_components = HashSet::new();

    // Collect all component references from the board
    for component in board.component_placements.iter() {
        // If not BOARD or NOREFDES
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

/// Validate that all the boards referenced in a panel are present.
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
        let library = parse_library_file("src/library.emp").unwrap();
        let board = parse_board_file("src/board.emn").unwrap();

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
        let panel = parse_board_file("src/panel.emn").unwrap();
        let boards = vec![parse_board_file("src/board.emn").unwrap()];

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
