use crate::component_placement::{ComponentPlacement, parse_component_placement_section};
use crate::drilled_holes::{Hole, parse_drilled_holes_section};
use crate::headers::{BoardPanelHeader, parse_board_or_panel_header};
use crate::notes::{Note, parse_notes_section};
use crate::outlines::{
    BoardPanelOutline, OtherOutline, PlacementGroupArea, PlacementKeepout, PlacementOutline,
    RoutingKeepout, RoutingOutline, ViaKeepout, parse_board_outline, parse_other_outline,
    parse_placement_group_area, parse_placement_keepout, parse_placement_outline,
    parse_routing_keepout, parse_routing_outline, parse_via_keepout,
};
use nom::multi::{many_m_n, many0};
use nom::{IResult, Parser};

/// Represents a board or panel file in the IDF format.
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
pub fn parse_board_or_panel(input: &str) -> IResult<&str, BoardPanel> {
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
        parse_board_outline,
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
        .parse(input)?;

    // Unwrap the notes section, if it exists. We expect there to be either 0 or 1 sections.
    let notes: Vec<Note> = if wrapped_notes.len() > 1 {
        wrapped_notes[0].clone()
    } else {
        Vec::new()
    };

    let board = BoardPanel {
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

    Ok((remaining, board))
}

#[cfg(test)]
mod tests {
    use super::*;
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

        let (remaining, board) = parse_board_or_panel(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(board.component_placements.len(), 3);
        assert_eq!(board.component_placements[0].package_name, "cs13_a");
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

        let (remaining, board) = parse_board_or_panel(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(board.component_placements.len(), 1);
        assert_eq!(board.component_placements[0].package_name, "sample_board");
    }
}
