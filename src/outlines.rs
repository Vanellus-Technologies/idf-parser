use nom::branch::alt;
use nom::sequence::{delimited, terminated};

use crate::idf_v3::primitives;
use crate::idf_v3::primitives::{owner, ws, Point};
use nom::bytes::complete::{is_not, tag};
use nom::multi::many1;
use nom::number::complete::float;
use nom::IResult;
use nom::Parser;

pub struct BoardOutline {
    owner: String, // MCAD, ECAD or UNOWNED
    thickness: f32,
    outline: Vec<Point>,
}

pub struct OtherOutline {
    owner: String, // MCAD, ECAD or UNOWNED
    id: String,
    extrude_thickness: f32,
    board_side: String, // TOP or BOTTOM
    outline: Vec<Point>,
}
pub struct RoutingOutline {
    owner: String,          // MCAD, ECAD or UNOWNED
    routing_layers: String, // TOP, BOTTOM, BOTH, INNER or ALL
    outline: Vec<Point>,
}

pub struct PlacementOutline {
    owner: String,       // MCAD, ECAD or UNOWNED
    board_side: String,  // TOP, BOTTOM or BOTH
    outline_height: f32, // Any (≥ 0)
    outline: Vec<Point>,
}

pub struct RoutingKeepout {
    owner: String,          // MCAD, ECAD or UNOWNED
    routing_layers: String, // TOP, BOTTOM, BOTH, INNER or ALL
    outline: Vec<Point>,
}

pub struct ViaKeepout {
    owner: String, // MCAD, ECAD or UNOWNED
    outline: Vec<Point>,
}

pub struct PlacementKeepout {
    owner: String,       // MCAD, ECAD or UNOWNED
    board_side: String,  // TOP, BOTTOM or BOTH
    keepout_height: f32, // Any (≥ 0)
    outline: Vec<Point>,
}

pub struct PlacementGroupArea {
    owner: String,      // MCAD, ECAD or UNOWNED
    board_side: String, // TOP, BOTTOM or BOTH
    group_name: String,
    outline: Vec<Point>,
}

pub fn parse_board_outline(input: &str) -> IResult<&str, BoardOutline> {
    let (remaining, (owner, thickness, outline, _end)) = (
        delimited(
            ws(alt((tag(".BOARD_OUTLINE"), tag(".PANEL_OUTLINE")))),
            owner,
            tag("\n"),
        ),
        terminated(float, tag("\n")),
        many1(terminated(primitives::point, tag("\n"))),
        alt((tag(".END_BOARD_OUTLINE"), tag(".END_PANEL_OUTLINE"))),
    )
        .parse(input)?;

    Ok((
        remaining,
        BoardOutline {
            owner: owner.to_string(),
            thickness,
            outline,
        },
    ))
}

pub fn parse_other_outline(input: &str) -> IResult<&str, OtherOutline> {
    let (remaining, (owner, id, extrude_thickness, board_side, outline, _end)) = (
        delimited(tag(".OTHER_OUTLINE "), owner, tag("\n")), // owner
        terminated(is_not(" "), tag(" ")),                   // ID
        terminated(float, tag(" ")),                         // extrude_thickness
        terminated(is_not("\n"), tag("\n")),                 // board_side
        many1(terminated(primitives::point, tag("\n"))),     // outline
        tag(".END_OTHER_OUTLINE"),
    )
        .parse(input)?;

    Ok((
        remaining,
        OtherOutline {
            owner: owner.to_string(),
            id: id.to_string(),
            extrude_thickness,
            board_side: board_side.to_string(),
            outline,
        },
    ))
}

pub fn parse_routing_outline(input: &str) -> IResult<&str, RoutingOutline> {
    let (remaining, (owner, routing_layers, outline, _end)) = (
        delimited(tag(".ROUTE_OUTLINE "), owner, tag("\n")),
        terminated(is_not("\n"), tag("\n")),
        many1(terminated(primitives::point, tag("\n"))),
        tag(".END_ROUTE_OUTLINE"),
    )
        .parse(input)?;

    Ok((
        remaining,
        RoutingOutline {
            owner: owner.to_string(),
            routing_layers: routing_layers.to_string(),
            outline,
        },
    ))
}

pub fn parse_placement_outline(input: &str) -> IResult<&str, PlacementOutline> {
    let (remaining, (owner, board_side, outline_height, outline, _end)) = (
        delimited(ws(tag(".PLACE_OUTLINE ")), owner, tag("\n")),
        terminated(is_not(" "), tag(" ")), // board_side
        terminated(float, tag("\n")),      // outline_height
        many1(terminated(primitives::point, tag("\n"))), // outline
        tag(".END_PLACE_OUTLINE"),
    )
        .parse(input)?;

    Ok((
        remaining,
        PlacementOutline {
            owner: owner.to_string(),
            board_side: board_side.to_string(),
            outline_height,
            outline,
        },
    ))
}

pub fn parse_routing_keepout(input: &str) -> IResult<&str, RoutingKeepout> {
    let (remaining, (owner, routing_layers, outline, _end)) = (
        delimited(ws(tag(".ROUTE_KEEPOUT ")), owner, tag("\n")),
        terminated(is_not("\n"), tag("\n")),
        many1(terminated(primitives::point, tag("\n"))),
        tag(".END_ROUTE_KEEPOUT"),
    )
        .parse(input)?;

    Ok((
        remaining,
        RoutingKeepout {
            owner: owner.to_string(),
            routing_layers: routing_layers.to_string(),
            outline,
        },
    ))
}

pub fn parse_via_keepout(input: &str) -> IResult<&str, ViaKeepout> {
    let (remaining, (owner, outline, _end)) = (
        delimited(tag(".VIA_KEEPOUT "), owner, tag("\n")),
        many1(terminated(primitives::point, tag("\n"))),
        tag(".END_VIA_KEEPOUT"),
    )
        .parse(input)?;

    Ok((
        remaining,
        ViaKeepout {
            owner: owner.to_string(),
            outline,
        },
    ))
}

pub fn parse_placement_keepout(input: &str) -> IResult<&str, PlacementKeepout> {
    let (remaining, (owner, board_side, keepout_height, outline, _end)) = (
        delimited(ws(tag(".PLACE_KEEPOUT ")), owner, tag("\n")),
        terminated(is_not(" "), tag(" ")), // board_side
        terminated(float, tag("\n")),      // keepout_height
        many1(terminated(primitives::point, tag("\n"))), // outline
        tag(".END_PLACE_KEEPOUT"),
    )
        .parse(input)?;

    Ok((
        remaining,
        PlacementKeepout {
            owner: owner.to_string(),
            board_side: board_side.to_string(),
            keepout_height,
            outline,
        },
    ))
}

pub fn parse_placement_group_area(input: &str) -> IResult<&str, PlacementGroupArea> {
    let (remaining, (owner, board_side, group_name, outline, _end)) = (
        delimited(tag(".PLACE_REGION "), owner, tag("\n")),
        terminated(is_not(" "), tag(" ")),   // board_side
        terminated(is_not("\n"), tag("\n")), // group_name
        many1(terminated(primitives::point, tag("\n"))), // outline
        tag(".END_PLACE_REGION"),
    )
        .parse(input)?;

    Ok((
        remaining,
        PlacementGroupArea {
            owner: owner.to_string(),
            board_side: board_side.to_string(),
            group_name: group_name.to_string(),
            outline,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::idf_v3::primitives::point;
    #[test]
    fn test_parse_board_outline() {
        let input = ".BOARD_OUTLINE MCAD
62.0
0 5.5 -120.0 0.0
0 36.1 -120.0 263.266
1 5127.5 56 360
.END_BOARD_OUTLINE";

        let (remaining, board_outline) = parse_board_outline(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(board_outline.owner, "MCAD");
        assert_eq!(board_outline.thickness, 62.0);
        assert_eq!(board_outline.outline.len(), 3);
        assert_eq!(board_outline.outline[0].label, 0);
        assert_eq!(board_outline.outline[0].x, 5.5);
        assert_eq!(board_outline.outline[0].y, -120.0);
        assert_eq!(board_outline.outline[0].angle, 0.0);
    }

    #[test]
    fn test_parse_other_outline() {
        let input = ".OTHER_OUTLINE MCAD
my_outline 62.0 TOP
0 5.5 -120.0 0.0
.END_OTHER_OUTLINE";

        let (remaining, other_outline) = parse_other_outline(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(other_outline.owner, "MCAD");
        assert_eq!(other_outline.id, "my_outline");
        assert_eq!(other_outline.extrude_thickness, 62.0);
        assert_eq!(other_outline.board_side, "TOP");
        assert_eq!(other_outline.outline.len(), 1);
        assert_eq!(other_outline.outline[0].label, 0);
        assert_eq!(other_outline.outline[0].x, 5.5);
        assert_eq!(other_outline.outline[0].y, -120.0);
        assert_eq!(other_outline.outline[0].angle, 0.0);
    }

    #[test]
    fn test_parse_routing_outline() {
        let input = ".ROUTE_OUTLINE ECAD
ALL
0 5112.5 150.0 0.0
0 5112.5 2058.2 0.0
.END_ROUTE_OUTLINE";

        let (remaining, routing_outline) = parse_routing_outline(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(routing_outline.owner, "ECAD");
        assert_eq!(routing_outline.routing_layers, "ALL");
        assert_eq!(routing_outline.outline.len(), 2);
        assert_eq!(routing_outline.outline[0].label, 0);
        assert_eq!(routing_outline.outline[0].x, 5112.5);
        assert_eq!(routing_outline.outline[0].y, 150.0);
        assert_eq!(routing_outline.outline[0].angle, 0.0);
    }

    #[test]
    fn test_parse_placement_outline() {
        let input = ".PLACE_OUTLINE MCAD
TOP 1000.0
0 -5.0 2034.9 -152.9
.END_PLACE_OUTLINE";

        let (remaining, placement_outline) = parse_placement_outline(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(placement_outline.owner, "MCAD");
        assert_eq!(placement_outline.board_side, "TOP");
        assert_eq!(placement_outline.outline_height, 1000.0);
        assert_eq!(placement_outline.outline.len(), 1);
        assert_eq!(placement_outline.outline[0].label, 0);
        assert_eq!(placement_outline.outline[0].x, -5.0);
        assert_eq!(placement_outline.outline[0].y, 2034.9);
        assert_eq!(placement_outline.outline[0].angle, -152.9);
    }

    #[test]
    fn test_parse_routing_keepout() {
        let input = ".ROUTE_KEEPOUT ECAD
ALL
0 2650.0 2350.0 0.0
0 3100.0 2350.0 360.0
.END_ROUTE_KEEPOUT";

        let (remaining, routing_keepout) = parse_routing_keepout(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(routing_keepout.owner, "ECAD");
        assert_eq!(routing_keepout.routing_layers, "ALL");
        assert_eq!(routing_keepout.outline.len(), 2);
        assert_eq!(routing_keepout.outline[0].label, 0);
        assert_eq!(routing_keepout.outline[0].x, 2650.0);
        assert_eq!(routing_keepout.outline[0].y, 2350.0);
        assert_eq!(routing_keepout.outline[0].angle, 0.0);
    }
    #[test]
    fn test_parse_via_keepout() {
        let input = ".VIA_KEEPOUT ECAD
0 2650.0 2350.0 0.0
0 3100.0 2350.0 360.0
.END_VIA_KEEPOUT";

        let (remaining, via_keepout) = parse_via_keepout(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(via_keepout.owner, "ECAD");
        assert_eq!(via_keepout.outline.len(), 2);
        assert_eq!(via_keepout.outline[0].label, 0);
        assert_eq!(via_keepout.outline[0].x, 2650.0);
        assert_eq!(via_keepout.outline[0].y, 2350.0);
        assert_eq!(via_keepout.outline[0].angle, 0.0);
    }

    #[test]
    fn test_parse_placement_keepout() {
        let input = ".PLACE_KEEPOUT MCAD
TOP 300.0
0 3700.0 5000.0 0.0
0 3700.0 5000.0 0.0
.END_PLACE_KEEPOUT";

        let (remaining, placement_keepout) = parse_placement_keepout(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(placement_keepout.owner, "MCAD");
        assert_eq!(placement_keepout.board_side, "TOP");
        assert_eq!(placement_keepout.keepout_height, 300.0);
        assert_eq!(placement_keepout.outline.len(), 2);
        assert_eq!(placement_keepout.outline[0].label, 0);
        assert_eq!(placement_keepout.outline[0].x, 3700.0);
        assert_eq!(placement_keepout.outline[0].y, 5000.0);
        assert_eq!(placement_keepout.outline[0].angle, 0.0);
    }

    #[test]
    fn test_parse_placement_group_area() {
        let input = ".PLACE_REGION UNOWNED
TOP the_best_group
0 5.5 -120.0 0.0
.END_PLACE_REGION";

        let (remaining, placement_group_area) = parse_placement_group_area(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(placement_group_area.owner, "UNOWNED");
        assert_eq!(placement_group_area.board_side, "TOP");
        assert_eq!(placement_group_area.group_name, "the_best_group");
        assert_eq!(placement_group_area.outline.len(), 1);
        assert_eq!(placement_group_area.outline[0].label, 0);
        assert_eq!(placement_group_area.outline[0].x, 5.5);
        assert_eq!(placement_group_area.outline[0].y, -120.0);
        assert_eq!(placement_group_area.outline[0].angle, 0.0);
    }

    #[test]
    fn test_point() {
        let input = "0 5.5 -120.0 0.0";
        let (remaining, point) = point(input).unwrap();

        assert_eq!(remaining, "");
        assert_eq!(point.label, 0);
        assert_eq!(point.x, 5.5);
        assert_eq!(point.y, -120.0);
        assert_eq!(point.angle, 0.0);
    }
}
