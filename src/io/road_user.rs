//! # Road User
//! This module defines the Acc Targets strategies
//!
use core::{f32::consts::PI, ops::Add, time::Duration};

use super::{ego::Ego, meter, mps, mps2, rad, sec, Acceleration, Angle, Length, Time, Velocity};
use crate::{io::kph, values::Hysterisis};
use acc_interface::{
    calibrations::Calibration,
    datatypes::{
        pmo_extended_t, pmocoll_extended_t, roadusercoll_t, DataQualifier, ObjectLaneAssociation,
        PMOClassificationType, PmoDynamicProperty, RoadUserCorridor, RoadUserId, RoadUserPosLongi,
        SideTargetT,
    },
};
use core::ops::{Div, Mul, Sub};
use num_derive::{FromPrimitive, ToPrimitive};
#[allow(unused_imports)]
use num_traits::Float;
use num_traits::{FromPrimitive, ToPrimitive};

/// Maximum front number of road users
pub const ACC_RU_NB: usize = 12;
/// distance between ego Rear Axle and Front Bumper
pub const P_TNG_EGO_RAXE_TO_FRBUMPER: f32 = 3.494;

/// TODO: Add calib param for this
pub const MIN_FOLLOW_UP_TIME: f32 = 0.5;
/// Pmo Dyn Prop
#[derive(Debug, Clone, Default, PartialEq)]
pub enum MouvementType {
    /// Targe is moving in same direction
    MovingSameDir,
    /// Target is moving in opposite direction
    MovingOppositeDir,
    /// Stopped
    Stopped,
    /// Standing
    Standing,
    /// Crossing Left
    CrossingLeft,
    /// Crossing Right
    CrossingRight,
    /// Unknown
    #[default]
    Unknown,
}

/// The Type of the final Target For the ACC Regulation
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Ord, PartialOrd, Eq, ToPrimitive, FromPrimitive,
)]
pub enum TargetType {
    /// Front : object 0
    Front1 = 0,
    /// Front Front Target : object 1
    Front2 = 1,
    /// Front FF Target : object 2
    Front3 = 2,
    /// Left 1
    Left1 = 3,
    /// Left 2
    Left2 = 4,
    /// Left 3
    Left3 = 5,
    /// Right 1
    Right1 = 6,
    /// Right 2
    Right2 = 7,
    /// Right 3
    Right3 = 8,
    /// Rear target in ego corridor
    Rear = 9,
    /// Rear Left
    RearLeft = 10,
    /// Rear Right
    RearRight = 11,
    #[default]
    /// No Target
    NoTarget = 12,
}

/// Secondary Target Display Type
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Ord, PartialOrd, Eq, ToPrimitive, FromPrimitive,
)]
pub enum SideTargetClass {
    /// No target
    #[default]
    NoTarget = 0,
    /// Right Lane Motor Bike
    RightMoto = 1,
    /// Left Lane MotorBike
    LeftMoto = 2,
    /// Right Lane Truck
    RightTruck = 3,
    /// Left Lane Truck
    LeftTruck = 4,
    /// Right Lane Car
    RightCar = 5,
    /// Left Lane Car
    LeftCar = 6,
    /// Right Lane Van
    RightVan = 7,
    /// Left Lane Van
    LeftVan = 8,
}

impl From<&SideTargetClass> for SideTargetT {
    fn from(value: &SideTargetClass) -> Self {
        use SideTargetClass::*;
        match value {
            NoTarget => Self::STT_NO_TARGET,
            RightMoto => Self::STT_RIGHT_LANE_MOTOR_BIKE,
            LeftMoto => Self::STT_LEFT_LANE_MOTOR_BIKE,
            RightTruck => Self::STT_RIGHT_LANE_TRUCK,
            LeftTruck => Self::STT_LEFT_LANE_TRUCK,
            RightCar => Self::STT_RIGHT_LANE_CAR,
            LeftCar => Self::STT_LEFT_LANE_CAR,
            RightVan => Self::STT_RIGHT_LANE_VAN,
            LeftVan => Self::STT_RIGHT_LANE_VAN,
        }
    }
}

impl TargetType {
    /// Check Rear target
    pub const fn is_rear(&self) -> bool {
        use TargetType::*;
        matches!(self, RearLeft | RearRight | Rear)
    }
    /// Check Front target
    pub const fn is_front_same_lane(&self) -> bool {
        use TargetType::*;
        matches!(self, Front1 | Front2 | Front3)
    }
    /// Is Front and left
    pub const fn is_left_front(&self) -> bool {
        use TargetType::*;
        matches!(self, Left1 | Left2 | Left3)
    }
    /// is Front and right
    pub const fn is_right_front(&self) -> bool {
        use TargetType::*;
        matches!(self, Right1 | Right2 | Right3)
    }
}

/// Object Lane Position
#[derive(Debug, Clone, Default, PartialEq)]
pub enum LanePos {
    /// Same Lane as Ego
    EgoLane,
    /// Right Ego Lane
    Right,
    /// Left Ego Lane
    Left,
    /// Ego right
    EgoRight,
    /// Ego Left
    EgoLeft,
    /// Not used for Acc target selection
    #[default]
    NotUsed,
}

/// Su Cut in/out
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SuCutInOut {
    /// Cut in target
    CutIn,
    /// Right Ego Lane
    CutOut,
    /// No CutIn / CutOut
    #[default]
    NoTarget,
}

impl From<ObjectLaneAssociation> for LanePos {
    fn from(value: ObjectLaneAssociation) -> Self {
        use ObjectLaneAssociation::*;
        match value {
            OLA_EGO_LANE => Self::EgoLane,
            OLA_RIGHT1_LANE => Self::Right,
            OLA_LEFT1_LANE => Self::Left,
            OLA_EGO_LEFT1_LANE => Self::EgoLeft,
            OLA_EGO_RIGHT1_LANE => Self::EgoRight,
            _ => Self::NotUsed,
        }
    }
}

impl From<PmoDynamicProperty> for MouvementType {
    fn from(value: PmoDynamicProperty) -> Self {
        match value {
            PmoDynamicProperty::PDP_MOVING_FOLLOWING => MouvementType::MovingSameDir,
            PmoDynamicProperty::PDP_MOVING_ONCOMING => MouvementType::MovingOppositeDir,
            PmoDynamicProperty::PDP_STANDING => MouvementType::Standing,
            PmoDynamicProperty::PDP_STOPPED_FOLLOWING
            | PmoDynamicProperty::PDP_STOPPED_ONCOMING => MouvementType::Stopped,
            PmoDynamicProperty::PDP_CROSSING_LEFT => MouvementType::CrossingLeft,
            PmoDynamicProperty::PDP_CROSSING_RIGHT => MouvementType::CrossingRight,
            _ => MouvementType::Unknown,
        }
    }
}

/// Road user class, used for ACC target strategie selection
#[derive(Debug, Clone, Default, PartialEq)]
pub enum RuClass {
    /// Truck
    TRUCK,
    /// Car
    CAR,
    /// Van
    VAN,
    /// Motorbike
    MOTORBIKE,
    /// Pedestrian
    PEDESTRIAN,
    /// Obstacle
    Obstacle,
    /// Others
    Others,
    /// Unknown
    #[default]
    UNKNOWN,
}
impl From<PMOClassificationType> for RuClass {
    fn from(value: PMOClassificationType) -> Self {
        // from ru
        match value {
            PMOClassificationType::PMOCT_AMBULANCE | PMOClassificationType::PMOCT_VAN => {
                RuClass::VAN
            }
            PMOClassificationType::PMOCT_HEAVY_TRUCK
            | PMOClassificationType::PMOCT_TRUCK
            | PMOClassificationType::PMOCT_BUS
            | PMOClassificationType::PMOCT_TRAILER
            | PMOClassificationType::PMOCT_FIRE_ENGINE => RuClass::TRUCK,
            PMOClassificationType::PMOCT_CAR
            | PMOClassificationType::PMOCT_POLICE_CAR
            | PMOClassificationType::PMOCT_UTILITY_VEHICLE
            | PMOClassificationType::PMOCT_SANITATION_VEHICLE
            | PMOClassificationType::PMOCT_TRAFFIC_CONTROLLER
            | PMOClassificationType::PMOCT_OTHER_VEHICLE => RuClass::CAR,
            PMOClassificationType::PMOCT_MOTORBIKE
            | PMOClassificationType::PMOCT_ESCOOTER
            | PMOClassificationType::PMOCT_BICYCLE => RuClass::MOTORBIKE,
            PMOClassificationType::PMOCT_ANIMAL
            | PMOClassificationType::PMOCT_TRICYCLE
            | PMOClassificationType::PMOCT_PEDESTRIAN
            | PMOClassificationType::PMOCT_WHEELCHAIR => RuClass::PEDESTRIAN,
            PMOClassificationType::PMOCT_OBSTACLE => RuClass::Obstacle,
            PMOClassificationType::PMOCT_TRAIN
            | PMOClassificationType::PMOCT_OVERDRIVABLE
            | PMOClassificationType::PMOCT_UNDERDRIVABLE => RuClass::Others,
            PMOClassificationType::PMOCT_UNKNOWN => RuClass::UNKNOWN,
        }
    }
}

/// target needed signals for Acc
/// TODO: Split this
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Target {
    /// flag to track detection target
    pub target_detected: bool,
    /// target type
    pub class: TargetType,
    /// ru class
    pub ru_class: RuClass,
    /// pmo id
    pub id: u32,
    /// Time to collision longitudinal
    pub ttc_long: Time,
    /// Time to collision lateral
    pub ttc_lat: Time,
    /// distance between ego and target
    pub pos_long: Length,
    /// distance between ego and target
    pub pos_lat: Length,
    /// Distance to middle line
    pub dis_mid_line: Length,
    /// Longitudinal Velocity
    pub vx: Velocity,
    /// Lateral Velocity
    pub vy: Velocity,
    /// Longitudinal Acceleration
    pub ax: Acceleration,
    /// Lateral Acceleration
    pub ay: Acceleration,
    /// Object Lane postion
    pub lane_pos: LanePos,
    /// Mouvement Status
    pub mvt_status: MouvementType,
    /// is Cut in
    pub cut_in_out: SuCutInOut,
    /// Follow up time
    pub follow_up_time: Time,
    /// Time to lane crossing
    pub tlc: Time,
    /// Target Id changed
    pub id_changed: bool,
    /// target Width
    pub width: Length,
    /// Percentage side line
    pub percentage_side_line: [f32; 2],
    /// Yaw
    pub yaw: Angle,
}

impl Target {
    /// Front RU 0 & 1 condition
    pub const fn front_ru_check(poslongi: RoadUserPosLongi, poscorridor: RoadUserCorridor) -> bool {
        matches!(poslongi, RoadUserPosLongi::ROAD_USER_POS_LONGI_FRONT)
            && matches!(poscorridor, RoadUserCorridor::ROAD_USER_CORRIDOR_EGO)
    }
    /// Find right ru
    pub const fn right_ru_check(poslongi: RoadUserPosLongi, poscorridor: RoadUserCorridor) -> bool {
        matches!(poslongi, RoadUserPosLongi::ROAD_USER_POS_LONGI_FRONT)
            && matches!(poscorridor, RoadUserCorridor::ROAD_USER_CORRIDOR_RIGHT)
    }
    /// Left ru check
    pub const fn left_ru_check(poslongi: RoadUserPosLongi, poscorridor: RoadUserCorridor) -> bool {
        matches!(poslongi, RoadUserPosLongi::ROAD_USER_POS_LONGI_FRONT)
            && matches!(poscorridor, RoadUserCorridor::ROAD_USER_CORRIDOR_LEFT)
    }
    /// Front RU Rear 0 condition
    pub const fn rear_ru_check(poslongi: RoadUserPosLongi, poscorridor: RoadUserCorridor) -> bool {
        matches!(poslongi, RoadUserPosLongi::ROAD_USER_POS_LONGI_REAR)
            && matches!(poscorridor, RoadUserCorridor::ROAD_USER_CORRIDOR_EGO)
    }
    /// Front Left Rear  condition
    pub const fn left_rear_ru_check(
        poslongi: RoadUserPosLongi,
        poscorridor: RoadUserCorridor,
    ) -> bool {
        matches!(poslongi, RoadUserPosLongi::ROAD_USER_POS_LONGI_REAR)
            && matches!(poscorridor, RoadUserCorridor::ROAD_USER_CORRIDOR_LEFT)
    }
    /// Right RU Rear condition
    pub const fn right_rear_ru_check(
        poslongi: RoadUserPosLongi,
        poscorridor: RoadUserCorridor,
    ) -> bool {
        matches!(poslongi, RoadUserPosLongi::ROAD_USER_POS_LONGI_REAR)
            && matches!(poscorridor, RoadUserCorridor::ROAD_USER_CORRIDOR_RIGHT)
    }

    /// Compute signed speed
    pub fn signed_speed(&self) -> Velocity {
        let vx = self.vx.get::<mps>().powi(2);
        let vy = self.vy.get::<mps>().powi(2);
        let speed = (vx + vy).sqrt();
        Velocity::new::<mps>(self.vx.get::<mps>().signum() * speed)
    }

    /// Compute signed acceleration
    pub fn signed_accel(&self) -> Acceleration {
        let ax = self.ax.get::<mps2>().powi(2);
        let ay = self.ay.get::<mps2>().powi(2);
        let accel = (ax + ay).sqrt();
        Acceleration::new::<mps2>(self.ax.get::<mps2>().signum() * accel)
    }

    /// Check if it's a front target
    pub fn in_ego_corridor(&self) -> bool {
        matches!(
            self.class,
            TargetType::Front1 | TargetType::Front2 | TargetType::Front3
        )
    }

    /// is target stopped
    pub fn stopped(&self, hys: &Hysterisis<f32>, calib: &Calibration) -> bool {
        use RuClass::*;
        (matches!(&self.ru_class, CAR | MOTORBIKE | TRUCK)
            || (self.ru_class.eq(&UNKNOWN)
                && self
                    .width
                    .get::<meter>()
                    .gt(&calib.P_tng_m_Acc_StoppedFrTargetMinWidth)))
            && (self.mvt_status.eq(&MouvementType::Stopped)
                || (self.mvt_status.eq(&MouvementType::Standing)
                    && self
                        .signed_speed()
                        .get::<kph>()
                        .le(&calib.P_tng_kph_Acc_StationnaryTargetSpeedManagement))
                || hys.is_low())
            && self.target_detected
    }
    /// is target braking
    pub fn braking(&self) -> bool {
        self.ax.get::<mps2>() < 0.0
    }
    /// Compute absolute distance
    pub fn distance_abs(&self) -> Length {
        let dx = self.pos_lat.get::<meter>().powf(2.0);
        let dy = self.pos_long.get::<meter>().powf(2.0);
        let dist = (dx + dy).sqrt();
        Length::new::<meter>(dist)
    }
    /// Frame conversion
    fn rear_to_front_bump(pmo: &pmo_extended_t, tg_type: &TargetType) -> Length {
        let c_yaw = pmo.pmobject.position.yaw.cos();
        let length = pmo.pmobject.boundingbox.extent[0];
        Length::new::<meter>(if tg_type.is_rear() {
            pmo.pmobject.position.position[0] + 0.5 * length * c_yaw
        } else {
            pmo.pmobject.position.position[0] - P_TNG_EGO_RAXE_TO_FRBUMPER - 0.5 * length * c_yaw
        })
    }

    /// Accel estimation using Intelligent driver model
    /// https://en.wikipedia.org/wiki/Intelligent_driver_model
    pub fn accel_estimation(&self, ego: &Ego, v_desired: f32, time_gap: f32) -> f32 {
        let v_ego = ego.dynamics.signed_speed().get::<mps>();
        let delta_v = v_ego - self.signed_speed().get::<mps>();
        let gap_cur = self.pos_long.get::<meter>();
        let s0 = 3.0;
        let a_max = 5.0;
        let b_comf = 5.0;
        let delta = 4.0;

        let s_star = (s0
            + ((v_ego * time_gap) + ((v_ego * delta_v) / (2.0 * (a_max * b_comf).sqrt()))))
        .max(0.0);

        a_max * (1.0 - (v_ego / v_desired).powf(delta) - (s_star / gap_cur).powi(2))
    }
}

impl From<(&pmo_extended_t, TargetType)> for Target {
    fn from(value: (&pmo_extended_t, TargetType)) -> Self {
        let (pmo, target_class) = value;
        Self {
            target_detected: true,
            id: pmo.pmobject.status.objectid,
            ru_class: pmo.mostprobaclass.into(),
            pos_long: Target::rear_to_front_bump(pmo, &target_class),
            pos_lat: Length::new::<meter>(pmo.pmobject.position.position[1]),
            dis_mid_line: Length::new::<meter>(pmo.dist2midlane),
            ttc_long: Time::new::<sec>(pmo.ttclongi),
            ttc_lat: Time::new::<sec>(pmo.ttclat),
            vx: Velocity::new::<mps>(pmo.pmobject.dynamics.velocity[0]),
            vy: Velocity::new::<mps>(pmo.pmobject.dynamics.velocity[1]),
            ax: Acceleration::new::<mps2>(pmo.pmobject.dynamics.acceleration[0]),
            ay: Acceleration::new::<mps2>(pmo.pmobject.dynamics.acceleration[1]),
            lane_pos: pmo.pmobject.lanerelatedinfo.laneassociation.into(),
            mvt_status: pmo.pmodynprop.into(),
            class: target_class,
            cut_in_out: if pmo.cutinflag {
                SuCutInOut::CutIn
            } else if pmo.cutoutflag {
                SuCutInOut::CutOut
            } else {
                SuCutInOut::NoTarget
            },
            follow_up_time: Time::new::<sec>(0.0),
            tlc: Time::new::<sec>(pmo.tlc),
            id_changed: false,
            width: Length::new::<meter>(pmo.pmobject.boundingbox.extent[1]),
            percentage_side_line: pmo.pmobject.lanerelatedinfo.percentagesidelane,
            yaw: Angle::new::<rad>(pmo.pmobject.boundingbox.extent[0]),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
/// Required Action to do for the ACC Target Following
pub enum RequiredAction {
    /// Reduce the distance to the Target
    ReduceDistance,
    /// Use the normal Target Regulation Strategie
    #[default]
    NormalTargetRegulation,
    /// Regulate on set speed
    RegulOnVset,
    /// Set Follow up time
    SetFollowUpTime(Time),
    /// Stop the car
    Stop(Length),
    /// Limit Accel
    LimitAccel,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Distance to the target classes
pub enum TargetDistanceType {
    /// No relevant Target detected
    #[default]
    NoTarget = 0,
    /// Far Target detected
    Far = 1,
    /// Middle Target detected
    Middle = 2,
    /// Close Target detected
    Close1 = 3,
    /// Closest Target to The ego
    Close2 = 4,
    /// Unspecified
    Unspecified = 5,
}

/// All Acc Relevant targets needed signals for Acc
#[derive(Debug, Default, Clone)]
pub struct Targets {
    /// Timestamp
    pub timestamp: Duration,
    /// flag to track detection target
    pub ru: [Target; ACC_RU_NB],
    /// Relevant Target Index,
    pub selected_target: TargetType,
    /// Required Action :
    pub required_action: RequiredAction,
    /// Autorestart Obstacle
    pub autorestart_obstacle: bool,
    /// Autorestart Pedestrian
    pub autorestart_ped: bool,
}

impl Targets {
    /// Get Target ID
    #[inline(always)]
    pub fn get_id(&self, tg_type: TargetType) -> Option<u32> {
        self.get_target(tg_type).map(|tgt| tgt.id)
    }
    /// check if id changed
    pub fn id_changed(&self, tg: TargetType) -> bool {
        self.get_target(tg)
            .map(|tg| tg.id_changed)
            .unwrap_or_default()
    }
    /// Get target Type
    pub fn get_side_tg_type(&self, tg_type: TargetType) -> SideTargetClass {
        use RuClass::*;
        let Some(tg) = self.get_target(tg_type) else {
            return SideTargetClass::NoTarget;
        };
        if tg_type.is_left_front() {
            match tg.ru_class {
                CAR => SideTargetClass::LeftCar,
                MOTORBIKE => SideTargetClass::LeftMoto,
                TRUCK => SideTargetClass::LeftTruck,
                VAN => SideTargetClass::LeftVan,
                _ => SideTargetClass::NoTarget,
            }
        } else if tg_type.is_right_front() {
            match tg.ru_class {
                CAR => SideTargetClass::RightCar,
                MOTORBIKE => SideTargetClass::RightMoto,
                TRUCK => SideTargetClass::RightTruck,
                VAN => SideTargetClass::RightVan,
                _ => SideTargetClass::NoTarget,
            }
        } else {
            SideTargetClass::NoTarget
        }
    }
    /// Distance between Ego and target
    pub fn calculate_distance(target: &Target) -> f32 {
        (target.pos_long.get::<meter>().powi(2) + target.pos_lat.get::<meter>().powi(2)).sqrt()
    }

    /// Return a reference to the target if targets.ru.get(target_type.index()) is Some, or a reference to a default Target otherwise
    #[inline(always)]
    pub fn get_target(&self, target_type: TargetType) -> Option<&Target> {
        self.ru
            .get(target_type.to_usize().unwrap_or_default())
            .filter(|tg| {
                tg.target_detected
                    && (tg.ru_class.ne(&RuClass::UNKNOWN) | tg.class.eq(&TargetType::Front1))
            })
    }
    /// REQ_SYS_S23-FACE_Design_12860
    /// Check if an obstacle at (x, y) is inside the Field of View (FoV)
    /// using polar coordinates.
    pub fn ped_inside_fov(&mut self, pmo: &pmo_extended_t, calib: &Calibration) {
        if self.autorestart_ped
            || pmo
                .mostprobaclass
                .ne(&PMOClassificationType::PMOCT_PEDESTRIAN)
        {
            return;
        }
        let (x, y) = (
            pmo.pmobject.position.position[0],
            pmo.pmobject.position.position[1],
        );

        let theta_min = (-calib.P_tng_degree_Acc_CameraFieldOfView.div(2.0)).mul(PI.div(180.0));
        let theta_max = (calib.P_tng_degree_Acc_CameraFieldOfView.div(2.0)).mul(PI.div(180.0));
        if x < 0.0 {
            return;
        }

        let r_obstacle = (x.powi(2) + y.powi(2)).sqrt();
        let theta_obstacle = y.atan2(x);

        if theta_obstacle < theta_min || theta_obstacle > theta_max {
            return;
        }
        self.autorestart_ped = r_obstacle <= calib.P_tng_m_Acc_AutorestartAera2Distance;
    }

    /// REQ_SYS_S23-FACE_Design_12859
    /// Checks if an obstacle at (x, y) is inside the monitored zone.
    pub fn obstacle_inside_monitored_zone(&mut self, x: f32, length: Option<Length>) {
        if self.autorestart_obstacle {
            return;
        }
        if let Some(len) = length {
            let x_min = 0.0;
            let x_max = len.get::<meter>();
            self.autorestart_obstacle = x_min <= x && x <= x_max;
        }
    }
    /// get Closest Obstacle
    fn get_closest_obstacle(
        &self,
        pmo: &pmo_extended_t,
        obs: &mut (Option<f32>, f32),
        calib: &Calibration,
    ) {
        if pmo
            .mostprobaclass
            .eq(&PMOClassificationType::PMOCT_OBSTACLE)
            && !self.autorestart_obstacle
        {
            let lowest_x = &mut obs.1;
            let (x, y) = (
                pmo.pmobject.position.position[0],
                pmo.pmobject.position.position[1],
            );
            let y_min = calib
                .P_tng_m_ownVehicleWidth
                .div(-2.0)
                .sub(calib.P_tng_m_Acc_AutoGoDetectionMarginR);
            let y_max = calib
                .P_tng_m_ownVehicleWidth
                .div(2.0)
                .add(calib.P_tng_m_Acc_AutoGoDetectionMarginL);
            if y > y_min && y < y_max && x < *lowest_x {
                *lowest_x = x;
                obs.0 = Some(pmo.pmobject.position.position[0]);
            }
        }
    }
}

#[inline(always)]
fn update_index(pmo_idx: &mut [Option<u32>], tgt: TargetType, id: u32) {
    if let Some(idx) = pmo_idx.get_mut(tgt as usize) {
        *idx = Some(id);
    }
}
impl From<(&roadusercoll_t, &pmocoll_extended_t, &Calibration)> for Targets {
    fn from(value: (&roadusercoll_t, &pmocoll_extended_t, &Calibration)) -> Self {
        use TargetType::*;
        const _: () = assert!(NoTarget as usize == ACC_RU_NB, "ACC_RU_NB value is wrong");
        let mut targets = Targets {
            timestamp: Duration::from_millis(value.1.timestamp_prediction.into()),
            ..Default::default()
        };
        // check timestamp
        if value.0.timestamp_prediction != value.1.timestamp_prediction {
            log::error!(
                "RU and PMO have a different timestamps {} vs {}",
                value.0.timestamp_prediction,
                value.1.timestamp_prediction
            );
        }
        // check dataqualifier
        if !matches!(value.1.dataqualifier, DataQualifier::DQ_NORMAL) {
            return targets;
        }
        let mut closest_obstacle: (Option<f32>, f32) = (None, f32::MAX);
        let mut pmo_idx: [Option<u32>; ACC_RU_NB] = [None; ACC_RU_NB];
        value
            .0
            .roaduser
            .iter()
            .take(value.0.nbroaduser.into())
            .for_each(|ru| match ru.ru_id {
                RoadUserId::ROAD_USER_ID_00 => {
                    if Target::front_ru_check(ru.poslongi, ru.poscorridor) {
                        update_index(pmo_idx.as_mut_slice(), Front1, ru.pmo_id);
                    } else if Target::right_ru_check(ru.poslongi, ru.poscorridor) {
                        update_index(pmo_idx.as_mut_slice(), Right1, ru.pmo_id);
                    } else if Target::left_ru_check(ru.poslongi, ru.poscorridor) {
                        update_index(pmo_idx.as_mut_slice(), Left1, ru.pmo_id);
                    } else if Target::rear_ru_check(ru.poslongi, ru.poscorridor) {
                        update_index(pmo_idx.as_mut_slice(), Rear, ru.pmo_id);
                    } else if Target::left_rear_ru_check(ru.poslongi, ru.poscorridor) {
                        update_index(pmo_idx.as_mut_slice(), RearLeft, ru.pmo_id);
                    } else if Target::right_rear_ru_check(ru.poslongi, ru.poscorridor) {
                        update_index(pmo_idx.as_mut_slice(), RearRight, ru.pmo_id);
                    }
                }
                RoadUserId::ROAD_USER_ID_01 => {
                    if Target::front_ru_check(ru.poslongi, ru.poscorridor) {
                        update_index(pmo_idx.as_mut_slice(), Front2, ru.pmo_id);
                    } else if Target::right_ru_check(ru.poslongi, ru.poscorridor) {
                        update_index(pmo_idx.as_mut_slice(), Right2, ru.pmo_id);
                    } else if Target::left_ru_check(ru.poslongi, ru.poscorridor) {
                        update_index(pmo_idx.as_mut_slice(), Left2, ru.pmo_id);
                    }
                }
                RoadUserId::ROAD_USER_ID_02 => {
                    if Target::front_ru_check(ru.poslongi, ru.poscorridor) {
                        update_index(pmo_idx.as_mut_slice(), Front3, ru.pmo_id);
                    } else if Target::right_ru_check(ru.poslongi, ru.poscorridor) {
                        update_index(pmo_idx.as_mut_slice(), Right3, ru.pmo_id);
                    } else if Target::left_ru_check(ru.poslongi, ru.poscorridor) {
                        update_index(pmo_idx.as_mut_slice(), Left3, ru.pmo_id);
                    }
                }
                _ => {}
            });
        value
            .1
            .pmo_extended
            .iter()
            .take(value.1.nbpmobject.into())
            .for_each(|pmo_ex| {
                targets.get_closest_obstacle(pmo_ex, &mut closest_obstacle, value.2);
                targets.ped_inside_fov(pmo_ex, value.2);
                pmo_idx
                    .iter()
                    .enumerate()
                    .filter_map(|(index, pmo_id)| match pmo_id {
                        Some(pmo_id) if pmo_ex.pmobject.status.objectid.eq(pmo_id) => Some(index),
                        _ => None,
                    })
                    .for_each(|index| {
                        if let (Some(tg_type), Some(ru)) =
                            (TargetType::from_usize(index), targets.ru.get_mut(index))
                        {
                            *ru = (pmo_ex, tg_type).into();
                        }
                    });
            });
        targets.obstacle_inside_monitored_zone(
            closest_obstacle.1,
            targets.get_target(Front1).map(|tg| tg.pos_long),
        );
        targets
    }
}

#[allow(clippy::indexing_slicing)]
#[cfg(test)]
mod tests {
    use crate::io::road_user::{RuClass, Target, Targets};
    use crate::io::{
        meter, mps, mps2, road_user::SuCutInOut, sec, Acceleration, Length, Time, Velocity,
    };
    use acc_interface::{
        calibrations::Calibration,
        datatypes::{
            pmo_extended_t, pmocoll_extended_t, roaduser_t, roadusercoll_t, ObjectLaneAssociation,
            PmoDynamicProperty, RoadUserCorridor, RoadUserDirection, RoadUserId, RoadUserPosLongi,
        },
    };

    use super::{LanePos, MouvementType, TargetType};

    #[test]
    fn test_from_object_lane_association_to_lane_pos() {
        let mut object_lane_association = ObjectLaneAssociation::OLA_EGO_LANE;
        let lane_pos: LanePos = object_lane_association.into();
        assert_eq!(lane_pos, LanePos::EgoLane);
        object_lane_association = ObjectLaneAssociation::OLA_RIGHT1_LANE;
        let lane_pos: LanePos = object_lane_association.into();
        assert_eq!(lane_pos, LanePos::Right);
        object_lane_association = ObjectLaneAssociation::OLA_LEFT1_LANE;
        let lane_pos: LanePos = object_lane_association.into();
        assert_eq!(lane_pos, LanePos::Left);
        object_lane_association = ObjectLaneAssociation::OLA_LEFT3_LEFT4_LANE;
        let lane_pos: LanePos = object_lane_association.into();
        assert_eq!(lane_pos, LanePos::NotUsed);
    }
    #[test]
    fn test_from_pmo_dynamic_property_to_mouvement_type() {
        let mut pmo_dynamic_property = PmoDynamicProperty::PDP_MOVING_FOLLOWING;
        let mouvement_type: MouvementType = pmo_dynamic_property.into();
        assert_eq!(mouvement_type, MouvementType::MovingSameDir);
        pmo_dynamic_property = PmoDynamicProperty::PDP_MOVING_ONCOMING;
        let mouvement_type: MouvementType = pmo_dynamic_property.into();
        assert_eq!(mouvement_type, MouvementType::MovingOppositeDir);
        pmo_dynamic_property = PmoDynamicProperty::PDP_STOPPED_ONCOMING;
        let mouvement_type: MouvementType = pmo_dynamic_property.into();
        assert_eq!(mouvement_type, MouvementType::Stopped);
        pmo_dynamic_property = PmoDynamicProperty::PDP_CROSSING_LEFT;
        let mouvement_type: MouvementType = pmo_dynamic_property.into();
        assert_eq!(mouvement_type, MouvementType::CrossingLeft);
        pmo_dynamic_property = PmoDynamicProperty::PDP_CROSSING_RIGHT;
        let mouvement_type: MouvementType = pmo_dynamic_property.into();
        assert_eq!(mouvement_type, MouvementType::CrossingRight);
        pmo_dynamic_property = PmoDynamicProperty::PDP_NOT_USED;
        let mouvement_type: MouvementType = pmo_dynamic_property.into();
        assert_eq!(mouvement_type, MouvementType::Unknown);
    }
    #[test]
    fn test_front_ru_check() {
        let mut ru = roaduser_t {
            poslongi: RoadUserPosLongi::ROAD_USER_POS_LONGI_FRONT,
            poscorridor: RoadUserCorridor::ROAD_USER_CORRIDOR_EGO,
            ..Default::default()
        };
        assert!(Target::front_ru_check(ru.poslongi, ru.poscorridor));
        ru.poslongi = RoadUserPosLongi::ROAD_USER_POS_LONGI_REAR;
        assert!(!Target::front_ru_check(ru.poslongi, ru.poscorridor));
    }
    #[test]
    fn test_to_target() {
        let mut pmo = pmo_extended_t::default();
        let target_class = TargetType::Front1;
        pmo.pmobject.status.objectid = 4;
        pmo.pmobject.position.position[0] = 47.6;
        pmo.pmobject.position.position[1] = 854.1;
        pmo.dist2midlane = 27.9;
        pmo.ttclongi = 47.0;
        pmo.ttclat = 17.8;
        pmo.pmobject.dynamics.velocity[0] = 85.6;
        pmo.pmobject.dynamics.velocity[1] = 5.4;
        pmo.pmobject.dynamics.acceleration[0] = 65.2;
        pmo.pmobject.dynamics.acceleration[1] = 4.3;
        pmo.pmobject.lanerelatedinfo.laneassociation = ObjectLaneAssociation::OLA_EGO_LANE;
        pmo.pmobject.boundingbox.extent[0] = 3.0;
        pmo.pmodynprop = PmoDynamicProperty::PDP_CROSSING_LEFT;
        pmo.cutinflag = true;
        pmo.cutoutflag = false;
        let target: Target = (&pmo, target_class).into();
        assert!(target.target_detected);
        assert_eq!(target.id, 4);
        assert_eq!(
            target.pos_long,
            Length::new::<meter>(47.6 - 3.494 - 0.5 * pmo.pmobject.boundingbox.extent[0])
        );
        assert_eq!(target.pos_lat, Length::new::<meter>(854.1));
        assert_eq!(target.dis_mid_line, Length::new::<meter>(27.9));
        assert_eq!(target.ttc_long, Time::new::<sec>(47.0));
        assert_eq!(target.ttc_lat, Time::new::<sec>(17.8));
        assert_eq!(target.vx, Velocity::new::<mps>(85.6));
        assert_eq!(target.vy, Velocity::new::<mps>(5.4));
        assert_eq!(target.ax, Acceleration::new::<mps2>(65.2));
        assert_eq!(target.ay, Acceleration::new::<mps2>(4.3));
        assert_eq!(target.lane_pos, LanePos::EgoLane);
        assert_eq!(target.mvt_status, MouvementType::CrossingLeft);
        assert_eq!(target.class, TargetType::Front1);
        assert_eq!(target.cut_in_out, SuCutInOut::CutIn);
    }
    #[test]
    fn test_targets_functions() {
        let mut targets = Targets {
            selected_target: TargetType::Front1,

            ..Default::default()
        };
        targets.ru[TargetType::Front1 as usize].target_detected = true;
        targets.ru[TargetType::Front1 as usize].id = 14;
        targets.ru[TargetType::Front1 as usize].ru_class = RuClass::CAR;

        assert_eq!(targets.get_id(TargetType::Front1), Some(14));
        targets.selected_target = TargetType::Front1;
        targets.ru[TargetType::Front1 as usize].target_detected = false;
        assert_eq!(targets.get_id(TargetType::Front1), None);
        // assert!(!target.target_detected);
    }

    #[test]
    fn test_from_road_user_pmo_extended() {
        let roaduser = roaduser_t {
            pmo_id: 24,
            ru_id: RoadUserId::ROAD_USER_ID_01,
            direction: RoadUserDirection::ROAD_USER_DIRECTION_MOVING,
            poslongi: RoadUserPosLongi::ROAD_USER_POS_LONGI_FRONT,
            poscorridor: RoadUserCorridor::ROAD_USER_CORRIDOR_EGO,
        };
        let road_user_tab = core::array::from_fn(|i| {
            if i == 0 {
                roaduser.clone()
            } else {
                roaduser_t::default()
            }
        });
        let road_user_coll = roadusercoll_t {
            nbroaduser: 1,
            nbroaduserfront: 1,
            roaduser: road_user_tab,
            ..Default::default()
        };

        let pmo_coll_extended = pmocoll_extended_t::default();
        let targets: Targets =
            (&road_user_coll, &pmo_coll_extended, &Calibration::default()).into();
        assert_eq!(targets.ru[0].class, TargetType::NoTarget);
    }
}
