//! Convert between ADAS-DB and Protobuf
use acc_interface::datatypes::AccAppObservableT;
use acc_interface::datatypes::DiscreteDistance;
use acc_interface::datatypes::DriverFeelStrategyLevel;
use acc_interface::datatypes::DriverGuide;
use acc_interface::datatypes::DriverGuideDisplay;
use acc_interface::datatypes::FollowTimeLevel;
use acc_interface::datatypes::InterfaceID;
use acc_interface::datatypes::LineCrossingRefPoints;
use acc_interface::datatypes::ManeuverTypeT;
use acc_interface::datatypes::OverrideType;
use acc_interface::datatypes::RoadObjectType;
use acc_interface::datatypes::SoundAlert;
use acc_interface::datatypes::SpeedControlOverSpeedInformation_t;
use acc_interface::datatypes::SpeedUnit;
use acc_interface::datatypes::TimeAlert;
use acc_interface::datatypes::TrajectoryDynamicsLevel;
use acc_interface::datatypes::{
    maneuverRequest_t, DrivingModeV2T, ManeuverControlAxis, ObjectiveType,
};
use acc_interface::datatypes::{
    ACCDisplay_t, ApplicationState, CollisionAlert, CruiseSettingState_t, DistanceSetting,
    DriverSelection, FailureType, FeatureID, IVIContextOption, Overriding, SpeedAvailability,
    SpeedDisplay_t, State_t,
};
use catalog_ampere_adas as oem;
use num_traits::ToPrimitive;
use oem::sdv_adas_common_object::TrajKinLimits_t;
use oem::sdv_adas_maneuver_request::CurveControlT;
use oem::sdv_adas_maneuver_request::LimitationProperties_t;
use oem::sdv_adas_maneuver_request::ManeuverRequest_t;
use oem::sdv_adas_maneuver_request::ManeuverSettings_t;
use protobuf::Enum;
use protobuf::{EnumOrUnknown, MessageField};

/// Convert [maneuverRequest_t] to protobuf
pub fn to_pb_maneuver(req: &maneuverRequest_t) -> ManeuverRequest_t {
    use oem::sdv_adas_common_object::DriverFeelStrategyLevel::*;
    use oem::sdv_adas_common_object::DrivingModeV2T::*;
    use oem::sdv_adas_common_object::LineCrossingRefPoints::*;
    use oem::sdv_adas_common_object::ManeuverControlAxis::*;
    use oem::sdv_adas_common_object::TrajectoryDynamicsLevel::*;
    use oem::sdv_adas_maneuver_request::FollowTimeLevel::*;
    use oem::sdv_adas_maneuver_request::ManeuverTypeT::*;
    use oem::sdv_adas_maneuver_request::ObjectiveType::*;
    use oem::sdv_adas_maneuver_request::RoadObjectType::*;
    use oem::sdv_adas_sensor_information::InterfaceID::*;
    let driving_mode = match req.drivingMode {
        DrivingModeV2T::DRIVING_MODE_V2_T_ECO => EnumOrUnknown::new(DRIVING_MODE_V2_T_ECO),
        DrivingModeV2T::DRIVING_MODE_V2_T_SPORT => EnumOrUnknown::new(DRIVING_MODE_V2_T_SPORT),
        DrivingModeV2T::DRIVING_MODE_V2_T_COMFORT => EnumOrUnknown::new(DRIVING_MODE_V2_T_COMFORT),
        DrivingModeV2T::DRIVING_MODE_V2_T_UNSPECIFIED => {
            EnumOrUnknown::new(DRIVING_MODE_V2_T_UNSPECIFIED)
        }
    };
    let control_axis = match req.controlAxis {
        ManeuverControlAxis::MCA_NONE => EnumOrUnknown::new(MCA_NONE),
        ManeuverControlAxis::MCA_LAT => EnumOrUnknown::new(MCA_LAT),
        ManeuverControlAxis::MCA_LONGI => EnumOrUnknown::new(MCA_LONGI),
        ManeuverControlAxis::MCA_LONGILAT => EnumOrUnknown::new(MCA_LONGILAT),
    };
    let trajectory_dynamics_level = match req.trajectoryDynamicsLevel {
        TrajectoryDynamicsLevel::TDL_LOW => EnumOrUnknown::new(TDL_LOW),
        TrajectoryDynamicsLevel::TDL_NORMAL => EnumOrUnknown::new(TDL_NORMAL),
        TrajectoryDynamicsLevel::TDL_HIGH => EnumOrUnknown::new(TDL_HIGH),
    };
    let driver_feel_strategy_level = match req.driverFeelStrategyLevel {
        DriverFeelStrategyLevel::DFSL_LEVEL_0 => EnumOrUnknown::new(DFSL_LEVEL_0),
        DriverFeelStrategyLevel::DFSL_LEVEL_1 => EnumOrUnknown::new(DFSL_LEVEL_1),
        DriverFeelStrategyLevel::DFSL_LEVEL_2 => EnumOrUnknown::new(DFSL_LEVEL_2),
        DriverFeelStrategyLevel::DFSL_LEVEL_3 => EnumOrUnknown::new(DFSL_LEVEL_3),
    };
    let maneuver_type = match req.maneuverType {
        ManeuverTypeT::MTT_LIMIT => EnumOrUnknown::new(MTT_LIMIT),
        ManeuverTypeT::MTT_COMFORT => EnumOrUnknown::new(MTT_COMFORT),
        ManeuverTypeT::MTT_FACTORY => EnumOrUnknown::new(MTT_FACTORY),
        ManeuverTypeT::MTT_EMERGENCY => EnumOrUnknown::new(MTT_EMERGENCY),
        ManeuverTypeT::MTT_UNSPECIFIED => EnumOrUnknown::new(MTT_UNSPECIFIED),
    };
    let maneuver_settings = req
        .maneuverSettings
        .iter()
        .take(req.nbManeuverSettings.into())
        .map(|v| {
            let follow_time = match v.followTime {
                FollowTimeLevel::FTL_UNDEFINED => EnumOrUnknown::new(FTL_UNDEFINED),
                FollowTimeLevel::FTL_SHORT => EnumOrUnknown::new(FTL_SHORT),
                FollowTimeLevel::FTL_MIDDLE => EnumOrUnknown::new(FTL_MIDDLE),
                FollowTimeLevel::FTL_REGLEMENTARY => EnumOrUnknown::new(FTL_REGLEMENTARY),
                FollowTimeLevel::FTL_LONG => EnumOrUnknown::new(FTL_LONG),
                FollowTimeLevel::FTL_CUSTOMIZED => EnumOrUnknown::new(FTL_CUSTOMIZED),
            };
            let obj_of_interest_type = match v.objOfInterestType {
                InterfaceID::IID_POTENTIALLY_MOVING_OBJECTS => {
                    EnumOrUnknown::new(IID_POTENTIALLY_MOVING_OBJECTS)
                }
                InterfaceID::IID_ROAD_OBJECTS => EnumOrUnknown::new(IID_ROAD_OBJECTS),
                InterfaceID::IID_STATIC_OBJECTS => EnumOrUnknown::new(IID_STATIC_OBJECTS),
                InterfaceID::IID_FREESPACE_AREA_OBJECTS => {
                    EnumOrUnknown::new(IID_FREESPACE_AREA_OBJECTS)
                }
                InterfaceID::IID_CAMERA_FEATURES => EnumOrUnknown::new(IID_CAMERA_FEATURES),
                InterfaceID::IID_ULTRASONIC_FEATURES => EnumOrUnknown::new(IID_ULTRASONIC_FEATURES),
                InterfaceID::IID_RADAR_DETECTIONS => EnumOrUnknown::new(IID_RADAR_DETECTIONS),
                InterfaceID::IID_LIDAR_DETECTIONS => EnumOrUnknown::new(IID_LIDAR_DETECTIONS),
                InterfaceID::IID_CAMERA_DETECTIONS => EnumOrUnknown::new(IID_CAMERA_DETECTIONS),
                InterfaceID::IID_ULTRASONIC_DETECTIONS => {
                    EnumOrUnknown::new(IID_ULTRASONIC_DETECTIONS)
                }
                InterfaceID::IID_SENSOR_PERFORMANCE => EnumOrUnknown::new(IID_SENSOR_PERFORMANCE),
                InterfaceID::IID_SENSOR_HEALTH_INFORMATION => {
                    EnumOrUnknown::new(IID_SENSOR_HEALTH_INFORMATION)
                }
                InterfaceID::IID_COMMON_SENSOR_INPUT => EnumOrUnknown::new(IID_COMMON_SENSOR_INPUT),
            };
            let road_object_type = match v.roadObjectType {
                RoadObjectType::ROAD_OBJECT_TYPE_UNDEFINED => {
                    EnumOrUnknown::new(ROAD_OBJECT_TYPE_UNDEFINED)
                }
                RoadObjectType::ROAD_OBJECT_TYPE_LINE => EnumOrUnknown::new(ROAD_OBJECT_TYPE_LINE),
                RoadObjectType::ROAD_OBJECT_TYPE_LANE => EnumOrUnknown::new(ROAD_OBJECT_TYPE_LANE),
            };
            let objective_type = match v.objectiveType {
                ObjectiveType::OBJECTIVE_TYPE_NONE => EnumOrUnknown::new(OBJECTIVE_TYPE_NONE),
                ObjectiveType::OBJECTIVE_TYPE_AVOID => EnumOrUnknown::new(OBJECTIVE_TYPE_AVOID),
                ObjectiveType::OBJECTIVE_TYPE_FOLLOW => EnumOrUnknown::new(OBJECTIVE_TYPE_FOLLOW),
                ObjectiveType::OBJECTIVE_TYPE_DIRECT_CONTROL => {
                    EnumOrUnknown::new(OBJECTIVE_TYPE_DIRECT_CONTROL)
                }
            };
            let ego_ref_point = match v.egoRefPoint {
                LineCrossingRefPoints::LCRP_FRONT_LEFT => EnumOrUnknown::new(LCRP_FRONT_LEFT),
                LineCrossingRefPoints::LCRP_FRONT_RIGHT => EnumOrUnknown::new(LCRP_FRONT_RIGHT),
                LineCrossingRefPoints::LCRP_REAR_LEFT => EnumOrUnknown::new(LCRP_REAR_LEFT),
                LineCrossingRefPoints::LCRP_REAR_RIGHT => EnumOrUnknown::new(LCRP_REAR_RIGHT),
                LineCrossingRefPoints::LCRP_MID_SIDE_MID_WIDTH => {
                    EnumOrUnknown::new(LCRP_MID_SIDE_MID_WIDTH)
                }
            };

            ManeuverSettings_t {
                timestamp_reference: v.timestampReference,
                follow_time,
                obj_of_interest_type,
                road_object_type,
                obj_of_interest_id: v.objOfInterestID,
                objective_type,
                hard_dist_longi: v.hardDistLongi,
                hard_dist_lat: v.hardDistLat,
                soft_dist_longi: v.softDistLongi,
                soft_dist_lat: v.softDistLat,
                ego_ref_point,
                adviced_speed: v.advicedSpeed,
                ..Default::default()
            }
        })
        .collect();
    ManeuverRequest_t {
        driving_mode,
        maneuver_with_brake: req.maneuverWithBrake,
        maneuver_type,
        control_axis,
        nb_maneuver_settings: req.nbManeuverSettings.into(),
        maneuver_settings,
        hard_kin_limits: MessageField::some(TrajKinLimits_t {
            jerk_longi_min_max: req.hardKinLimits.jerkLongiMinMax.into(),
            jerk_lat_min_max: req.hardKinLimits.jerkLatMinMax.into(),
            accel_longi_min_max: req.hardKinLimits.accelLongiMinMax.into(),
            accel_lat_min_max: req.hardKinLimits.accelLatMinMax.into(),
            speed_min_max: req.hardKinLimits.speedMinMax.into(),
            swa_min_max: req.hardKinLimits.swaMinMax.into(),
            swa_speed_min_max: req.hardKinLimits.swaSpeedMinMax.into(),
            ..Default::default()
        }),
        set_speed: req.setSpeed,
        maneuver_time_window: req.maneuverTimeWindow,
        stand_still_flag: req.standStillFlag,
        limitation_properties: MessageField::some(LimitationProperties_t {
            time_constant_tsl: req.limitationProperties.time_constant_tsl,
            ratio_cf: req.limitationProperties.ratio_cf,
            ..Default::default()
        }),
        trajectory_dynamics_level,
        driver_feel_strategy_level,
        curve_control_strategies: MessageField::some(CurveControlT {
            is_curve_control_yaw_rate_active: req
                .curveControlStrategies
                .isCurveControlYawRateActive,
            is_curve_control_steering_angle_active: req
                .curveControlStrategies
                .isCurveControlSteeringAngleActive,
            is_curve_control_cam_active: req.curveControlStrategies.isCurveControlCamActive,
            is_curve_control_nav_curve_active: req
                .curveControlStrategies
                .isCurveControlNavCurveActive,
            is_curve_control_rain_active: req.curveControlStrategies.isCurveControlRainActive,
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// Convert [State_t] to protobuf
pub fn to_pb_state(st: &State_t) -> oem::sdv_adas_hmi_manager_types::State_t {
    use oem::sdv_adas_hmi_manager_types::ApplicationState::*;
    use oem::sdv_adas_hmi_manager_types::FailureType::*;
    use oem::sdv_adas_hmi_manager_types::FeatureID::*;
    let feat_id = match st.FeatID {
        FeatureID::FID_ACC => EnumOrUnknown::new(FID_ACC),
        _ => unreachable!("{:?}", st.FeatID),
    };
    let state = match st.State {
        ApplicationState::AS_OFF => EnumOrUnknown::new(AS_OFF),
        ApplicationState::AS_FAILSAFE => EnumOrUnknown::new(AS_FAILSAFE),
        ApplicationState::AS_SUSPENDED => EnumOrUnknown::new(AS_SUSPENDED),
        ApplicationState::AS_INITIALIZATION => EnumOrUnknown::new(AS_INITIALIZATION),
        ApplicationState::AS_OPERATIONAL => EnumOrUnknown::new(AS_OPERATIONAL),
    };
    let type_ = match st.Type {
        FailureType::FT_NO_FAILURE => EnumOrUnknown::new(FT_NO_FAILURE),
        FailureType::FT_SILENT_INHIBITION => EnumOrUnknown::new(FT_SILENT_INHIBITION),
        FailureType::FT_DEGRADED_MODE1 => EnumOrUnknown::new(FT_DEGRADED_MODE1),
        FailureType::FT_DEGRADED_MODE2 => EnumOrUnknown::new(FT_DEGRADED_MODE2),
        FailureType::FT_TEMPORARY_FAILURE => EnumOrUnknown::new(FT_TEMPORARY_FAILURE),
        FailureType::FT_PERMANENT_FAILURE => EnumOrUnknown::new(FT_PERMANENT_FAILURE),
    };
    oem::sdv_adas_hmi_manager_types::State_t {
        feat_id,
        state,
        type_,
        ..Default::default()
    }
}

/// Convert [ACCDisplay_t] to protobuf
pub fn to_pb_acc_display(display: &ACCDisplay_t) -> oem::sdv_adas_hmi_manager_types::ACCDisplay_t {
    use oem::sdv_adas_hmi_manager_types::CollisionAlert::*;
    use oem::sdv_adas_hmi_manager_types::FeatureID::*;
    use oem::sdv_adas_hmi_manager_types::Overriding::*;
    let feat_id = match display.FeatID {
        FeatureID::FID_ACC => EnumOrUnknown::new(FID_ACC),
        _ => unreachable!("{:?}", display.FeatID),
    };
    let alert = match display.Alert {
        CollisionAlert::CA_NO_ALERT => EnumOrUnknown::new(CA_NO_ALERT),
        CollisionAlert::CA_VEHICLE_APPROACH_ALERT => EnumOrUnknown::new(CA_VEHICLE_APPROACH_ALERT),
        CollisionAlert::CA_VEHICLE_ALERT_REMINDER => EnumOrUnknown::new(CA_VEHICLE_ALERT_REMINDER),
        CollisionAlert::CA_VEHICLE_COLLISION_ALERT => {
            EnumOrUnknown::new(CA_VEHICLE_COLLISION_ALERT)
        }
    };
    let speed_override = match display.SpeedOverride {
        Overriding::OVERRIDING_NO_OVERRIDE => EnumOrUnknown::new(OVERRIDING_NO_OVERRIDE),
        Overriding::OVERRIDING_SPEED_OVERRIDE => EnumOrUnknown::new(OVERRIDING_SPEED_OVERRIDE),
    };
    let distance_override = match display.DistanceOverride {
        Overriding::OVERRIDING_NO_OVERRIDE => EnumOrUnknown::new(OVERRIDING_NO_OVERRIDE),
        Overriding::OVERRIDING_SPEED_OVERRIDE => EnumOrUnknown::new(OVERRIDING_SPEED_OVERRIDE),
    };
    oem::sdv_adas_hmi_manager_types::ACCDisplay_t {
        feat_id,
        alert,
        speed_override,
        distance_override,
        ..Default::default()
    }
}

/// Convert [CruiseSettingState_t] to protobuf
pub fn to_pb_cruise_setting_state(
    setting: &CruiseSettingState_t,
) -> oem::sdv_adas_hmi_manager_types::CruiseSettingState_t {
    use oem::sdv_adas_hmi_manager_types::DistanceSetting::*;
    use oem::sdv_adas_hmi_manager_types::DriverSelection::*;
    use oem::sdv_adas_hmi_manager_types::FeatureID::*;
    use oem::sdv_adas_hmi_manager_types::IVIContextOption::*;
    let feat_id = match setting.FeatID {
        FeatureID::FID_ACC => EnumOrUnknown::new(FID_ACC),
        _ => unreachable!("{:?}", setting.FeatID),
    };
    let context = match setting.Context {
        IVIContextOption::IVICO_NO_CONTEXT => EnumOrUnknown::new(IVICO_NO_CONTEXT),
        IVIContextOption::IVICO_CONTEXT_ROAD => EnumOrUnknown::new(IVICO_CONTEXT_ROAD),
        IVIContextOption::IVICO_CONTEXT_SPEED => EnumOrUnknown::new(IVICO_CONTEXT_SPEED),
    };
    let da = match setting.AccIntelligentDistanceActivation {
        DriverSelection::DS_UNSPECIFIED => EnumOrUnknown::new(DS_UNSPECIFIED),
        DriverSelection::DS_ACTIVATION => EnumOrUnknown::new(DS_ACTIVATION),
        DriverSelection::DS_DEACTIVATION => EnumOrUnknown::new(DS_DEACTIVATION),
    };
    let ds = match setting.SetTimeOfCollision {
        DistanceSetting::DS_SHORT => EnumOrUnknown::new(DS_SHORT),
        DistanceSetting::DS_MIDDLE => EnumOrUnknown::new(DS_MIDDLE),
        DistanceSetting::DS_LONG => EnumOrUnknown::new(DS_LONG),
        DistanceSetting::DS_REGLEMENTARY => EnumOrUnknown::new(DS_REGLEMENTARY),
        DistanceSetting::DS_NO_DISPLAY => EnumOrUnknown::new(DS_NO_DISPLAY),
    };
    oem::sdv_adas_hmi_manager_types::CruiseSettingState_t {
        timestamp: setting.timestamp,
        feat_id,
        context,
        offset: setting.Offset.into(),
        acc_intelligent_distance_activation: da,
        set_time_of_collision: ds,
        ..Default::default()
    }
}

/// Convert [SpeedControlOverSpeedInformation_t] to protobuf
pub fn to_pb_speed_control_over_speed_information(
    value: &SpeedControlOverSpeedInformation_t,
) -> oem::sdv_adas_hmi_manager_types::SpeedControlOverSpeedInformation_t {
    use oem::sdv_adas_hmi_manager_types::FeatureID::*;
    use oem::sdv_adas_hmi_manager_types::OverrideType::*;
    use oem::sdv_adas_hmi_manager_types::SoundAlert::*;

    let feat_id = match value.FeatID {
        FeatureID::FID_ACC => EnumOrUnknown::new(FID_ACC),
        _ => unreachable!("{:?}", value.FeatID),
    };
    let speed_control_override = match value.SpeedControlOverride {
        OverrideType::OT_NO_OVERRIDE => EnumOrUnknown::new(OT_NO_OVERRIDE),
        OverrideType::OT_DISTANCE_OVERRIDE => EnumOrUnknown::new(OT_DISTANCE_OVERRIDE),
        OverrideType::OT_SPEED_VOLUNTARY_OVERRIDE => {
            EnumOrUnknown::new(OT_SPEED_VOLUNTARY_OVERRIDE)
        }
        OverrideType::OT_SPEED_INVOLUNTARY_OVERRIDE => {
            EnumOrUnknown::new(OT_SPEED_INVOLUNTARY_OVERRIDE)
        }
    };
    let over_speed_sound_alert = match value.OverSpeedSoundAlert {
        SoundAlert::SA_NO_ALERT => EnumOrUnknown::new(SA_NO_ALERT),
        SoundAlert::SA_ALERT => EnumOrUnknown::new(SA_ALERT),
    };
    oem::sdv_adas_hmi_manager_types::SpeedControlOverSpeedInformation_t {
        feat_id,
        speed_control_override,
        over_speed_sound_alert,
        ..Default::default()
    }
}

/// Convert [SpeedUnit] to protobuf
pub fn to_pb_driver_guide(dg: &DriverGuide) -> oem::sdv_adas_hmi_manager_types::DriverGuide {
    use oem::sdv_adas_hmi_manager_types::DriverGuideDisplay::*;
    use oem::sdv_adas_hmi_manager_types::FeatureID::*;
    let feature_id = match dg.feature_id {
        FeatureID::FID_ACC => EnumOrUnknown::new(FID_ACC),
        _ => unreachable!("{:?}", dg.feature_id),
    };

    let driver_guide_display = match dg.driver_guide_display {
        DriverGuideDisplay::DGD_NO_DISPLAY => DGD_NO_DISPLAY,
        DriverGuideDisplay::DGD_ACC_REG_READY_TO_GO => DGD_ACC_REG_READY_TO_GO,
        DriverGuideDisplay::DGD_ACC_ACT_TO_START_REG => DGD_ACC_ACT_TO_START_REG,
        DriverGuideDisplay::DGD_PARK_ASSIST_ACTIVE => DGD_PARK_ASSIST_ACTIVE,
        DriverGuideDisplay::DGD_PARKING_BRAKE_ACTIVATED => DGD_PARKING_BRAKE_ACTIVATED,
        DriverGuideDisplay::DGD_HDC_ACTIVATED => DGD_HDC_ACTIVATED,
        DriverGuideDisplay::DGD_VDC_DEACTIVATED => DGD_VDC_DEACTIVATED,
        DriverGuideDisplay::DGD_VDC_IN_REGULATION => DGD_VDC_IN_REGULATION,
        DriverGuideDisplay::DGD_OUT_OF_SPEED_RANGE => DGD_OUT_OF_SPEED_RANGE,
        DriverGuideDisplay::DGD_EVC_PRIORITY_MODE => DGD_EVC_PRIORITY_MODE,
        DriverGuideDisplay::DGD_ECO_PRIORITY_MODE => DGD_ECO_PRIORITY_MODE,
        DriverGuideDisplay::DGD_DISCONNECTION_LEVEL1 => DGD_DISCONNECTION_LEVEL1,
        DriverGuideDisplay::DGD_DISCONNECTION_LEVEL2 => DGD_DISCONNECTION_LEVEL2,
        DriverGuideDisplay::DGD_DEACTIVATION_LEVEL1 => DGD_DEACTIVATION_LEVEL1,
        DriverGuideDisplay::DGD_DEACTIVATION_LEVEL2 => DGD_DEACTIVATION_LEVEL2,
        DriverGuideDisplay::DGD_RACE_MODE => DGD_RACE_MODE,
        DriverGuideDisplay::DGD_LCA_ACTIVE => DGD_LCA_ACTIVE,
        DriverGuideDisplay::DGD_LKA_ACTIVE => DGD_LKA_ACTIVE,
        DriverGuideDisplay::DGD_PRESS_SET_FIRST => DGD_PRESS_SET_FIRST,
        DriverGuideDisplay::DGD_INVALID_SPEED_ECO => DGD_INVALID_SPEED_ECO,
    }
    .into();
    oem::sdv_adas_hmi_manager_types::DriverGuide {
        feature_id,
        driver_guide_display,
        ..Default::default()
    }
}

/// Convert [TimeAlert] to protobuf
pub fn to_pb_time_alert(alert: TimeAlert) -> oem::sdv_adas_hmi_manager_types::TimeAlert {
    use oem::sdv_adas_hmi_manager_types::TimeAlert::*;
    match alert {
        TimeAlert::TA_ALERT => TA_ALERT,
        TimeAlert::TA_NO_ALERT => TA_NO_ALERT,
    }
}

/// Convert [DiscreteDistance] to protobuf
pub fn to_pb_discrete_dist(
    distance: DiscreteDistance,
) -> oem::sdv_adas_hmi_manager_types::DiscreteDistance {
    use oem::sdv_adas_hmi_manager_types::DiscreteDistance::*;
    match distance {
        DiscreteDistance::DD_UNSPECIFIED => DD_UNSPECIFIED,
        DiscreteDistance::DD_NO_TARGET => DD_NO_TARGET,
        DiscreteDistance::DD_TARGET_DISTANCE1 => DD_TARGET_DISTANCE1,
        DiscreteDistance::DD_TARGET_DISTANCE2 => DD_TARGET_DISTANCE2,
        DiscreteDistance::DD_TARGET_DISTANCE3 => DD_TARGET_DISTANCE3,
        DiscreteDistance::DD_TARGET_DISTANCE4 => DD_TARGET_DISTANCE4,
    }
}

/// Convert [SpeedUnit] to protobuf
pub fn to_pb_speed_unit(unit: SpeedUnit) -> oem::sdv_common_types::SpeedUnit {
    use oem::sdv_common_types::SpeedUnit::*;
    match unit {
        SpeedUnit::SPEED_UNIT_UNKNOWN => SPEED_UNIT_UNKNOWN,
        SpeedUnit::SPEED_UNIT_MILES_PER_HOUR => SPEED_UNIT_MILES_PER_HOUR,
        SpeedUnit::SPEED_UNIT_KILOMETER_PER_HOUR => SPEED_UNIT_KILOMETER_PER_HOUR,
    }
}

/// Convert [SpeedDisplay_t] to protobuf
pub fn to_pb_speed_display(
    display: &SpeedDisplay_t,
) -> oem::sdv_adas_hmi_manager_types::SpeedDisplay_t {
    use oem::sdv_adas_hmi_manager_types::SpeedAvailability::*;
    let speed_display_state = match display.SpeedDisplayState {
        SpeedAvailability::SA_AVAILABLE => SA_AVAILABLE,
        SpeedAvailability::SA_UNAVAILABLE => SA_UNAVAILABLE,
    }
    .into();
    oem::sdv_adas_hmi_manager_types::SpeedDisplay_t {
        speed_display_state,
        speed_display_kph: display.SpeedDisplay_kph,
        speed_display_mph: display.SpeedDisplay_mph,
        speed_display_mps: display.SpeedDisplay_mps,
        ..Default::default()
    }
}

/// Convert [AccAppObservableT] to protobuf
pub fn to_pb_observable(obs: &AccAppObservableT) -> oem::sdv_adas_debug_topics::AccAppObservable {
    oem::sdv_adas_debug_topics::AccAppObservable {
        obs_acc_app: Some(oem::sdv_adas_debug_acc_app_types::AccAppObservableT {
            acc_status: oem::sdv_adas_debug_acc_app_types::AccStatus::from_i32(
                obs.acc_status.to_i32().unwrap_or_default(),
            )
            .unwrap_or_default()
            .into(),
            acc_abwb: oem::sdv_adas_debug_acc_app_types::AccAbwbState::from_i32(
                obs.acc_abwb.to_i32().unwrap_or_default(),
            )
            .unwrap_or_default()
            .into(),
            abwb_left_low_decel_confirmed: obs.abwb_left_low_decel_confirmed,
            abwb_right_low_decel_confirmed: obs.abwb_right_low_decel_confirmed,
            abwb_left_maneuver_aborted: obs.abwb_left_maneuver_aborted,
            abwb_right_maneuver_aborted: obs.abwb_right_maneuver_aborted,
            abwb_left_cancelled: obs.abwb_left_cancelled,
            abwb_right_cancelled: obs.abwb_right_cancelled,
            acc_antiundertake: oem::sdv_adas_debug_acc_app_types::AccAntiUndertakeState::from_i32(
                obs.acc_antiundertake.to_i32().unwrap_or_default(),
            )
            .unwrap_or_default()
            .into(),
            antiundertake_line_confirmed: obs.antiundertake_line_confirmed,
            antiundertake_low_decel_confirmed: obs.antiundertake_low_decel_confirmed,
            acc_autorestart_status:
                oem::sdv_adas_debug_acc_app_types::AccAutoRestartState::from_i32(
                    obs.acc_autorestart_status.to_i32().unwrap_or_default(),
                )
                .unwrap_or_default()
                .into(),
            acc_autorestart_zone1: obs.acc_autorestart_zone1,
            acc_autorestart_zone2: obs.acc_autorestart_zone2,
            acc_overtaking_support:
                oem::sdv_adas_debug_acc_app_types::AccOvertakingState::from_i32(
                    obs.acc_overtaking_support.to_i32().unwrap_or_default(),
                )
                .unwrap_or_default()
                .into(),
            acc_apply_maneuver_id: obs.acc_apply_maneuver_id.into(),
            acc_cancel_maneuver_id: obs.acc_cancel_maneuver_id.into(),
            acc_cancel_cutoff_duration: obs.acc_cancel_cutoff_duration,
            acc_cut_in_left: oem::sdv_adas_debug_acc_app_types::AccCutIn::from_i32(
                obs.acc_cut_in_left.to_i32().unwrap_or_default(),
            )
            .unwrap_or_default()
            .into(),
            acc_cut_in_right: oem::sdv_adas_debug_acc_app_types::AccCutIn::from_i32(
                obs.acc_cut_in_right.to_i32().unwrap_or_default(),
            )
            .unwrap_or_default()
            .into(),
            acc_hmi_setting_time_of_collision:
                oem::sdv_adas_hmi_manager_types::DistanceSetting::from_i32(
                    obs.acc_hmi_setting_time_of_collision
                        .to_i32()
                        .unwrap_or_default(),
                )
                .unwrap_or_default()
                .into(),
            acc_hmi_setting_intelligent_dist_activation:
                oem::sdv_adas_hmi_manager_types::DriverSelection::from_i32(
                    obs.acc_hmi_setting_intelligent_dist_activation
                        .to_i32()
                        .unwrap_or_default(),
                )
                .unwrap_or_default()
                .into(),
            acc_hmi_setting_set_veh_speed: obs.acc_hmi_setting_set_veh_speed,
            acc_hmi_alerts_display_alert:
                oem::sdv_adas_hmi_manager_types::CollisionAlert::from_i32(
                    obs.acc_hmi_alerts_display_alert
                        .to_i32()
                        .unwrap_or_default(),
                )
                .unwrap_or_default()
                .into(),
            acc_hmi_alerts_display_dist_override:
                oem::sdv_adas_hmi_manager_types::Overriding::from_i32(
                    obs.acc_hmi_alerts_display_dist_override
                        .to_i32()
                        .unwrap_or_default(),
                )
                .unwrap_or_default()
                .into(),
            acc_hmi_alerts_sp_ctrl_override:
                oem::sdv_adas_hmi_manager_types::OverrideType::from_i32(
                    obs.acc_hmi_alerts_sp_ctrl_override
                        .to_i32()
                        .unwrap_or_default(),
                )
                .unwrap_or_default()
                .into(),
            acc_hmi_alerts_vehicle_detected_distance:
                oem::sdv_adas_hmi_manager_types::DiscreteDistance::from_i32(
                    obs.acc_hmi_alerts_vehicle_detected_distance
                        .to_i32()
                        .unwrap_or_default(),
                )
                .unwrap_or_default()
                .into(),
            acc_hmi_alerts_driver_guide: Some(to_pb_driver_guide(&obs.acc_hmi_alerts_driver_guide))
                .into(),
            acc_hmi_state: Some(to_pb_state(&obs.acc_hmi_state)).into(),
            acc_deactivation_cause:
                oem::sdv_adas_debug_acc_app_types::AccDeactivationCause::from_i32(
                    obs.acc_deactivation_cause.to_i32().unwrap_or_default(),
                )
                .unwrap_or_default()
                .into(),
            ..Default::default()
        })
        .into(),
        ..Default::default()
    }
}
