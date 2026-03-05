//! Define ego Stop Status for ACC
//!
use acc_interface::datatypes::{
    stop_t, ParkingBrakeBindingStatusEgo, ParkingBrakeRequestInProgressEgo, ParkingBrakeStatusEgo,
    RolloverDetectionEgo, StandStillStatusEgo,
};
#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
/// Stand Still Status
pub enum StandStillStatus {
    /// Stand Still Not hold
    #[default]
    NotHold,
    /// Stand Still Hold
    Hold,
    /// Stand Still Hold Not possible
    HoldNotPossible,
}

/// Parking Brake Request In Progress Status
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum ParkingBrakeReqStatus {
    #[default]
    /// No parking brake request
    NoBrakeRequest,
    /// Park Brake tightened
    Tightening,
    /// Park brake Released
    Release,
}

/// Parking Brake Status
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum ParkingBrakeStatus {
    #[default]
    /// No parking brake Not Applied
    NotApplied,
    /// Park Brake Applied
    Applied,
    /// Park brake status Unknow
    Unknown,
}

#[derive(Debug, Clone, Default)]
/// Ego Stop Status
pub struct Stop {
    /// Global standstill status
    pub global_standstill_status: StandStillStatus,
    /// prev global standstill status
    pub prev_global_standstill_status: StandStillStatus,
    /// Park brake req
    pub parking_brake_req: ParkingBrakeReqStatus,
    /// Parking Brake Status
    pub parking_brake_status: ParkingBrakeStatus,
    /// Parking Brake Binding Status
    pub parking_brake_binding_status: ParkingBrakeStatus,
    /// Roll Over Detection
    pub is_roll_over_detected: bool,
    /// Min wheel torque for release
    pub min_wheel_torque_for_release: f32,
}

impl From<&stop_t> for Stop {
    fn from(value: &stop_t) -> Self {
        Self {
            global_standstill_status: value.globalStandstillStatus.into(),
            prev_global_standstill_status: value.globalStandstillStatus.into(),
            parking_brake_req: value.parkingBrakeRequestInProgress.into(),
            parking_brake_binding_status: value.parkingBrakeBindingStatus.into(),
            parking_brake_status: value.parkingBrakeStatus.into(),
            is_roll_over_detected: matches!(
                value.rollOverDetection,
                RolloverDetectionEgo::RDE_DETECTED
            ),
            min_wheel_torque_for_release: value.minWhlTqForRelease,
        }
    }
}

#[cfg(feature = "caros")]
impl From<&oem::sdv_adas_ego_ego_braking::Stop_t> for Stop {
    fn from(value: &oem::sdv_adas_ego_ego_braking::Stop_t) -> Self {
        use oem::sdv_adas_ego_ego_braking::RolloverDetectionEgo;
        Self {
            global_standstill_status: value
                .global_standstill_status
                .enum_value()
                .unwrap_or_default()
                .into(),
            prev_global_standstill_status: value
                .global_standstill_status
                .enum_value()
                .unwrap_or_default()
                .into(),
            parking_brake_req: value
                .parking_brake_request_in_progress
                .enum_value()
                .unwrap_or_default()
                .into(),
            parking_brake_binding_status: value
                .parking_brake_binding_status
                .enum_value()
                .unwrap_or_default()
                .into(),
            parking_brake_status: value
                .parking_brake_status
                .enum_value()
                .unwrap_or_default()
                .into(),
            is_roll_over_detected: matches!(
                value.roll_over_detection.enum_value(),
                Ok(RolloverDetectionEgo::RDE_DETECTED)
            ),
            min_wheel_torque_for_release: value.min_whl_tq_for_release,
        }
    }
}

impl From<ParkingBrakeBindingStatusEgo> for ParkingBrakeStatus {
    fn from(value: ParkingBrakeBindingStatusEgo) -> Self {
        match value {
            ParkingBrakeBindingStatusEgo::PBBSE_BRAKE_RELEASED => Self::NotApplied,
            ParkingBrakeBindingStatusEgo::PBBSE_BRAKE_TIGHTENED => Self::Applied,
            ParkingBrakeBindingStatusEgo::PBBSE_NOT_AVAILABLE => Self::Unknown,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_braking::ParkingBrakeBindingStatusEgo> for ParkingBrakeStatus {
    fn from(value: oem::sdv_adas_ego_ego_braking::ParkingBrakeBindingStatusEgo) -> Self {
        use oem::sdv_adas_ego_ego_braking::ParkingBrakeBindingStatusEgo;
        match value {
            ParkingBrakeBindingStatusEgo::PBBSE_BRAKE_RELEASED => Self::NotApplied,
            ParkingBrakeBindingStatusEgo::PBBSE_BRAKE_TIGHTENED => Self::Applied,
            ParkingBrakeBindingStatusEgo::PBBSE_NOT_AVAILABLE => Self::Unknown,
        }
    }
}

impl From<ParkingBrakeStatusEgo> for ParkingBrakeStatus {
    fn from(value: ParkingBrakeStatusEgo) -> Self {
        match value {
            ParkingBrakeStatusEgo::PBSE_AUTOMATIC_OR_MANUAL_PARKING_BRAKE_APPLIED => Self::Applied,
            ParkingBrakeStatusEgo::PBSE_PARKING_BRAKE_STATUS_NOT_AVAILABLE => Self::Unknown,
            ParkingBrakeStatusEgo::PBSE_AUTOMATIC_OR_MANUAL_PARKING_BRAKE_NOT_APPLIED => {
                Self::NotApplied
            }
            ParkingBrakeStatusEgo::PBSE_PARKING_BRAKE_UNSPECIFIED => Self::Unknown,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_braking::ParkingBrakeStatusEgo> for ParkingBrakeStatus {
    fn from(value: oem::sdv_adas_ego_ego_braking::ParkingBrakeStatusEgo) -> Self {
        use oem::sdv_adas_ego_ego_braking::ParkingBrakeStatusEgo;
        match value {
            ParkingBrakeStatusEgo::PBSE_AUTOMATIC_OR_MANUAL_PARKING_BRAKE_APPLIED => Self::Applied,
            ParkingBrakeStatusEgo::PBSE_PARKING_BRAKE_STATUS_NOT_AVAILABLE => Self::Unknown,
            ParkingBrakeStatusEgo::PBSE_AUTOMATIC_OR_MANUAL_PARKING_BRAKE_NOT_APPLIED => {
                Self::NotApplied
            }
            ParkingBrakeStatusEgo::PBSE_PARKING_BRAKE_UNSPECIFIED => Self::Unknown,
        }
    }
}

impl From<StandStillStatusEgo> for StandStillStatus {
    fn from(value: StandStillStatusEgo) -> Self {
        match value {
            StandStillStatusEgo::SSSE_VEHICLE_NOT_HOLD => Self::NotHold,
            StandStillStatusEgo::SSSE_VEHICLE_HOLD => Self::Hold,
            StandStillStatusEgo::SSSE_VEHICLE_HOLD_NOT_POSSIBLE => Self::HoldNotPossible,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_braking::StandStillStatusEgo> for StandStillStatus {
    fn from(value: oem::sdv_adas_ego_ego_braking::StandStillStatusEgo) -> Self {
        use oem::sdv_adas_ego_ego_braking::StandStillStatusEgo;
        match value {
            StandStillStatusEgo::SSSE_VEHICLE_NOT_HOLD => Self::NotHold,
            StandStillStatusEgo::SSSE_VEHICLE_HOLD => Self::Hold,
            StandStillStatusEgo::SSSE_VEHICLE_HOLD_NOT_POSSIBLE => Self::HoldNotPossible,
        }
    }
}

impl From<ParkingBrakeRequestInProgressEgo> for ParkingBrakeReqStatus {
    fn from(value: ParkingBrakeRequestInProgressEgo) -> Self {
        match value {
            ParkingBrakeRequestInProgressEgo::PBRIPE_BRAKE_RELEASE_REQUEST => Self::Release,
            ParkingBrakeRequestInProgressEgo::PBRIPE_NO_PARKING_BRAKE_REQUEST => {
                Self::NoBrakeRequest
            }
            ParkingBrakeRequestInProgressEgo::PBRIPE_BRAKE_TIGHTENING_REQUEST => Self::Tightening,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_braking::ParkingBrakeRequestInProgressEgo>
    for ParkingBrakeReqStatus
{
    fn from(value: oem::sdv_adas_ego_ego_braking::ParkingBrakeRequestInProgressEgo) -> Self {
        use oem::sdv_adas_ego_ego_braking::ParkingBrakeRequestInProgressEgo;
        match value {
            ParkingBrakeRequestInProgressEgo::PBRIPE_BRAKE_RELEASE_REQUEST => Self::Release,
            ParkingBrakeRequestInProgressEgo::PBRIPE_NO_PARKING_BRAKE_REQUEST => {
                Self::NoBrakeRequest
            }
            ParkingBrakeRequestInProgressEgo::PBRIPE_BRAKE_TIGHTENING_REQUEST => Self::Tightening,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_conversion() {
        let stop_t = stop_t {
            globalStandstillStatus: StandStillStatusEgo::SSSE_VEHICLE_HOLD,
            parkingBrakeRequestInProgress:
                ParkingBrakeRequestInProgressEgo::PBRIPE_BRAKE_TIGHTENING_REQUEST,
            ..Default::default()
        };

        let stop: Stop = (&stop_t).into();

        assert_eq!(stop.global_standstill_status, StandStillStatus::Hold);
        assert_eq!(stop.parking_brake_req, ParkingBrakeReqStatus::Tightening);
    }

    #[test]
    fn test_standstill_status_conversion() {
        assert_eq!(
            StandStillStatus::from(StandStillStatusEgo::SSSE_VEHICLE_NOT_HOLD),
            StandStillStatus::NotHold
        );
        assert_eq!(
            StandStillStatus::from(StandStillStatusEgo::SSSE_VEHICLE_HOLD),
            StandStillStatus::Hold
        );
        assert_eq!(
            StandStillStatus::from(StandStillStatusEgo::SSSE_VEHICLE_HOLD_NOT_POSSIBLE),
            StandStillStatus::HoldNotPossible
        );
    }

    #[test]
    fn test_parking_brake_req_status_conversion() {
        assert_eq!(
            ParkingBrakeReqStatus::from(
                ParkingBrakeRequestInProgressEgo::PBRIPE_BRAKE_RELEASE_REQUEST
            ),
            ParkingBrakeReqStatus::Release
        );
        assert_eq!(
            ParkingBrakeReqStatus::from(
                ParkingBrakeRequestInProgressEgo::PBRIPE_NO_PARKING_BRAKE_REQUEST
            ),
            ParkingBrakeReqStatus::NoBrakeRequest
        );
        assert_eq!(
            ParkingBrakeReqStatus::from(
                ParkingBrakeRequestInProgressEgo::PBRIPE_BRAKE_TIGHTENING_REQUEST
            ),
            ParkingBrakeReqStatus::Tightening
        );
    }
    #[test]
    fn test_from_parking_brake_binding_status_ego() {
        let mut parking_brake_binding_status_ego =
            ParkingBrakeBindingStatusEgo::PBBSE_BRAKE_RELEASED;
        let parking_brake_status: ParkingBrakeStatus = parking_brake_binding_status_ego.into();
        assert_eq!(parking_brake_status, ParkingBrakeStatus::NotApplied);
        parking_brake_binding_status_ego = ParkingBrakeBindingStatusEgo::PBBSE_BRAKE_TIGHTENED;
        let parking_brake_status: ParkingBrakeStatus = parking_brake_binding_status_ego.into();
        assert_eq!(parking_brake_status, ParkingBrakeStatus::Applied);
        parking_brake_binding_status_ego = ParkingBrakeBindingStatusEgo::PBBSE_NOT_AVAILABLE;
        let parking_brake_status: ParkingBrakeStatus = parking_brake_binding_status_ego.into();
        assert_eq!(parking_brake_status, ParkingBrakeStatus::Unknown);
    }
    #[test]
    fn from_parking_brake_status_ego() {
        let mut parking_brake_status_ego =
            ParkingBrakeStatusEgo::PBSE_AUTOMATIC_OR_MANUAL_PARKING_BRAKE_APPLIED;
        let parking_brake_status: ParkingBrakeStatus = parking_brake_status_ego.into();
        assert_eq!(parking_brake_status, ParkingBrakeStatus::Applied);
        parking_brake_status_ego = ParkingBrakeStatusEgo::PBSE_PARKING_BRAKE_STATUS_NOT_AVAILABLE;
        let parking_brake_status: ParkingBrakeStatus = parking_brake_status_ego.into();
        assert_eq!(parking_brake_status, ParkingBrakeStatus::Unknown);
        parking_brake_status_ego =
            ParkingBrakeStatusEgo::PBSE_AUTOMATIC_OR_MANUAL_PARKING_BRAKE_NOT_APPLIED;
        let parking_brake_status: ParkingBrakeStatus = parking_brake_status_ego.into();
        assert_eq!(parking_brake_status, ParkingBrakeStatus::NotApplied);
    }
}
