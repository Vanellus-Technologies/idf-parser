use crate::component_placement::{ComponentPlacement, parse_component_placement_section};
use crate::drilled_holes::{Hole, parse_drilled_holes_section};
use crate::headers::{BoardPanelHeader, parse_board_or_panel_header};
use crate::notes::{Note, parse_notes_section};
use crate::outlines::{
    BoardPanelOutline, OtherOutline, PlacementGroupArea, PlacementKeepout, PlacementOutline,
    RoutingKeepout, RoutingOutline, ViaKeepout, parse_board_panel_outline, parse_other_outline,
    parse_placement_group_area, parse_placement_keepout, parse_placement_outline,
    parse_routing_keepout, parse_routing_outline, parse_via_keepout,
};
use nom::Parser;
use nom::multi::{many_m_n, many0};

/// Represents a board or panel file in the IDF format.
#[derive(Clone, Debug, PartialEq)]
pub struct BoardPanel {
    header: BoardPanelHeader,
    outline: BoardPanelOutline,
    other_outlines: Vec<OtherOutline>,
    routing_outlines: Vec<RoutingOutline>,
    placement_outlines: Vec<PlacementOutline>,
    routing_keepouts: Vec<RoutingKeepout>,
    via_keepouts: Vec<ViaKeepout>,
    placement_keepouts: Vec<PlacementKeepout>,
    placement_group_areas: Vec<PlacementGroupArea>,
    drilled_holes: Vec<Hole>,
    notes: Vec<Note>,
    component_placements: Vec<ComponentPlacement>,
}

/// Parse the content of a board or panel .emn file into a Board struct.
/// File specification: http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=8
pub fn parse_board_or_panel(input: &str) -> Result<BoardPanel, nom::Err<nom::error::Error<&str>>> {
    let (
        remaining,
        (
            header,
            outline,
            other_outlines,
            routing_outlines,
            placement_outlines,
            routing_keepouts,
            via_keepouts,
            placement_keepouts,
            placement_group_areas,
            drilled_holes,
            wrapped_notes,
            component_placements,
        ),
    ) = (
        parse_board_or_panel_header,
        parse_board_panel_outline,
        // expect there to be between 0 and n sections
        many0(parse_other_outline),
        many0(parse_routing_outline),
        many0(parse_placement_outline),
        many0(parse_routing_keepout),
        many0(parse_via_keepout),
        many0(parse_placement_keepout),
        many0(parse_placement_group_area),
        // expect one section
        parse_drilled_holes_section,
        // expect either 0 or 1 sections
        many_m_n(0, 1, parse_notes_section),
        // expect one section
        parse_component_placement_section,
    )
        .parse(input)
        .unwrap();

    // Unwrap the notes section, if it exists. We expect there to be either 0 or 1 sections.
    let notes: Vec<Note> = if wrapped_notes.len() == 1 {
        wrapped_notes[0].clone()
    } else if wrapped_notes.is_empty() {
        Vec::new()
    } else {
        panic!(
            "Expected either 1 or no notes sections, but found: {}",
            wrapped_notes.len()
        );
    };

    let board_panel = BoardPanel {
        header,
        outline,
        other_outlines,
        routing_outlines,
        placement_outlines,
        routing_keepouts,
        via_keepouts,
        placement_keepouts,
        placement_group_areas,
        drilled_holes,
        notes,
        component_placements,
    };

    // Check if there is any unparsed data remaining
    if !remaining.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            remaining,
            nom::error::ErrorKind::Eof,
        )));
    } else {
        Ok(board_panel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;
    #[test]
    fn test_parse_board() {
        let input = ".HEADER
BOARD_FILE 3.0 \"Sample File Generator\" 10/22/96.16:02:44 1
sample_board THOU
.END_HEADER
.BOARD_OUTLINE MCAD
62.0
0 5030.5 -120.0 0.0
1 3000.0 2350.0 360.0
.END_BOARD_OUTLINE
.ROUTE_OUTLINE ECAD
ALL
0 5112.5 150.0 0.0
0 5112.5 150.0 0.0
.END_ROUTE_OUTLINE
.PLACE_OUTLINE MCAD
TOP 1000.0
0 5080.0 2034.9 0.0
0 5080.0 2034.9 0.0
.END_PLACE_OUTLINE
.PLACE_OUTLINE UNOWNED
BOTTOM 200.0
0 300.0 200.0 0.0
0 4800.0 200.0 0.0
.END_PLACE_OUTLINE
.ROUTE_KEEPOUT ECAD
ALL
0 2650.0 2350.0 0.0
0 3100.0 2350.0 360.0
.END_ROUTE_KEEPOUT
.PLACE_KEEPOUT MCAD
BOTH 0.0
0 2650.0 2350.0 0.0
0 3100.0 2350.0 360.0
.END_PLACE_KEEPOUT
.PLACE_KEEPOUT MCAD
TOP 300.0
0 3700.0 5000.0 0.0
0 3700.0 5000.0 0.0
.END_PLACE_KEEPOUT
.DRILLED_HOLES
30.0 1800.0 100.0 PTH J1 PIN ECAD
20.0 2000.0 1600.0 PTH BOARD VIA ECAD
93.0 5075.0 0.0 PTH BOARD MTG UNOWNED
93.0 0.0 4800.0 NPTH BOARD TOOL MCAD
.END_DRILLED_HOLES
.NOTES
1800.0 300.0 75.0 1700.0 \"Do not move connectors!\"
.END_NOTES
.PLACEMENT
cs13_a pn-cap C1
4000.0 1000.0 100.0 0.0 TOP PLACED
cc1210 pn-cc1210 C2
3000.0 3500.0 0.0 0.0 TOP PLACED
cc1210 pn-cc1210 C3
3200.0 1800.0 0.0 0.0 BOTTOM PLACED
.END_PLACEMENT";

        // Construct expected board
        let header = BoardPanelHeader {
            file_type: "BOARD_FILE".to_string(),
            version: 3,
            system_id: "Sample File Generator".to_string(),
            date: "10/22/96.16:02:44".to_string(),
            file_version: 1,
            board_name: "sample_board".to_string(),
            units: "THOU".to_string(),
        };

        let outline = BoardPanelOutline {
            owner: "MCAD".to_string(),
            thickness: 62.0,
            outline: vec![
                Point {
                    loop_label: 0,
                    x: 5030.5,
                    y: -120.0,
                    angle: 0.0,
                },
                Point {
                    loop_label: 1,
                    x: 3000.0,
                    y: 2350.0,
                    angle: 360.0,
                },
            ],
        };

        let routing_outlines = vec![RoutingOutline {
            owner: "ECAD".to_string(),
            routing_layers: "ALL".to_string(),
            outline: vec![
                Point {
                    loop_label: 0,
                    x: 5112.5,
                    y: 150.0,
                    angle: 0.0,
                },
                Point {
                    loop_label: 0,
                    x: 5112.5,
                    y: 150.0,
                    angle: 0.0,
                },
            ],
        }];

        let placement_outlines = vec![
            PlacementOutline {
                owner: "MCAD".to_string(),
                board_side: "TOP".to_string(),
                outline_height: 1000.0,
                outline: vec![
                    Point {
                        loop_label: 0,
                        x: 5080.0,
                        y: 2034.9,
                        angle: 0.0,
                    },
                    Point {
                        loop_label: 0,
                        x: 5080.0,
                        y: 2034.9,
                        angle: 0.0,
                    },
                ],
            },
            PlacementOutline {
                owner: "UNOWNED".to_string(),
                board_side: "BOTTOM".to_string(),
                outline_height: 200.0,
                outline: vec![
                    Point {
                        loop_label: 0,
                        x: 300.0,
                        y: 200.0,
                        angle: 0.0,
                    },
                    Point {
                        loop_label: 0,
                        x: 4800.0,
                        y: 200.0,
                        angle: 0.0,
                    },
                ],
            },
        ];

        let routing_keepouts = vec![RoutingKeepout {
            owner: "ECAD".to_string(),
            routing_layers: "ALL".to_string(),
            outline: vec![
                Point {
                    loop_label: 0,
                    x: 2650.0,
                    y: 2350.0,
                    angle: 0.0,
                },
                Point {
                    loop_label: 0,
                    x: 3100.0,
                    y: 2350.0,
                    angle: 360.0,
                },
            ],
        }];

        let placement_keepouts = vec![
            PlacementKeepout {
                owner: "MCAD".to_string(),
                board_side: "BOTH".to_string(),
                keepout_height: 0.0,
                outline: vec![
                    Point {
                        loop_label: 0,
                        x: 2650.0,
                        y: 2350.0,
                        angle: 0.0,
                    },
                    Point {
                        loop_label: 0,
                        x: 3100.0,
                        y: 2350.0,
                        angle: 360.0,
                    },
                ],
            },
            PlacementKeepout {
                owner: "MCAD".to_string(),
                board_side: "TOP".to_string(),
                keepout_height: 300.0,
                outline: vec![
                    Point {
                        loop_label: 0,
                        x: 3700.0,
                        y: 5000.0,
                        angle: 0.0,
                    },
                    Point {
                        loop_label: 0,
                        x: 3700.0,
                        y: 5000.0,
                        angle: 0.0,
                    },
                ],
            },
        ];

        let drilled_holes = vec![
            Hole {
                diameter: 30.0,
                x: 1800.0,
                y: 100.0,
                plating_style: "PTH".to_string(),
                associated_part: "J1".to_string(),
                hole_type: "PIN".to_string(),
                owner: "ECAD".to_string(),
            },
            Hole {
                diameter: 20.0,
                x: 2000.0,
                y: 1600.0,
                plating_style: "PTH".to_string(),
                associated_part: "BOARD".to_string(),
                hole_type: "VIA".to_string(),
                owner: "ECAD".to_string(),
            },
            Hole {
                diameter: 93.0,
                x: 5075.0,
                y: 0.0,
                plating_style: "PTH".to_string(),
                associated_part: "BOARD".to_string(),
                hole_type: "MTG".to_string(),
                owner: "UNOWNED".to_string(),
            },
            Hole {
                diameter: 93.0,
                x: 0.0,
                y: 4800.0,
                plating_style: "NPTH".to_string(),
                associated_part: "BOARD".to_string(),
                hole_type: "TOOL".to_string(),
                owner: "MCAD".to_string(),
            },
        ];

        let notes = vec![Note {
            x: 1800.0,
            y: 300.0,
            text_height: 75.0,
            test_string_physical_length: 1700.0,
            text: "Do not move connectors!".to_string(),
        }];

        let component_placements = vec![
            ComponentPlacement {
                package_name: "cs13_a".to_string(),
                part_number: "pn-cap".to_string(),
                reference_designator: "C1".to_string(),
                x: 4000.0,
                y: 1000.0,
                mounting_offset: 100.0,
                rotation_angle: 0.0,
                board_side: "TOP".to_string(),
                placement_status: "PLACED".to_string(),
            },
            ComponentPlacement {
                package_name: "cc1210".to_string(),
                part_number: "pn-cc1210".to_string(),
                reference_designator: "C2".to_string(),
                x: 3000.0,
                y: 3500.0,
                mounting_offset: 0.0,
                rotation_angle: 0.0,
                board_side: "TOP".to_string(),
                placement_status: "PLACED".to_string(),
            },
            ComponentPlacement {
                package_name: "cc1210".to_string(),
                part_number: "pn-cc1210".to_string(),
                reference_designator: "C3".to_string(),
                x: 3200.0,
                y: 1800.0,
                mounting_offset: 0.0,
                rotation_angle: 0.0,
                board_side: "BOTTOM".to_string(),
                placement_status: "PLACED".to_string(),
            },
        ];

        let expected_board = BoardPanel {
            header,
            outline,
            other_outlines: vec![],
            routing_outlines,
            placement_outlines,
            routing_keepouts,
            via_keepouts: vec![],
            placement_keepouts,
            placement_group_areas: vec![],
            drilled_holes,
            notes,
            component_placements,
        };

        let board = parse_board_or_panel(input).unwrap();
        assert_eq!(board, expected_board);
    }
    #[test]
    fn test_parse_panel() {
        let input = ".HEADER
PANEL_FILE 3.0 \"Sample File Generator\" 10/22/96.16:20:19 1
sample_panel THOU
.END_HEADER
.PANEL_OUTLINE MCAD
62.0
0 0.0 0.0 0.0
0 16000.0 0.0 0.0
.END_PANEL_OUTLINE
.PLACE_KEEPOUT MCAD
BOTTOM 0.0
0 13500.0 0.0 0.0
0 13500.0 12000.0 0.0
0 13500.0 0.0 0.0
.END_PLACE_KEEPOUT
.PLACE_KEEPOUT MCAD
BOTTOM 0.0
0 0.0 0.0 0.0
0 2200.0 0.0 0.0
0 2200.0 12000.0 0.0
0 0.0 12000.0 0.0
0 0.0 0.0 0.0
.END_PLACE_KEEPOUT
.DRILLED_HOLES
250.0 15500.0 11500.0 NPTH PANEL TOOL MCAD
250.0 500.0 500.0 NPTH PANEL TOOL MCAD
.END_DRILLED_HOLES
.PLACEMENT
sample_board pn-board BOARD
1700.0 3300.0 0.0 0.0 TOP MCAD
.END_PLACEMENT";

        let header = BoardPanelHeader {
            file_type: "PANEL_FILE".to_string(),
            version: 3,
            system_id: "Sample File Generator".to_string(),
            date: "10/22/96.16:20:19".to_string(),
            file_version: 1,
            board_name: "sample_panel".to_string(),
            units: "THOU".to_string(),
        };

        let outline = BoardPanelOutline {
            owner: "MCAD".to_string(),
            thickness: 62.0,
            outline: vec![
                Point {
                    loop_label: 0,
                    x: 0.0,
                    y: 0.0,
                    angle: 0.0,
                },
                Point {
                    loop_label: 0,
                    x: 16000.0,
                    y: 0.0,
                    angle: 0.0,
                },
            ],
        };

        let placement_keepouts = vec![
            PlacementKeepout {
                owner: "MCAD".to_string(),
                board_side: "BOTTOM".to_string(),
                keepout_height: 0.0,
                outline: vec![
                    Point {
                        loop_label: 0,
                        x: 13500.0,
                        y: 0.0,
                        angle: 0.0,
                    },
                    Point {
                        loop_label: 0,
                        x: 13500.0,
                        y: 12000.0,
                        angle: 0.0,
                    },
                    Point {
                        loop_label: 0,
                        x: 13500.0,
                        y: 0.0,
                        angle: 0.0,
                    },
                ],
            },
            PlacementKeepout {
                owner: "MCAD".to_string(),
                board_side: "BOTTOM".to_string(),
                keepout_height: 0.0,
                outline: vec![
                    Point {
                        loop_label: 0,
                        x: 0.0,
                        y: 0.0,
                        angle: 0.0,
                    },
                    Point {
                        loop_label: 0,
                        x: 2200.0,
                        y: 0.0,
                        angle: 0.0,
                    },
                    Point {
                        loop_label: 0,
                        x: 2200.0,
                        y: 12000.0,
                        angle: 0.0,
                    },
                    Point {
                        loop_label: 0,
                        x: 0.0,
                        y: 12000.0,
                        angle: 0.0,
                    },
                    Point {
                        loop_label: 0,
                        x: 0.0,
                        y: 0.0,
                        angle: 0.0,
                    },
                ],
            },
        ];

        let drilled_holes = vec![
            Hole {
                diameter: 250.0,
                x: 15500.0,
                y: 11500.0,
                plating_style: "NPTH".to_string(),
                associated_part: "PANEL".to_string(),
                hole_type: "TOOL".to_string(),
                owner: "MCAD".to_string(),
            },
            Hole {
                diameter: 250.0,
                x: 500.0,
                y: 500.0,
                plating_style: "NPTH".to_string(),
                associated_part: "PANEL".to_string(),
                hole_type: "TOOL".to_string(),
                owner: "MCAD".to_string(),
            },
        ];

        let component_placements = vec![ComponentPlacement {
            package_name: "sample_board".to_string(),
            part_number: "pn-board".to_string(),
            reference_designator: "BOARD".to_string(),
            x: 1700.0,
            y: 3300.0,
            mounting_offset: 0.0,
            rotation_angle: 0.0,
            board_side: "TOP".to_string(),
            placement_status: "MCAD".to_string(),
        }];

        let expected_panel = BoardPanel {
            header,
            outline,
            other_outlines: vec![],
            routing_outlines: vec![],
            placement_outlines: vec![],
            routing_keepouts: vec![],
            via_keepouts: vec![],
            placement_keepouts,
            placement_group_areas: vec![],
            drilled_holes,
            notes: vec![],
            component_placements,
        };

        let panel = parse_board_or_panel(input).unwrap();
        assert_eq!(panel, expected_panel);
    }
}
