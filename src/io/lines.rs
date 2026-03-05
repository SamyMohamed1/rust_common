//! Acc Lines
//!
use core::time::Duration;

use super::{meter, mps, Length, Velocity};
use acc_interface::datatypes::{
    egolineposition_t, linecrossing_t, linecrossingcoll_t, multiclothoid_t,
    roadobjectcoll_extended_t, roadsurface_t, wmline_t, ClothoidSegmentParam, DataQualifier,
    LaneBorderSideType, LineCrossingRefPoints, RoadMarkingType, RoadObjectLaneAssociation,
    RoadSurfaceConditionClassificationType,
};

#[allow(unused_imports)]
use num_traits::Float;

/// Line Pos from  ego position
#[derive(Debug, Clone, PartialEq, Default)]
pub enum LinePos {
    /// Left Line
    Left,
    /// Right Lane
    Right,
    /// Left Left Line,
    LeftLeft,
    /// Right Right Line
    RightRight,
    #[default]
    /// Not Used
    NotUsed,
}

/// Line Marking Type
#[derive(Debug, Clone, Default, PartialEq)]
pub enum LineType {
    #[default]
    /// Unknown
    Unknown,
    /// Dashed Line
    Dashed,
    /// Solid Line
    Solid,
}

impl From<&RoadMarkingType> for LineType {
    fn from(val: &RoadMarkingType) -> Self {
        use RoadMarkingType::*;
        match val {
            RMT_CENTRE_LINE_DASHED_MARKING
            | RMT_EDGE_LINE_DASHED_MARKING
            | RMT_CENTRE_LINE_DOUBLE_LINE_DASHED
            | RMT_EDGE_LINE_DOUBLE_LINE_DASHED
            | RMT_CENTRE_LINE_MULTIPLE_LINE_DASHED
            | RMT_EDGE_LINE_MULTIPLE_LINE_DASHED => Self::Dashed,
            RMT_SOLID | RMT_DOUBLE_LINE_SOLID | RMT_MULTIPLE_LINE_SOLID => Self::Solid,
            _ => Self::Unknown,
        }
    }
}

/// Distance from ego to Line
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LineDis {
    /// FRONT LEFT
    pub fl: f32,
    /// FRONT RIGTH
    pub fr: f32,
    /// REAR LEFT
    pub rl: f32,
    /// REAR RIGHT
    pub rr: f32,
    /// Center of Geometry
    pub cog: f32,
}

/// Line Crossing Referencial Point
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EgoTimeToLineCross {
    /// Front left corner time to line cross
    pub fl: f32,
    /// Front right corner time to line cross
    pub fr: f32,
    /// rear left corner time to line cross
    pub rl: f32,
    /// rear right corner time to line cross
    pub rr: f32,
    /// Mid side Mid width time to line cross
    pub center: f32,
}

/// Clothoid Segment
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct ClothoidSegment {
    /// x coord
    pub x0: f32,
    /// y coord
    pub y0: f32,
    /// heading angle
    pub psi0: f32,
    /// curvature
    pub c0: f32,
    /// curvature rate
    pub c1: f32,
    /// length of the clothoid
    pub length: f32,
}
impl ClothoidSegment {
    /// [`ClothoidSegment`] constructor
    pub const fn new(x0: f32, y0: f32, psi0: f32, c0: f32, c1: f32, length: f32) -> Self {
        Self {
            x0,
            y0,
            psi0,
            c0,
            c1,
            length,
        }
    }
    /// Check if the clothoid is a straight line
    pub fn is_straight(&self) -> bool {
        self.c0.eq(&0.0) && self.c1.eq(&0.0)
    }
}

/// Road Surface Classification
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum RoadSurfaceCls {
    #[default]
    /// Not Used
    NotUsed,
    /// Wet
    Wet,
}

impl From<&roadsurface_t> for RoadSurfaceCls {
    fn from(value: &roadsurface_t) -> Self {
        let is_wet = value
            .conditionclass
            .iter()
            .take(value.nbrsconditionclass.into())
            .any(|x| matches!(x.r#type, RoadSurfaceConditionClassificationType::RSCCT_WET));

        if is_wet {
            RoadSurfaceCls::Wet
        } else {
            RoadSurfaceCls::NotUsed
        }
    }
}

/// Line
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Line {
    /// Line ID
    pub id: u32,
    /// line Type
    pub line_type: LineType,
    /// Line Position
    pub pos: LinePos,
    /// Confidence
    pub existence_prob: f32,
    /// Width
    pub width: Length,
    /// ego line postion
    pub ego_pos: LineDis,
    /// first clothoid segment
    pub first_clothoid: ClothoidSegment,
    /// second clothoid segment
    pub second_clothoid: Option<ClothoidSegment>,
    /// Time to line crossing
    pub ttlc: EgoTimeToLineCross,
}

impl Line {
    /// Check if the line is a solid line
    pub fn is_solid(&self) -> bool {
        self.line_type.eq(&LineType::Solid)
    }

    /// Check if the line is a dashed line
    pub fn is_dashed(&self) -> bool {
        self.line_type.eq(&LineType::Dashed)
    }

    /// Compute Vlat from velocity and heading angle
    /// Thx to LS
    pub fn compute_vlat(vx: Velocity, vy: Velocity, psi0: f32) -> f32 {
        -vx.get::<mps>() * psi0.sin() + vy.get::<mps>() * psi0.cos()
    }
    #[allow(clippy::indexing_slicing)]
    fn to_multiclothoid(
        m_clothoid: &multiclothoid_t,
    ) -> (ClothoidSegment, Option<ClothoidSegment>) {
        let x0 = m_clothoid.clothoid[0].param[ClothoidSegmentParam::CSP_X0 as usize];
        let length = m_clothoid.clothoid[0].param[ClothoidSegmentParam::CSP_LEN as usize];
        let y0 = m_clothoid.clothoid[0].param[ClothoidSegmentParam::CSP_Y0 as usize];
        let psi0 = m_clothoid.clothoid[0].param[ClothoidSegmentParam::CSP_PSI0 as usize];
        let c0 = m_clothoid.clothoid[0].param[ClothoidSegmentParam::CSP_C0 as usize];
        let c1 = m_clothoid.clothoid[0].param[ClothoidSegmentParam::CSP_C1 as usize];
        let clothoid_segment = ClothoidSegment::new(x0, y0, psi0, c0, c1, length);
        let clothoid_segment2 = (m_clothoid.nb > 1).then(|| {
            let x0 = m_clothoid.clothoid[1].param[ClothoidSegmentParam::CSP_X0 as usize];
            let length = m_clothoid.clothoid[1].param[ClothoidSegmentParam::CSP_LEN as usize];
            let y0 = m_clothoid.clothoid[1].param[ClothoidSegmentParam::CSP_Y0 as usize];
            let psi0 = m_clothoid.clothoid[1].param[ClothoidSegmentParam::CSP_PSI0 as usize];
            let c0 = m_clothoid.clothoid[1].param[ClothoidSegmentParam::CSP_C0 as usize];
            let c1 = m_clothoid.clothoid[1].param[ClothoidSegmentParam::CSP_C1 as usize];
            ClothoidSegment::new(x0, y0, psi0, c0, c1, length)
        });
        (clothoid_segment, clothoid_segment2)
    }
    /// Distance between line and tire < threshold (Ego close to line)
    pub fn ego_is_closed_to_line(&self, threshold: f32, side: LinePos) -> bool {
        match side {
            LinePos::Right => self
                .ego_pos
                .fr // TO BE CONFIRMED
                .lt(&threshold),
            LinePos::Left => self
                .ego_pos
                .fl // TO BE CONFIRMED
                .lt(&threshold),
            _ => false,
        }
    }
}
impl From<(&wmline_t, &egolineposition_t, LinePos, &linecrossing_t)> for Line {
    #[allow(clippy::indexing_slicing)]
    fn from(value: (&wmline_t, &egolineposition_t, LinePos, &linecrossing_t)) -> Self {
        use LineCrossingRefPoints::*;

        let (line, ego_pos, pos, line_cross) = value;
        let multi_clothoid = Line::to_multiclothoid(&line.multiclothoid);
        Self {
            id: line.status.objectid,
            pos,
            line_type: (&line.information.markingtype).into(),
            existence_prob: f32::from(line.status.existenceprobability) / 100.0,
            width: Length::new::<meter>(line.information.width),
            first_clothoid: multi_clothoid.0,
            second_clothoid: multi_clothoid.1,
            ego_pos: LineDis {
                fl: ego_pos.linedist[LCRP_FRONT_LEFT as usize],
                fr: ego_pos.linedist[LCRP_FRONT_RIGHT as usize],
                rl: ego_pos.linedist[LCRP_REAR_LEFT as usize],
                rr: ego_pos.linedist[LCRP_REAR_RIGHT as usize],
                cog: ego_pos.linedist[LCRP_MID_SIDE_MID_WIDTH as usize],
            },
            ttlc: EgoTimeToLineCross {
                fl: line_cross.ttlc[LCRP_FRONT_LEFT as usize],
                fr: line_cross.ttlc[LCRP_FRONT_RIGHT as usize],
                rl: line_cross.ttlc[LCRP_REAR_LEFT as usize],
                rr: line_cross.ttlc[LCRP_REAR_RIGHT as usize],
                center: line_cross.ttlc[LCRP_MID_SIDE_MID_WIDTH as usize],
            },
        }
    }
}

/// Ego Lines from the ego position
#[derive(Debug, Clone, Default)]
pub struct Lines {
    /// Timestamp
    pub timestamp: Duration,
    /// left Line
    pub left: Option<Line>,
    /// right Line
    pub right: Option<Line>,
    /// left left
    pub leftleft: Option<Line>,
    /// right right
    pub rightright: Option<Line>,
    /// nb lanes
    pub nb_lanes: u8,
    /// Road Surface Cls
    pub road_surface_cls: RoadSurfaceCls,
}
impl Lines {
    /// both lines are detected
    pub fn is_both_detected(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }
    /// Right line is detected && dashed
    pub fn is_right_dashed(&self) -> bool {
        self.right
            .as_ref()
            .map(|line| line.is_dashed())
            .unwrap_or_default()
    }
    /// Left line is detected && dashed
    pub fn is_left_dashed(&self) -> bool {
        self.left
            .as_ref()
            .map(|line| line.is_dashed())
            .unwrap_or_default()
    }

    /// is left line crossed (using COG)
    pub fn is_left_crossed(&self) -> bool {
        self.left
            .as_ref()
            .map(|line| line.ego_pos.fr.lt(&0.0))
            .unwrap_or_default()
    }

    /// is right line crossed (using COG)
    pub fn is_right_crossed(&self) -> bool {
        self.right
            .as_ref()
            .map(|line| line.ego_pos.fl.gt(&0.0))
            .unwrap_or_default()
    }
}
macro_rules! get_line {
    ($n0:ident, $n1:ident, $e:expr, $n2:ident) => {
        $n1.and_then(|boarder| {
            if boarder.valid {
                let index: usize = boarder.lineidx as usize;
                let line = $n0.line.get(index);
                let pos = $n0.egolineposition.get(index);
                let line_cross = $n2.linecrossing.get(index);
                line.and_then(|line| pos.and_then(|pos| line_cross.map(|line_cross| (line, pos, $e, line_cross).into())))
            } else {
                None
            }
        })
    };
}

impl From<(&roadobjectcoll_extended_t, &linecrossingcoll_t)> for Lines {
    fn from((roadobj, line_cross_coll): (&roadobjectcoll_extended_t, &linecrossingcoll_t)) -> Self {
        use LaneBorderSideType::*;

        const RIGHT_ROLA: [RoadObjectLaneAssociation; 3] = [
            RoadObjectLaneAssociation::ROLA_RIGHT1_LANE,
            RoadObjectLaneAssociation::ROLA_RIGHT2_LANE,
            RoadObjectLaneAssociation::ROLA_RIGHT3_LANE,
        ];
        const LEFT_ROLA: [RoadObjectLaneAssociation; 3] = [
            RoadObjectLaneAssociation::ROLA_LEFT1_LANE,
            RoadObjectLaneAssociation::ROLA_LEFT2_LANE,
            RoadObjectLaneAssociation::ROLA_LEFT3_LANE,
        ];
        // check timestamp
        if roadobj.timestamp_prediction != line_cross_coll.timestamp_prediction {
            log::error!(
                "RO and LineCrossing collections have different timestamps {} vs {}",
                roadobj.timestamp_prediction,
                line_cross_coll.timestamp_prediction
            );
            return Self {
                timestamp: Duration::from_millis(roadobj.timestamp_prediction.into()),
                left: None,
                right: None,
                leftleft: None,
                rightright: None,
                nb_lanes: 0,
                road_surface_cls: RoadSurfaceCls::NotUsed,
            };
        }
        // check dataqualifier
        if !matches!(roadobj.dataqualifier, DataQualifier::DQ_NORMAL) {
            return Self {
                timestamp: Duration::from_millis(roadobj.timestamp_prediction.into()),
                left: None,
                right: None,
                leftleft: None,
                rightright: None,
                nb_lanes: 0,
                road_surface_cls: RoadSurfaceCls::NotUsed,
            };
        }
        // first we find the ego_lane, if nothing is defined we do not have lines
        let Some(ego_lane) = roadobj
            .lane
            .iter()
            .take(roadobj.nblane.into())
            .find(|lane| lane.laneassociation == RoadObjectLaneAssociation::ROLA_EGO_LANE)
        else {
            return Self {
                timestamp: Duration::from_millis(roadobj.timestamp_prediction.into()),
                left: None,
                right: None,
                leftleft: None,
                rightright: None,
                nb_lanes: 0,
                road_surface_cls: RoadSurfaceCls::NotUsed,
            };
        };
        let laneboarder = ego_lane.laneborder.as_ref();
        let leftleft = laneboarder.get(LBST_LEFT_BOUNDARY as usize);
        let mut leftleft: Option<Line> =
            get_line!(roadobj, leftleft, LinePos::LeftLeft, line_cross_coll);
        let left = laneboarder.get(LBST_LEFT_MARKING as usize);
        let mut left: Option<Line> = get_line!(roadobj, left, LinePos::Left, line_cross_coll);
        let right = laneboarder.get(LBST_RIGHT_MARKING as usize);
        let mut right: Option<Line> = get_line!(roadobj, right, LinePos::Right, line_cross_coll);
        let rightright = laneboarder.get(LBST_RIGHT_BOUNDARY as usize);
        let mut rightright: Option<Line> =
            get_line!(roadobj, rightright, LinePos::RightRight, line_cross_coll);
        // handle case of RoadEdge without Marking
        if left.is_none() {
            left = leftleft.take();
            if let Some(left) = left.as_mut() {
                left.pos = LinePos::Left;
            }
        }
        if right.is_none() {
            right = rightright.take();
            if let Some(right) = right.as_mut() {
                right.pos = LinePos::Right;
            }
        }
        // In case left left is not found try to find it in the left ego lane
        for rola in LEFT_ROLA {
            if leftleft.is_some() {
                break;
            }
            if let Some(ego_left) = roadobj
                .lane
                .iter()
                .take(roadobj.nblane.into())
                .find(|lane| lane.laneassociation == rola)
            {
                let laneboarder = ego_left.laneborder.as_ref();
                for boarder in [
                    LBST_RIGHT_BOUNDARY,
                    LBST_RIGHT_MARKING,
                    LBST_LEFT_MARKING,
                    LBST_LEFT_BOUNDARY,
                ] {
                    let line = laneboarder.get(boarder as usize);
                    let line: Option<Line> =
                        get_line!(roadobj, line, LinePos::LeftLeft, line_cross_coll);
                    if let Some(line) = line {
                        if left.as_ref().map(|v| v.id).ne(&Some(line.id)) {
                            leftleft = Some(line);
                            break;
                        }
                    }
                }
            }
        }
        // In case rigth rigth is not found try to find it in the rigth ego lane
        for rola in RIGHT_ROLA {
            if rightright.is_some() {
                break;
            }
            if let Some(ego_right) = roadobj
                .lane
                .iter()
                .take(roadobj.nblane.into())
                .find(|lane| lane.laneassociation == rola)
            {
                let laneboarder = ego_right.laneborder.as_ref();
                for boarder in [
                    LBST_LEFT_BOUNDARY,
                    LBST_LEFT_MARKING,
                    LBST_RIGHT_MARKING,
                    LBST_RIGHT_BOUNDARY,
                ] {
                    let line = laneboarder.get(boarder as usize);
                    let line: Option<Line> =
                        get_line!(roadobj, line, LinePos::RightRight, line_cross_coll);
                    if let Some(line) = line {
                        if right.as_ref().map(|v| v.id).ne(&Some(line.id)) {
                            rightright = Some(line);
                            break;
                        }
                    }
                }
            }
        }
        // finally switch left left/rigth rigth to left/right if needed
        if left.is_none() {
            left = leftleft.take();
            if let Some(left) = left.as_mut() {
                left.pos = LinePos::Left;
            }
        }
        if right.is_none() {
            right = rightright.take();
            if let Some(right) = right.as_mut() {
                right.pos = LinePos::Right;
            }
        }
        Self {
            timestamp: Duration::from_millis(roadobj.timestamp_prediction.into()),
            left,
            right,
            leftleft,
            rightright,
            nb_lanes: roadobj.nblane,
            road_surface_cls: (&roadobj.roadsurface).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_type_from_road_marking_type() {
        assert_eq!(
            LineType::from(&RoadMarkingType::RMT_CENTRE_LINE_DASHED_MARKING),
            LineType::Dashed
        );
        assert_eq!(LineType::from(&RoadMarkingType::RMT_SOLID), LineType::Solid);
        assert_eq!(
            LineType::from(&RoadMarkingType::RMT_UNKNOWN),
            LineType::Unknown
        );
    }

    #[test]
    fn test_clothoid_segment_is_straight() {
        let straight_clothoid = ClothoidSegment::new(0.0, 0.0, 0.0, 0.0, 0.0, 10.0);
        let curved_clothoid = ClothoidSegment::new(0.0, 0.0, 0.0, 0.1, 0.2, 10.0);

        assert!(straight_clothoid.is_straight());
        assert!(!curved_clothoid.is_straight());
    }

    #[test]
    fn test_line_is_solid_and_dashed() {
        let solid_line = Line {
            id: 1,
            line_type: LineType::Solid,
            pos: LinePos::Left,
            existence_prob: 9.0,
            width: Length::new::<meter>(3.5),
            ego_pos: LineDis::default(),
            first_clothoid: ClothoidSegment::default(),
            second_clothoid: None,
            ttlc: Default::default(),
        };

        let dashed_line = Line {
            id: 2,
            line_type: LineType::Dashed,
            pos: LinePos::Right,
            existence_prob: 8.0,
            width: Length::new::<meter>(2.5),
            ego_pos: LineDis::default(),
            first_clothoid: ClothoidSegment::default(),
            second_clothoid: None,
            ttlc: Default::default(),
        };

        assert!(solid_line.is_solid());
        assert!(!solid_line.is_dashed());
        assert!(!dashed_line.is_solid());
        assert!(dashed_line.is_dashed());
    }

    #[test]
    fn test_lines_detection_and_crossing() {
        let left_line = Line {
            id: 1,
            line_type: LineType::Solid,
            pos: LinePos::Left,
            existence_prob: 9.0,
            width: Length::new::<meter>(3.5),
            ego_pos: LineDis {
                fl: 0.0,
                fr: -1.0,
                rl: 0.0,
                rr: 0.0,
                cog: -1.0,
            },
            first_clothoid: ClothoidSegment::default(),
            second_clothoid: None,
            ttlc: Default::default(),
        };

        let right_line = Line {
            id: 2,
            line_type: LineType::Dashed,
            pos: LinePos::Right,
            existence_prob: 8.0,
            width: Length::new::<meter>(2.5),
            ego_pos: LineDis {
                fl: 1.0,
                fr: 0.0,
                rl: 0.0,
                rr: 0.0,
                cog: 1.0,
            },
            first_clothoid: ClothoidSegment::default(),
            second_clothoid: None,
            ttlc: Default::default(),
        };

        let lines = Lines {
            timestamp: Duration::ZERO,
            left: Some(left_line.clone()),
            right: Some(right_line.clone()),
            leftleft: None,
            rightright: None,
            nb_lanes: 0,
            ..Default::default()
        };

        assert!(lines.is_both_detected());
        assert!(lines.is_right_dashed());
        assert!(!lines.is_left_dashed());
        assert!(lines.is_left_crossed());
        assert!(lines.is_right_crossed());
    }
}
