use nom::branch::alt;
use nom::sequence::{delimited, terminated};

use crate::parse_section;
use crate::point::{Point, point};
use crate::primitives::ws;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::newline;
use nom::multi::many1;
use nom::number::complete::float;

/// Board/panel outline.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=10
///
/// This section defines the board or panel outline and its internal cutouts as a 2D profile with
/// thickness. The outline and cutouts consist of simple closed curves made up of arcs and lines.
/// Only one outline may be specified, but multiple cutouts are allowed.
#[derive(Clone, Debug, PartialEq)]
pub struct BoardPanelOutline {
    pub owner: String, // MCAD, ECAD or UNOWNED
    pub thickness: f32,
    pub outline: Vec<Point>,
}

pub fn parse_board_panel_outline(input: &str) -> IResult<&str, BoardPanelOutline> {
    fn interior_contents(input: &str) -> IResult<&str, (&str, f32, Vec<Point>)> {
        (
            terminated(owner, newline),
            terminated(float, newline),
            many1(terminated(point, newline)),
        )
            .parse(input)
    }

    let (remaining, (owner, thickness, outline)) = ws(alt((
        parse_section!("BOARD_OUTLINE", interior_contents),
        parse_section!("PANEL_OUTLINE", interior_contents),
    )))
    .parse(input)?;

    Ok((
        remaining,
        BoardPanelOutline {
            owner: owner.to_string(),
            thickness,
            outline,
        },
    ))
}

/// Other outline.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=11
///
/// This section defines an additional outline with cutouts that can be used for other purposes than
/// the board outline such as for defining a heatsink or board core. The outline and cutouts consist of
/// simple closed curves made up of arcs and lines. Multiple other outline sections may be specified.
#[derive(Clone, Debug, PartialEq)]
pub struct OtherOutline {
    pub owner: String, // MCAD, ECAD or UNOWNED
    pub id: String,
    pub extrude_thickness: f32,
    pub board_side: String, // TOP or BOTTOM
    pub outline: Vec<Point>,
}

pub fn parse_other_outline(input: &str) -> IResult<&str, OtherOutline> {
    let (remaining, (owner, id, extrude_thickness, board_side, outline)) = parse_section!(
        "OTHER_OUTLINE",
        (
            terminated(owner, newline),        // owner
            terminated(is_not(" "), tag(" ")), // ID
            terminated(float, tag(" ")),       // extrude_thickness
            terminated(is_not("\n"), newline), // board_side
            many1(terminated(point, newline)), // outline
        )
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

/// Routing outline.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=14
///
/// This section defines a routing outline for the board or panel. Each routing outline specifies a
/// region within which routing must be confined, and consists of a simple closed curve made up of
/// arcs and lines. Portions of routing outlines on a panel that lie on a board in the panel are inherited
/// by that board. Multiple routing outlines may be defined.
#[derive(Clone, Debug, PartialEq)]
pub struct RoutingOutline {
    pub owner: String,          // MCAD, ECAD or UNOWNED
    pub routing_layers: String, // TOP, BOTTOM, BOTH, INNER or ALL
    pub outline: Vec<Point>,
}

pub fn parse_routing_outline(input: &str) -> IResult<&str, RoutingOutline> {
    let (remaining, (owner, routing_layers, outline)) = parse_section!(
        "ROUTE_OUTLINE",
        (
            terminated(owner, newline),
            terminated(is_not("\n"), newline),
            many1(terminated(point, newline)),
        )
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

/// Placement outline.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=16
///
/// This section defines a placement outline for the board or panel. Each placement outline specifies
/// a region within which components must be placed, and consists of a simple closed curve made up
/// of arcs and lines plus a height restriction. Portions of placement outlines on a panel that lie on a
/// board in the panel are inherited by that board. Multiple placement outlines may be defined.
#[derive(Clone, Debug, PartialEq)]
pub struct PlacementOutline {
    pub owner: String,       // MCAD, ECAD or UNOWNED
    pub board_side: String,  // TOP, BOTTOM or BOTH
    pub outline_height: f32, // Any (≥ 0)
    pub outline: Vec<Point>,
}

pub fn parse_placement_outline(input: &str) -> IResult<&str, PlacementOutline> {
    let (remaining, (owner, board_side, outline_height, outline)) = parse_section!(
        "PLACE_OUTLINE",
        (
            terminated(owner, newline),        // owner
            terminated(is_not(" "), tag(" ")), // board_side
            terminated(float, newline),        // outline_height
            many1(terminated(point, newline)), // outline
        )
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

/// Routing keepout.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=18
///
/// This section defines a routing keepout for the board or panel. Routing keepouts specify regions
/// where routing is not allowed. Routing keepouts can exist on top, bottom, both top and bottom,
/// or all routing layers. Each keepout consists of a simple closed curve made up of arcs and lines.
/// Portions of routing keepouts on a panel that lie on a board in the panel are inherited by that board.
/// Multiple keepouts are allowed.
#[derive(Clone, Debug, PartialEq)]
pub struct RoutingKeepout {
    pub owner: String,          // MCAD, ECAD or UNOWNED
    pub routing_layers: String, // TOP, BOTTOM, BOTH, INNER or ALL
    pub outline: Vec<Point>,
}

pub fn parse_routing_keepout(input: &str) -> IResult<&str, RoutingKeepout> {
    let (remaining, (owner, routing_layers, outline)) = parse_section!(
        "ROUTE_KEEPOUT",
        (
            terminated(owner, newline),
            terminated(is_not("\n"), newline),
            many1(terminated(point, newline)),
        )
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

/// Via keepout.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=20
///
/// This section defines a via keepout for the board or panel. Via keepouts specify regions where vias
/// are not allowed (although routing is still allowed). Each keepout consists of a simple closed curve
/// made up of arcs and lines. Portions of via keepouts on a panel that lie on a board in the panel are
/// inherited by that board. Multiple via keepouts are allowed. Only through vias (vias that go all the
/// way through the board) are supported.
#[derive(Clone, Debug, PartialEq)]
pub struct ViaKeepout {
    pub owner: String, // MCAD, ECAD or UNOWNED
    pub outline: Vec<Point>,
}

pub fn parse_via_keepout(input: &str) -> IResult<&str, ViaKeepout> {
    let (remaining, (owner, outline)) = parse_section!(
        "VIA_KEEPOUT",
        (
            terminated(owner, newline),
            many1(terminated(point, newline)),
        )
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

/// Placement keepout.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=21
///
/// This section defines a placement keepout for the board or panel. Placement keepouts specify
/// regions on the board where components cannot be placed. A keepout can apply to all
/// components, or to only those components above a specified height. Placement keepouts can exist
/// on the top, bottom, or both top and bottom of the board or panel. Each keepout consists of a
/// simple closed curve made up of arcs and lines along with a height restriction. Portions of
/// placement keepouts on a panel that lie on a board in the panel are inherited by that board.
/// Multiple keepouts are allowed.
#[derive(Clone, Debug, PartialEq)]
pub struct PlacementKeepout {
    pub owner: String,       // MCAD, ECAD or UNOWNED
    pub board_side: String,  // TOP, BOTTOM or BOTH
    pub keepout_height: f32, // Any (≥ 0)
    pub outline: Vec<Point>,
}

pub fn parse_placement_keepout(input: &str) -> IResult<&str, PlacementKeepout> {
    let (remaining, (owner, board_side, keepout_height, outline)) = parse_section!(
        "PLACE_KEEPOUT",
        (
            terminated(owner, newline),
            terminated(is_not(" "), tag(" ")), // board_side
            terminated(float, newline),        // keepout_height
            many1(terminated(point, newline)), // outline
        )
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

/// Placement group area.
/// http://www.aertia.com/docs/priware/IDF_V30_Spec.pdf#page=23
///
/// This section specifies an area where a group of related components is to be placed. For example,
/// it may be desirable to place all analog components in a particular area for thermal considerations.
/// Each placement group area consists of a simple closed curve made up of arcs and lines along with
/// a name designating the group of components to be placed in that area. Multiple areas are
/// allowed.
#[derive(Clone, Debug, PartialEq)]
pub struct PlacementGroupArea {
    pub owner: String,      // MCAD, ECAD or UNOWNED
    pub board_side: String, // TOP, BOTTOM or BOTH
    pub group_name: String,
    pub outline: Vec<Point>,
}

pub fn parse_placement_group_area(input: &str) -> IResult<&str, PlacementGroupArea> {
    let (remaining, (owner, board_side, group_name, outline)) = parse_section!(
        "PLACE_REGION",
        (
            terminated(owner, newline),
            terminated(is_not(" "), tag(" ")), // board_side
            terminated(is_not("\n"), newline), // group_name
            many1(terminated(point, newline)), // outline
        )
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

/// Determine the owner of an outline or set of holes.
pub fn owner(input: &str) -> IResult<&str, &str> {
    alt((tag("ECAD"), tag("MCAD"), tag("UNOWNED"))).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_board_outline() {
        let input = ".BOARD_OUTLINE MCAD
62.0
0 5.5 -120.0 0.0
0 36.1 -120.0 263.266
1 5127.5 56 360
.END_BOARD_OUTLINE";

        let (remaining, board_outline) = parse_board_panel_outline(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(board_outline.owner, "MCAD");
        assert_eq!(board_outline.thickness, 62.0);
        assert_eq!(board_outline.outline.len(), 3);
        assert_eq!(board_outline.outline[0].loop_label, 0);
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
        assert_eq!(other_outline.outline[0].loop_label, 0);
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
        assert_eq!(routing_outline.outline[0].loop_label, 0);
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
        assert_eq!(placement_outline.outline[0].loop_label, 0);
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
        assert_eq!(routing_keepout.outline[0].loop_label, 0);
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
        assert_eq!(via_keepout.outline[0].loop_label, 0);
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
        assert_eq!(placement_keepout.outline[0].loop_label, 0);
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
        assert_eq!(placement_group_area.outline[0].loop_label, 0);
        assert_eq!(placement_group_area.outline[0].x, 5.5);
        assert_eq!(placement_group_area.outline[0].y, -120.0);
        assert_eq!(placement_group_area.outline[0].angle, 0.0);
    }

    #[test]
    fn test_point() {
        let input = "0 5.5 -120.0 0.0";
        let (remaining, point) = point(input).unwrap();

        assert_eq!(remaining, "");
        assert_eq!(point.loop_label, 0);
        assert_eq!(point.x, 5.5);
        assert_eq!(point.y, -120.0);
        assert_eq!(point.angle, 0.0);
    }

    #[test]
    fn test_owner() {
        let input = "ECADMCADUNOWNED";
        let (remaining, owner_str) = owner(input).unwrap();
        assert_eq!(remaining, "MCADUNOWNED");
        assert_eq!(owner_str, "ECAD");

        let (remaining, owner_str) = owner(remaining).unwrap();
        assert_eq!(remaining, "UNOWNED");
        assert_eq!(owner_str, "MCAD");

        let (remaining, owner_str) = owner(remaining).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(owner_str, "UNOWNED");
    }
}
