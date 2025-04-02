use crate::idf_v3::component_placement::{component_placement_section, ComponentPlacement};
use crate::idf_v3::drilled_holes::{drilled_holes_section, Hole};
use crate::idf_v3::headers::{board_header, BoardHeader};
use crate::idf_v3::notes::{notes_section, Note};
use crate::idf_v3::outlines::{
    parse_board_outline, parse_other_outline, parse_placement_group_area, parse_placement_keepout, parse_placement_outline,
    parse_routing_keepout, parse_routing_outline, parse_via_keepout, BoardOutline, OtherOutline,
    PlacementGroupArea, PlacementKeepout, PlacementOutline,
    RoutingKeepout, RoutingOutline, ViaKeepout,
};
use crate::idf_v3::primitives::ws;
use nom::multi::{many0, many_m_n};
use nom::{IResult, Parser};

/// Parse board or panel .emn file

struct Board {
    header: BoardHeader,
    outline: BoardOutline,
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
/// 
/// # Examples
/// 
/// ```
/// let input = ".HEADER
/// BOARD_FILE 3.0 \"Sample File Generator\" 10/22/96.16:02:44 1
/// sample_board THOU
/// .END_HEADER
/// ...";
/// 
/// let (remaining, board) = board(input).unwrap();
/// ```
fn board(input: &str) -> IResult<&str, Board> {
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
        board_header,
        ws(parse_board_outline),
        many0(parse_other_outline),
        many0(parse_routing_outline),
        many0(parse_placement_outline),
        many0(parse_routing_keepout),
        many0(parse_via_keepout),
        many0(parse_placement_keepout),
        many0(parse_placement_group_area),
        drilled_holes_section,
        many_m_n(0, 1, notes_section),
        component_placement_section,
    )
        .parse(input)?;

    let notes: Vec<Note> = if wrapped_notes.len() > 1 {
        wrapped_notes[0].clone()
    } else {
        Vec::new()
    };

    let board = Board {
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
    fn test_board() {
        let input = ".HEADER
BOARD_FILE 3.0 \"Sample File Generator\" 10/22/96.16:02:44 1
sample_board THOU
.END_HEADER
.BOARD_OUTLINE MCAD
62.0
0 5030.5 -120.0 0.0
0 5187.5 -120.0 0.0
0 5187.5 2130.0 0.0
0 5155.0 2130.0 0.0
0 5155.0 2550.0 -180.0
0 5187.5 2550.0 0.0
0 5187.5 4935.0 0.0
0 4945.0 5145.0 0.0
0 4945.0 5420.0 0.0
0 4865.0 5500.0 0.0
0 210.0 5500.0 0.0
0 130.0 5420.0 0.0
0 130.0 5145.0 0.0
0 -112.5 4935.0 0.0
0 -112.5 2550.0 0.0
0 -80.0 2550.0 0.0
0 -80.0 2130.0 -180.0
0 -112.5 2130.0 0.0
0 -112.5 -140.0 0.0
0 45.5 -140.0 0.0
0 45.5 -400.0 0.0
0 2442.5 -400.0 0.0
0 2442.5 -140.0 0.0
0 2631.5 -140.0 0.0
0 2631.5 -400.0 0.0
0 5030.5 -400.0 0.0
0 5030.5 -120.0 0.0
1 2650.0 2350.0 0.0
1 3000.0 2350.0 360.0
.END_BOARD_OUTLINE
.ROUTE_OUTLINE ECAD
ALL
0 5112.5 150.0 0.0
0 5112.5 2058.2 0.0
0 5112.5 2621.8 -162.9
0 5112.5 4863.2 0.0
0 4878.8 5075.0 0.0
0 226.4 5075.0 0.0
0 138.0 4910.3 0.0
0 138.0 4800.0 0.0
0 -37.5 4662.5 0.0
0 -37.5 2621.8 0.0
0 -37.5 2058.2 -162.9
0 -37.5 150.0 0.0
0 162.5 0.0 0.0
0 4912.5 0.0 0.0
0 5112.5 150.0 0.0
.END_ROUTE_OUTLINE
.PLACE_OUTLINE MCAD
TOP 1000.0
0 5080.0 2034.9 0.0
0 5080.0 2645.1 -152.9
0 5080.0 4837.3 0.0
0 4855.3 5042.5 0.0
0 252.9 5042.5 0.0
0 170.5 4896.9 0.0
0 170.5 4798.4 0.0
0 -5.0 4659.0 0.0
0 -5.0 2645.1 0.0
0 -5.0 2034.9 -152.9
0 -5.0 182.5 0.0
0 192.0 32.5 0.0
0 4883.1 32.5 0.0
0 5080.0 182.5 0.0
0 5080.0 2034.9 0.0
.END_PLACE_OUTLINE
.PLACE_OUTLINE UNOWNED
BOTTOM 200.0
0 300.0 200.0 0.0
0 4800.0 200.0 0.0
0 4800.0 4800.0 0.0
0 300.0 4800.0 0.0
0 300.0 200.0 0.0
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
0 3700.0 4300.0 0.0
0 4000.0 4300.0 0.0
0 4000.0 3700.0 0.0
0 5000.0 3700.0 0.0
0 5000.0 4800.0 0.0
0 4800.0 5000.0 0.0
0 3700.0 5000.0 0.0
.END_PLACE_KEEPOUT
.DRILLED_HOLES
30.0 1800.0 100.0 PTH J1 PIN ECAD
30.0 1700.0 100.0 PTH J1 PIN ECAD
30.0 1600.0 100.0 PTH J1 PIN ECAD
30.0 1500.0 100.0 PTH J1 PIN ECAD
30.0 1400.0 100.0 PTH J1 PIN ECAD
30.0 1300.0 100.0 PTH J1 PIN ECAD
30.0 1200.0 100.0 PTH J1 PIN ECAD
30.0 1100.0 100.0 PTH J1 PIN ECAD
30.0 1000.0 100.0 PTH J1 PIN ECAD
30.0 0900.0 100.0 PTH J1 PIN ECAD
30.0 0800.0 100.0 PTH J1 PIN ECAD
30.0 0700.0 100.0 PTH J1 PIN ECAD
30.0 0700.0 200.0 PTH J1 PIN ECAD
30.0 0800.0 200.0 PTH J1 PIN ECAD
30.0 0900.0 200.0 PTH J1 PIN ECAD
30.0 1000.0 200.0 PTH J1 PIN ECAD
30.0 1100.0 200.0 PTH J1 PIN ECAD
30.0 1200.0 200.0 PTH J1 PIN ECAD
30.0 1300.0 200.0 PTH J1 PIN ECAD
30.0 1400.0 200.0 PTH J1 PIN ECAD
30.0 1500.0 200.0 PTH J1 PIN ECAD
30.0 1600.0 200.0 PTH J1 PIN ECAD
30.0 1700.0 200.0 PTH J1 PIN ECAD
30.0 1800.0 200.0 PTH J1 PIN ECAD
30.0 4400.0 100.0 PTH J2 PIN ECAD
30.0 4300.0 100.0 PTH J2 PIN ECAD
30.0 4200.0 100.0 PTH J2 PIN ECAD
30.0 4100.0 100.0 PTH J2 PIN ECAD
30.0 4000.0 100.0 PTH J2 PIN ECAD
30.0 3900.0 100.0 PTH J2 PIN ECAD
30.0 3800.0 100.0 PTH J2 PIN ECAD
30.0 3700.0 100.0 PTH J2 PIN ECAD
30.0 3600.0 100.0 PTH J2 PIN ECAD
30.0 3500.0 100.0 PTH J2 PIN ECAD
30.0 3400.0 100.0 PTH J2 PIN ECAD
30.0 3300.0 100.0 PTH J2 PIN ECAD
30.0 3300.0 200.0 PTH J2 PIN ECAD
30.0 3400.0 200.0 PTH J2 PIN ECAD
30.0 3500.0 200.0 PTH J2 PIN ECAD
30.0 3600.0 200.0 PTH J2 PIN ECAD
30.0 3700.0 200.0 PTH J2 PIN ECAD
30.0 3800.0 200.0 PTH J2 PIN ECAD
30.0 3900.0 200.0 PTH J2 PIN ECAD
30.0 4000.0 200.0 PTH J2 PIN ECAD
30.0 4100.0 200.0 PTH J2 PIN ECAD
30.0 4200.0 200.0 PTH J2 PIN ECAD
30.0 4300.0 200.0 PTH J2 PIN ECAD
30.0 4400.0 200.0 PTH J2 PIN ECAD
30.0 3000.0 3300.0 PTH U3 PIN ECAD
30.0 3024.2 3203.0 PTH U3 PIN ECAD
30.0 3048.4 3105.9 PTH U3 PIN ECAD
30.0 3072.6 3008.9 PTH U3 PIN ECAD
30.0 3096.8 2911.9 PTH U3 PIN ECAD
30.0 3121.0 2814.9 PTH U3 PIN ECAD
30.0 3145.2 2717.8 PTH U3 PIN ECAD
30.0 3436.2 2790.4 PTH U3 PIN ECAD
30.0 3412.1 2887.4 PTH U3 PIN ECAD
30.0 3387.9 2984.5 PTH U3 PIN ECAD
30.0 3363.7 3081.5 PTH U3 PIN ECAD
30.0 3339.5 3178.5 PTH U3 PIN ECAD
30.0 3315.3 3275.6 PTH U3 PIN ECAD
30.0 3291.1 3372.6 PTH U3 PIN ECAD
30.0 2200.0 2500.0 PTH U4 PIN ECAD
30.0 2100.0 2500.0 PTH U4 PIN ECAD
30.0 2000.0 2500.0 PTH U4 PIN ECAD
30.0 1900.0 2500.0 PTH U4 PIN ECAD
30.0 1800.0 2500.0 PTH U4 PIN ECAD
30.0 1700.0 2500.0 PTH U4 PIN ECAD
30.0 1600.0 2500.0 PTH U4 PIN ECAD
30.0 1600.0 2200.0 PTH U4 PIN ECAD
30.0 1700.0 2200.0 PTH U4 PIN ECAD
30.0 1800.0 2200.0 PTH U4 PIN ECAD
30.0 1900.0 2200.0 PTH U4 PIN ECAD
30.0 2000.0 2200.0 PTH U4 PIN ECAD
30.0 2100.0 2200.0 PTH U4 PIN ECAD
30.0 2200.0 2200.0 PTH U4 PIN ECAD
20.0 2500.0 3100.0 PTH BOARD VIA ECAD
20.0 2500.0 3200.0 PTH BOARD VIA ECAD
20.0 2500.0 3300.0 PTH BOARD VIA ECAD
20.0 2000.0 1600.0 PTH BOARD VIA ECAD
20.0 1100.0 0900.0 PTH BOARD VIA ECAD
20.0 1200.0 1600.0 PTH BOARD VIA ECAD
20.0 3900.0 3800.0 PTH BOARD VIA ECAD
20.0 3900.0 2300.0 PTH BOARD VIA ECAD
100.0 3100.0 -50.0 NPTH J2 MTG ECAD
100.0 4600.0 -50.0 NPTH J2 MTG ECAD
100.0 500.0 -50.0 NPTH J1 MTG ECAD
100.0 2000.0 -50.0 NPTH J1 MTG ECAD
93.0 5075.0 0.0 PTH BOARD MTG UNOWNED
93.0 0.0 4800.0 NPTH BOARD TOOL MCAD
93.0 0.0 0.0 PTH BOARD MTG UNOWNED
.END_DRILLED_HOLES
.NOTES
3500.0 3300.0 75.0 2500.0 \"This component rotated 14 degrees\"
400.0 4400.0 75.0 3200.0 \"Component height limited by enclosure latch\"
1800.0 300.0 75.0 1700.0 \"Do not move connectors!\"
.END_NOTES
.PLACEMENT
cs13_a pn-cap C1
4000.0 1000.0 100.0 0.0 TOP PLACED
cc1210 pn-cc1210 C2
3000.0 3500.0 0.0 0.0 TOP PLACED
cc1210 pn-cc1210 C3
3200.0 1800.0 0.0 0.0 BOTTOM PLACED
cc1210 pn-cc1210 C4
1400.0 2300.0 0.0 270.0 TOP PLACED
cc1210 pn-cc1210 C5
1799.5 3518.1 0.0 0.0 BOTTOM PLACED
conn_din24 connector J1
1800.0 100.0 0.0 0.0 TOP MCAD
conn_din24 connector J2
4400.0 100.0 0.0 0.0 TOP MCAD
plcc_20 pn-pal16l8-plcc U1
1800.0 3200.0 0.0 0.0 BOTTOM ECAD
plcc_20 pn-pal16l8-plcc U2
3200.0 1800.0 0.0 0.0 TOP PLACED
dip_14w pn-hs346-dip U3
3000.0 3300.0 0.0 14.0 TOP PLACED
dip_14w pn-hs346-dip U4
2200.0 2500.0 0.0 270.0 TOP PLACED
.END_PLACEMENT";

        let (remaining, board) = board(input).unwrap();
        assert_eq!(remaining, "");
    }
    #[test]
    fn test_board_with_invalid_input() {
        let input = ".HEADER
PANEL_FILE 3.0 \"Sample File Generator\" 10/22/96.16:20:19 1
sample_panel THOU
.END_HEADER
.PANEL_OUTLINE MCAD
62.0
0 0.0 0.0 0.0
0 16000.0 0.0 0.0
0 16000.0 12000.0 0.0
0 0.0 12000.0 0.0
0 0.0 0.0 0.0
.END_PANEL_OUTLINE
.PLACE_KEEPOUT MCAD
BOTTOM 0.0
0 13500.0 0.0 0.0
0 16000.0 0.0 0.0
0 16000.0 12000.0 0.0
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
250.0 500.0 11500.0 NPTH PANEL TOOL MCAD
250.0 500.0 500.0 NPTH PANEL TOOL MCAD
.END_DRILLED_HOLES
.PLACEMENT
sample_board pn-board BOARD
1700.0 3300.0 0.0 0.0 TOP MCAD
sample_board pn-board BOARD
14000.0 3300.0 0.0 0.0 BOTTOM MCAD
.END_PLACEMENT";

        let (remaining, board) = board(input).unwrap();
        assert_eq!(remaining, "");
    }
}
