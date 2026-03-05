//! Define ego global trajectory status for ACC
//!
use crate::interp::{InterpResult, LookUp1DTable};
use acc_interface::{
    datatypes::{
        AdasBrakeWheelTorqueOrderT, AdasEmergencyBrakeDecelOrderT, AdasStandStillRequestV2T,
        TrajectoryGlobalStatusV2T,
    },
    Calibration,
};
#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;

///  Standstill request
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum StandStillReq {
    #[default]
    /// Standstill hold or launch not requested
    FunctionOff,
    /// Standstill Hold requested
    HoldRequested,
    /// Standstill Launch requested
    LaunchRequested,
}

///  AEB request
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum AebRequested {
    #[default]
    /// aeb braking not requested
    AebNotRequested,
    /// aeb braking requested (moderate/maximum)
    AebRequested,
}

/// Define TrajectoryGlobalStatus for ACC
#[derive(Debug, Default, Clone)]
pub struct TrajectoryGlobalStatus {
    /// Powertrain wheel torque requested by traj control
    pub pwt_wheel_torque: f32,
    /// Powertrain wheel torque order
    pub pwt_wheel_torque_order: bool,
    /// Brake wheel torque requested by traj control
    pub brake_wheel_torque: f32,
    /// Brake wheel torque order
    pub brake_wheel_torque_order: bool,
    /// Standstill request from trajectory control
    pub standstill_req: StandStillReq,
    /// aeb ongoing
    pub aeb_requested: AebRequested,
}

impl TrajectoryGlobalStatus {
    /// true if aeb is requested by traj control
    pub fn is_aeb_requested(&self) -> bool {
        self.aeb_requested == AebRequested::AebRequested
    }
    /// calculate the cut off duration with current torque
    pub fn current_cut_off_duration(&self, calib: &Calibration) -> InterpResult<f32> {
        if !self.pwt_wheel_torque_order && !self.brake_wheel_torque_order {
            return Ok(0.0);
        }
        let (torque_lut, cutoff_lut, torque) = if self.pwt_wheel_torque_order {
            (
                &calib.P_tng_Nm_Acc_PWTTorqueCmd_LUT,
                &calib.P_tng_s_Acc_CutoffOnPWTTorqueCmd_LUT,
                self.pwt_wheel_torque,
            )
        } else {
            (
                &calib.P_tng_Nm_Acc_BrakeTorqueReq_LUT,
                &calib.P_tng_s_Acc_CutoffOnBrakeTorqueReq_LUT,
                self.brake_wheel_torque,
            )
        };
        LookUp1DTable::new(torque_lut, cutoff_lut)?.interpolate(torque)
    }
}

impl From<AdasStandStillRequestV2T> for StandStillReq {
    fn from(value: AdasStandStillRequestV2T) -> Self {
        match value {
            AdasStandStillRequestV2T::ASSRV2T_FUNCTION_OFF => Self::FunctionOff,
            AdasStandStillRequestV2T::ASSRV2T_HOLD_REQUESTED => Self::HoldRequested,
            AdasStandStillRequestV2T::ASSRV2T_LAUNCH_REQUESTED => Self::LaunchRequested,
        }
    }
}

impl From<&AdasEmergencyBrakeDecelOrderT> for AebRequested {
    fn from(value: &AdasEmergencyBrakeDecelOrderT) -> Self {
        match value {
            AdasEmergencyBrakeDecelOrderT::AEBDOT_NOBRAKE_REQUEST => AebRequested::AebNotRequested,
            AdasEmergencyBrakeDecelOrderT::AEBDOT_DECEL_MODERATE => AebRequested::AebRequested,
            AdasEmergencyBrakeDecelOrderT::AEBDOT_DECEL_MAXIMUM => AebRequested::AebRequested,
            AdasEmergencyBrakeDecelOrderT::AEBDOT_UNAVAILABLE => AebRequested::AebNotRequested,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_trajectory_interfaces::AdasEmergencyBrakeDecelOrderT> for AebRequested {
    fn from(value: oem::sdv_adas_trajectory_interfaces::AdasEmergencyBrakeDecelOrderT) -> Self {
        use oem::sdv_adas_trajectory_interfaces::AdasEmergencyBrakeDecelOrderT;
        match value {
            AdasEmergencyBrakeDecelOrderT::AEBDOT_NOBRAKE_REQUEST => AebRequested::AebNotRequested,
            AdasEmergencyBrakeDecelOrderT::AEBDOT_DECEL_MODERATE => AebRequested::AebRequested,
            AdasEmergencyBrakeDecelOrderT::AEBDOT_DECEL_MAXIMUM => AebRequested::AebRequested,
            AdasEmergencyBrakeDecelOrderT::AEBDOT_UNAVAILABLE => AebRequested::AebNotRequested,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_trajectory_interfaces::AdasStandStillRequestV2T> for StandStillReq {
    fn from(value: oem::sdv_adas_trajectory_interfaces::AdasStandStillRequestV2T) -> Self {
        use oem::sdv_adas_trajectory_interfaces::AdasStandStillRequestV2T;
        match value {
            AdasStandStillRequestV2T::ASSRV2T_FUNCTION_OFF => Self::FunctionOff,
            AdasStandStillRequestV2T::ASSRV2T_HOLD_REQUESTED => Self::HoldRequested,
            AdasStandStillRequestV2T::ASSRV2T_LAUNCH_REQUESTED => Self::LaunchRequested,
        }
    }
}

impl From<&TrajectoryGlobalStatusV2T> for TrajectoryGlobalStatus {
    fn from(value: &TrajectoryGlobalStatusV2T) -> Self {
        Self {
            pwt_wheel_torque: value.pwtTorqueCommand.wheelTorqueRequest,
            pwt_wheel_torque_order: value.pwtTorqueCommand.wheelTorqueOrder,
            brake_wheel_torque: value.brakeWheelTorqueRequest.into(),
            brake_wheel_torque_order: value
                .brakeWheelTorqueOrder
                .eq(&AdasBrakeWheelTorqueOrderT::ABWTOT_BRAKE_ACC_AD),
            standstill_req: value.standStillRequest.into(),
            aeb_requested: (&value.emergencyBrakeDecelOrder).into(),
        }
    }
}

#[cfg(feature = "caros")]
impl From<&oem::sdv_adas_trajectory_interfaces::TrajectoryGlobalStatusV2T>
    for TrajectoryGlobalStatus
{
    fn from(value: &oem::sdv_adas_trajectory_interfaces::TrajectoryGlobalStatusV2T) -> Self {
        Self {
            pwt_wheel_torque: value
                .pwt_torque_command
                .as_ref()
                .map(|v| v.wheel_torque_request)
                .unwrap_or_default(),
            pwt_wheel_torque_order: value
                .pwt_torque_command
                .as_ref()
                .map(|v| v.wheel_torque_order)
                .unwrap_or_default(),
            brake_wheel_torque: value.brake_wheel_torque_request as f32,
            brake_wheel_torque_order: value
                .brake_wheel_torque_order
                .enum_value_or_default()
                .eq(&oem::sdv_adas_trajectory_interfaces::AdasBrakeWheelTorqueOrderT::ABWTOT_BRAKE_ACC_AD),
            standstill_req: StandStillReq::FunctionOff,
            aeb_requested: value
                .emergency_brake_decel_order
                .enum_value()
                .unwrap_or_default()
                .into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{StandStillReq, TrajectoryGlobalStatus};
    use acc_interface::{
        datatypes::{
            AdasBrakeWheelTorqueOrderT, AdasStandStillRequestV2T, AdasWheelTorqueCommandT,
            TrajectoryGlobalStatusV2T,
        },
        Calibration,
    };
    use std::time::Duration;
    #[test]
    fn test_from_hold() {
        let value = TrajectoryGlobalStatusV2T {
            pwtTorqueCommand: AdasWheelTorqueCommandT {
                wheelTorqueRequest: 14.0,
                ..Default::default()
            },
            standStillRequest: AdasStandStillRequestV2T::ASSRV2T_HOLD_REQUESTED,
            ..Default::default()
        };
        let result = TrajectoryGlobalStatus::from(&value);
        assert_eq!(result.pwt_wheel_torque, 14.0);
        assert_eq!(result.standstill_req, StandStillReq::HoldRequested);
    }
    #[test]
    fn test_from_function_off() {
        let value = TrajectoryGlobalStatusV2T {
            pwtTorqueCommand: AdasWheelTorqueCommandT {
                wheelTorqueRequest: 0.0,
                ..Default::default()
            },
            standStillRequest: AdasStandStillRequestV2T::ASSRV2T_FUNCTION_OFF,
            ..Default::default()
        };
        let result = TrajectoryGlobalStatus::from(&value);
        assert_eq!(result.pwt_wheel_torque, 0.0);
        assert_eq!(result.standstill_req, StandStillReq::FunctionOff);
    }
    #[test]
    fn test_from_launch_requested() {
        let value = TrajectoryGlobalStatusV2T {
            pwtTorqueCommand: AdasWheelTorqueCommandT {
                wheelTorqueRequest: 20.0,
                ..Default::default()
            },
            standStillRequest: AdasStandStillRequestV2T::ASSRV2T_LAUNCH_REQUESTED,
            ..Default::default()
        };
        let result = TrajectoryGlobalStatus::from(&value);
        assert_eq!(result.pwt_wheel_torque, 20.0);
        assert_eq!(result.standstill_req, StandStillReq::LaunchRequested);
    }

    #[test]
    fn test_cutoff_duration_interpolate() {
        let calib = Calibration::default();
        // take pwt torque when wheelTorqueOrder is true
        let mut traj_status = TrajectoryGlobalStatusV2T {
            pwtTorqueCommand: AdasWheelTorqueCommandT {
                wheelTorqueRequest: 2000.0,
                wheelTorqueOrder: true,
            },
            brakeWheelTorqueRequest: 1000u16,
            standStillRequest: AdasStandStillRequestV2T::ASSRV2T_LAUNCH_REQUESTED,
            ..Default::default()
        };
        let cutoff = TrajectoryGlobalStatus::from(&traj_status).current_cut_off_duration(&calib);
        assert_eq!(cutoff, Ok(2.0));

        // when wheelTorqueOrder is false, if brake wheel order is true, take brake wheel torque for the interpolate
        // otherwise cutoff duration is 0.0
        traj_status.pwtTorqueCommand.wheelTorqueOrder = false;
        let cutoff = TrajectoryGlobalStatus::from(&traj_status).current_cut_off_duration(&calib);
        assert_eq!(cutoff, Ok(0.0));

        traj_status.brakeWheelTorqueOrder = AdasBrakeWheelTorqueOrderT::ABWTOT_BRAKE_ACC_AD;
        let cutoff = TrajectoryGlobalStatus::from(&traj_status).current_cut_off_duration(&calib);
        assert_eq!(cutoff, Ok(1.0));

        let duration = Duration::from_secs_f32(cutoff.unwrap_or_default());
        assert_eq!(duration.as_secs_f32(), 1.0);
    }
}
