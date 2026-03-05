//! Acc Driver Guide
//!

use core::time::Duration;

use acc_interface::{
    datatypes::{DriverGuide, DriverGuideDisplay, FeatureID},
    Calibration,
};
use num_derive::FromPrimitive;

use crate::{utils::to_duration, values::Delay};

/// Wait inhibitions
#[derive(Debug, Clone, Copy, FromPrimitive, Default, PartialEq)]
pub enum WaitConditions {
    /// Driver Belt Unfastened
    DriverBeltUnfastened,
    /// Door open
    OneDoorOpen,
    /// Over accel
    OverAccelerationCounterOn,
    /// Brake Vacuum
    BrakeVacuumCounterOn,
    /// Heavy Rain
    HeavyRain,
    /// Park
    ParkOnWait,
    /// Acc in Failure
    InhibitAccInFailure,
    /// Tailgate Status
    TailGateStatus,
    /// Oem mode
    OemNoModeToDyno,
    /// Maneuver Rejected
    ManeuverRejected,
    /// Aeb Requested
    AebRequested,
    /// Switching to waiting after temporary failure
    AfterTemporaryFailure,
    /// Speed Greater than max eco speed
    EcoSpeedOutOfRange,
    /// No conditions is true
    #[default]
    NotActive,
}

impl From<&WaitConditions> for DriverGuideDisplay {
    fn from(value: &WaitConditions) -> Self {
        use WaitConditions::*;
        match value {
            ParkOnWait => DriverGuideDisplay::DGD_PARKING_BRAKE_ACTIVATED,
            DriverBeltUnfastened => DriverGuideDisplay::DGD_DISCONNECTION_LEVEL1,
            OverAccelerationCounterOn
            | OneDoorOpen
            | AebRequested
            | ManeuverRejected
            | HeavyRain => DriverGuideDisplay::DGD_DISCONNECTION_LEVEL2,
            EcoSpeedOutOfRange => DriverGuideDisplay::DGD_ECO_PRIORITY_MODE,
            _ => DriverGuideDisplay::DGD_NO_DISPLAY,
        }
    }
}

/// Suspend inhibitions
#[derive(Debug, Clone, FromPrimitive, Copy, Default, PartialEq)]
pub enum SuspendConditions {
    /// Driver brake pressed
    DriverBrakePressed,
    /// Gaz with brake
    GazWithBrake,
    /// Out of max Lateral Accel
    OutOfLateralAccelThresholds,
    /// Overspeed
    Overspeed,
    /// Overspeed than ECO Vmax
    OverspeedAtEco,
    /// Rear, Neutral or Park Gear
    InvalidGear,
    /// RolloverDetected
    RolloverDetected,
    /// HMi Request
    HmiRequest,
    /// TargetLost
    TargetLostUnderVmin,
    /// Autorestart timeout
    AutoRestartTimeout,
    /// Low Speed
    LowSpeed,
    /// Low Ego Speed No StopAndGo
    LowEgoSpeedNoStopAndGo,
    /// TSF/AYC/MSR/ABS/ASR/ in regulation
    HigherPrioInRegulation,
    /// Powertrain Inhibition
    PWTInhibit,
    /// Parking Brake Applied
    ParkingBrakeTightened,
    /// Steering wheel angle too high
    SteeringWheelAngleTooHigh,
    /// Get Discrepancy speed state from PP
    DiscrepancySpeed,
    /// CC below min Speed
    CcBelowMinSpeed,
    /// No conditions is true
    #[default]
    NotActive,
}

/// Off Inhibitions
#[derive(Debug, Clone, Default, Copy, FromPrimitive, PartialEq)]
pub enum OffConditions {
    /// Vdc Disabled and state is Off
    VdcDisabledAndOffState,
    /// Vdc for Other states
    VdcDisabled,
    /// Standstill slope
    StandstillMaxSlopeActive,
    /// Standstill requested but not active after 10 sec
    StandstillActivationTimeout,
    /// Not enought powertrain to take off
    NotEnoughPowertrain,
    /// Take Off too long
    TakeOffTooLong,
    /// Switching to Off after permanent failure
    AfterPermanentFailure,
    /// overspeed with door unclosed
    OverspeedOnUnclosedDoor,
    /// No conditons is active
    #[default]
    NotActive,
}

impl From<&OffConditions> for DriverGuideDisplay {
    fn from(value: &OffConditions) -> Self {
        use OffConditions::*;
        match value {
            VdcDisabledAndOffState => DriverGuideDisplay::DGD_VDC_DEACTIVATED,
            VdcDisabled
            | NotEnoughPowertrain
            | StandstillMaxSlopeActive
            | StandstillActivationTimeout
            | TakeOffTooLong => DriverGuideDisplay::DGD_DEACTIVATION_LEVEL1,
            _ => DriverGuideDisplay::DGD_NO_DISPLAY,
        }
    }
}

/// Deactivation Cause
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DeactivationCause {
    /// Restart need manual Action
    ManualRestart,
    /// Automatic Restart
    AutomaticRestart,
    /// waiting conditions
    Wait(WaitConditions),
    /// Susped Conditions
    Suspend(SuspendConditions),
    /// Off Conditions
    Off(OffConditions),
    /// Temporary or Permanent failure when in regulation, override or stop
    FailureDeac2,
    /// Temporary or Permanent failure when in Wait or Suspended
    FailureDeac1,
    /// Refuse to activate
    RefuseActivation,
    /// None conditions is true
    #[default]
    NotActive,
}

impl From<&DeactivationCause> for acc_interface::datatypes::AccDeactivationCause {
    fn from(value: &DeactivationCause) -> Self {
        use acc_interface::datatypes::AccDeactivationCause::*;
        match value {
            DeactivationCause::ManualRestart => ADC_MANUAL_RESTART,
            DeactivationCause::AutomaticRestart => ADC_AUTOMATIC_RESTART,
            DeactivationCause::Wait(WaitConditions::DriverBeltUnfastened) => {
                ADC_DRIVER_BELT_UNFASTENED
            }
            DeactivationCause::Wait(WaitConditions::OneDoorOpen) => ADC_ONE_DOOR_OPEN,
            DeactivationCause::Wait(WaitConditions::OverAccelerationCounterOn) => {
                ADC_OVER_ACCEL_COUNTER_ON
            }
            DeactivationCause::Wait(WaitConditions::BrakeVacuumCounterOn) => {
                ADC_BRAKE_VACUUM_COUNTER_ON
            }
            DeactivationCause::Wait(WaitConditions::HeavyRain) => ADC_HEAVY_RAIN,
            DeactivationCause::Wait(WaitConditions::ParkOnWait) => ADC_PARK_ON_WAIT,
            DeactivationCause::Wait(WaitConditions::InhibitAccInFailure) => {
                ADC_INHIBIT_ACC_IN_FAILURE
            }
            DeactivationCause::Wait(WaitConditions::TailGateStatus) => ADC_TAIL_GATE,
            DeactivationCause::Wait(WaitConditions::OemNoModeToDyno) => ADC_OEM_NO_MODE_TO_DYNO,
            DeactivationCause::Wait(WaitConditions::ManeuverRejected) => ADC_MANEUVER_REJECTED,
            DeactivationCause::Wait(WaitConditions::AebRequested) => ADC_AEB_REQUESTED,
            DeactivationCause::Wait(WaitConditions::AfterTemporaryFailure) => {
                ADC_AFTER_TEMPORARY_FAILURE
            }
            DeactivationCause::Wait(WaitConditions::EcoSpeedOutOfRange) => {
                ADC_ECO_SPEED_OUT_OF_RANGE
            }
            DeactivationCause::Suspend(SuspendConditions::DriverBrakePressed) => {
                ADC_DRIVER_BRAKE_PRESSED
            }
            DeactivationCause::Suspend(SuspendConditions::GazWithBrake) => ADC_GAZ_WITH_BRAKE,
            DeactivationCause::Suspend(SuspendConditions::OutOfLateralAccelThresholds) => {
                ADC_OUT_OF_LATERAL_ACCEL_THRESHOLDS
            }
            DeactivationCause::Suspend(SuspendConditions::Overspeed) => ADC_OVERSPEED,
            DeactivationCause::Suspend(SuspendConditions::InvalidGear) => ADC_INVALID_GEAR,
            DeactivationCause::Suspend(SuspendConditions::RolloverDetected) => {
                ADC_ROLLOVER_DETECTED
            }
            DeactivationCause::Suspend(SuspendConditions::HmiRequest) => ADC_HMI_REQUEST,
            DeactivationCause::Suspend(SuspendConditions::TargetLostUnderVmin) => {
                ADC_TARGET_LOST_UNDER_VMIN
            }
            DeactivationCause::Suspend(SuspendConditions::AutoRestartTimeout) => {
                ADC_AUTO_RESTART_TIMEOUT
            }
            DeactivationCause::Suspend(SuspendConditions::LowSpeed) => ADC_LOW_SPEED,
            DeactivationCause::Suspend(SuspendConditions::LowEgoSpeedNoStopAndGo) => {
                ADC_LOW_EGO_SPEED_NO_STOP_AND_GO
            }
            DeactivationCause::Suspend(SuspendConditions::HigherPrioInRegulation) => {
                ADC_HIGHER_PRIO_IN_REGULATION
            }
            DeactivationCause::Suspend(SuspendConditions::PWTInhibit) => ADC_PWT_INHIBIT,
            DeactivationCause::Suspend(SuspendConditions::ParkingBrakeTightened) => {
                ADC_PARKING_BRAKE_TIGHTENED
            }
            DeactivationCause::Suspend(SuspendConditions::SteeringWheelAngleTooHigh) => {
                ADC_STEERING_WHEEL_ANGLE_TOO_HIGH
            }
            DeactivationCause::Suspend(SuspendConditions::DiscrepancySpeed) => {
                ADC_DISCREPANCY_SPEED
            }
            DeactivationCause::Suspend(SuspendConditions::CcBelowMinSpeed) => {
                ADC_CC_BELOW_MIN_SPEED
            }
            DeactivationCause::Suspend(SuspendConditions::OverspeedAtEco) => {
                ADC_ECO_SPEED_OUT_OF_RANGE
            }
            DeactivationCause::Off(OffConditions::VdcDisabledAndOffState) => {
                ADC_VDC_DISABLED_AND_OFF_STATE
            }
            DeactivationCause::Off(OffConditions::VdcDisabled) => ADC_VDC_DISABLED,
            DeactivationCause::Off(OffConditions::StandstillMaxSlopeActive) => {
                ADC_STANDSTILL_MAX_SLOPE_ACTIVE
            }
            DeactivationCause::Off(OffConditions::StandstillActivationTimeout) => {
                ADC_STANDSTILL_ACTIVATION_TIMEOUT
            }
            DeactivationCause::Off(OffConditions::NotEnoughPowertrain) => ADC_NOT_ENOUGH_POWERTRAIN,
            DeactivationCause::Off(OffConditions::TakeOffTooLong) => ADC_TAKE_OFF_TOO_LONG,
            DeactivationCause::Off(OffConditions::AfterPermanentFailure) => {
                ADC_AFTER_PERMANENT_FAILURE
            }
            DeactivationCause::Off(OffConditions::OverspeedOnUnclosedDoor) => {
                ADC_OVERSPEED_ON_UNCLOSED_DOOR
            }
            DeactivationCause::FailureDeac2 => ADC_FAILURE_DEACT_2,
            DeactivationCause::FailureDeac1 => ADC_FAILURE_DEACT_1,
            DeactivationCause::RefuseActivation => ADC_REFUSE_ACTIVATION,
            DeactivationCause::NotActive
            | DeactivationCause::Wait(WaitConditions::NotActive)
            | DeactivationCause::Suspend(SuspendConditions::NotActive)
            | DeactivationCause::Off(OffConditions::NotActive) => ADC_NOT_ACTIVE,
        }
    }
}

impl From<WaitConditions> for DeactivationCause {
    fn from(value: WaitConditions) -> Self {
        Self::Wait(value)
    }
}

impl From<SuspendConditions> for DeactivationCause {
    fn from(value: SuspendConditions) -> Self {
        Self::Suspend(value)
    }
}

impl From<OffConditions> for DeactivationCause {
    fn from(value: OffConditions) -> Self {
        Self::Off(value)
    }
}

impl From<&SuspendConditions> for DriverGuideDisplay {
    fn from(value: &SuspendConditions) -> Self {
        use SuspendConditions::*;
        match value {
            InvalidGear | AutoRestartTimeout | PWTInhibit | ParkingBrakeTightened => {
                DriverGuideDisplay::DGD_DISCONNECTION_LEVEL1
            }
            Overspeed
            | OutOfLateralAccelThresholds
            | HigherPrioInRegulation
            | LowEgoSpeedNoStopAndGo
            | SteeringWheelAngleTooHigh
            | DiscrepancySpeed
            | CcBelowMinSpeed => DriverGuideDisplay::DGD_DISCONNECTION_LEVEL2,
            OverspeedAtEco => DriverGuideDisplay::DGD_ECO_PRIORITY_MODE,
            _ => DriverGuideDisplay::DGD_NO_DISPLAY,
        }
    }
}

impl From<&DeactivationCause> for DriverGuide {
    fn from(value: &DeactivationCause) -> Self {
        use DeactivationCause::*;
        let driver_guide = match value {
            Suspend(v) => v.into(),
            Wait(v) => v.into(),
            Off(v) => v.into(),
            NotActive | RefuseActivation => DriverGuideDisplay::DGD_NO_DISPLAY,
            ManualRestart => DriverGuideDisplay::DGD_ACC_ACT_TO_START_REG,
            AutomaticRestart => DriverGuideDisplay::DGD_ACC_REG_READY_TO_GO,
            FailureDeac1 => DriverGuideDisplay::DGD_DEACTIVATION_LEVEL1,
            FailureDeac2 => DriverGuideDisplay::DGD_DEACTIVATION_LEVEL2,
        };
        DriverGuide {
            feature_id: FeatureID::FID_ACC,
            driver_guide_display: driver_guide,
        }
    }
}

/// HMI Driver Guide
#[derive(Debug, Default)]
pub struct HmiDriverGuide {
    /// Driver Guide Display
    pub driver_guide: DeactivationCause,
    /// Send Driver Guide
    pub send_driver_guide: bool,
    /// Delayed Dg already trigerred
    delay_dg: bool,
    /// Timer
    delay_timer: Delay<bool>,
}

impl HmiDriverGuide {
    /// Send without Delay
    fn dg_no_delay(&self, cur: DriverGuideDisplay) -> bool {
        use DriverGuideDisplay::*;
        matches!(
            cur,
            DGD_ACC_ACT_TO_START_REG
                | DGD_ACC_REG_READY_TO_GO
                | DGD_NO_DISPLAY
                | DGD_DISCONNECTION_LEVEL1
                | DGD_DISCONNECTION_LEVEL2
                | DGD_DEACTIVATION_LEVEL1
                | DGD_DEACTIVATION_LEVEL2
                | DGD_VDC_DEACTIVATED
                | DGD_ECO_PRIORITY_MODE
                | DGD_INVALID_SPEED_ECO
        )
    }

    /// send driver guide
    pub fn update_dg(&mut self, prev: &DeactivationCause, elapsed: Duration, calib: &Calibration) {
        let cur_dg = DriverGuide::from(&self.driver_guide).driver_guide_display;
        let prev_dg = DriverGuide::from(prev).driver_guide_display;
        let dg_changed = cur_dg.ne(&prev_dg);
        self.delay_timer.step(
            !self.dg_no_delay(cur_dg),
            to_duration(calib.P_tng_s_Acc_DriverGuideTimeout),
            Duration::ZERO,
            elapsed,
        );
        if dg_changed {
            self.delay_timer.reset();
        }
        self.send_driver_guide = (dg_changed && self.dg_no_delay(cur_dg))
            || (self.delay_timer.is_on() && !self.delay_dg);
        self.delay_dg = self.delay_timer.is_on();
    }
}

#[cfg(test)]
mod test {
    use num_traits::FromPrimitive;

    use super::*;
    #[test]
    fn test_dg_no_delay() {
        let mut prev_dc = DeactivationCause::NotActive;
        let mut cur_dc = DeactivationCause::AutomaticRestart;
        let mut dg = HmiDriverGuide {
            driver_guide: cur_dc,
            ..Default::default()
        };
        let mut elapsed = Duration::from_millis(10);
        dg.update_dg(&prev_dc, elapsed, &Calibration::default());
        println!(
            "{:?}",
            dg.dg_no_delay(DriverGuide::from(&cur_dc).driver_guide_display)
        );
        assert!(dg.send_driver_guide);

        prev_dc = dg.driver_guide;
        dg.update_dg(&prev_dc, elapsed, &Calibration::default());
        assert!(!dg.send_driver_guide);

        dg.driver_guide = DeactivationCause::Off(OffConditions::TakeOffTooLong);
        elapsed = Duration::from_millis(20);
        dg.update_dg(&prev_dc, elapsed, &Calibration::default());
        assert!(dg.send_driver_guide);

        elapsed = Duration::from_millis(20);
        prev_dc = dg.driver_guide;
        dg.update_dg(&prev_dc, elapsed, &Calibration::default());
        assert!(!dg.send_driver_guide);
        elapsed = Duration::from_millis(20);
        dg.driver_guide = DeactivationCause::NotActive;
        dg.update_dg(&prev_dc, elapsed, &Calibration::default());
        assert!(dg.send_driver_guide);

        elapsed = Duration::from_millis(2000);
        prev_dc = dg.driver_guide;
        dg.update_dg(&prev_dc, elapsed, &Calibration::default());
        assert!(!dg.send_driver_guide);

        cur_dc = DeactivationCause::default();
        dg.driver_guide = cur_dc;
        dg.update_dg(&prev_dc, elapsed, &Calibration::default());
        assert!(!dg.send_driver_guide);
    }

    #[test]
    fn test_send_dg() {
        let mut prev_dc = DeactivationCause::default();
        let mut cur_dc = DeactivationCause::Off(OffConditions::StandstillActivationTimeout);
        let mut dg = HmiDriverGuide {
            driver_guide: cur_dc,
            ..Default::default()
        };
        let mut elapsed = Duration::from_millis(10);
        dg.update_dg(&prev_dc, elapsed, &Calibration::default());
        assert!(dg.send_driver_guide);

        prev_dc = cur_dc;
        dg.update_dg(&prev_dc, elapsed, &Calibration::default());
        assert!(!dg.send_driver_guide);

        cur_dc = DeactivationCause::Wait(WaitConditions::ParkOnWait);
        dg.driver_guide = cur_dc;
        elapsed = Duration::from_millis(20);
        dg.update_dg(&prev_dc, elapsed, &Calibration::default());
        assert!(!dg.send_driver_guide);

        elapsed = Duration::from_millis(100);
        dg.update_dg(&prev_dc, elapsed, &Calibration::default());

        assert!(!dg.send_driver_guide);

        elapsed = Duration::from_millis(2000);
        prev_dc = cur_dc;
        dg.update_dg(&prev_dc, elapsed, &Calibration::default());
        assert!(dg.send_driver_guide);

        dg.update_dg(&prev_dc, elapsed, &Calibration::default());
        println!("{dg:#?}");
        assert!(!dg.send_driver_guide);
        cur_dc = DeactivationCause::default();
        dg.driver_guide = cur_dc;
        dg.update_dg(&prev_dc, elapsed, &Calibration::default());
        assert!(dg.send_driver_guide);
        prev_dc = cur_dc;
        dg.update_dg(&prev_dc, elapsed, &Calibration::default());
        assert!(!dg.send_driver_guide);
    }

    #[test]
    fn test_dg_out_of_range() {
        let dc = DeactivationCause::Off(OffConditions::from_usize(100).unwrap_or_default());
        assert_eq!(dc, DeactivationCause::Off(OffConditions::NotActive));
        let dc = DeactivationCause::Suspend(SuspendConditions::from_usize(100).unwrap_or_default());
        assert_eq!(dc, DeactivationCause::Suspend(SuspendConditions::NotActive));
        let dc = DeactivationCause::Wait(WaitConditions::from_usize(100).unwrap_or_default());
        assert_eq!(dc, DeactivationCause::Wait(WaitConditions::NotActive));
    }

    #[test]
    fn test_wait_conditions_to_driver_guide_display() {
        use WaitConditions::*;
        assert_eq!(
            DriverGuideDisplay::from(&OverAccelerationCounterOn),
            DriverGuideDisplay::DGD_DISCONNECTION_LEVEL2
        );
        assert_eq!(
            DriverGuideDisplay::from(&ParkOnWait),
            DriverGuideDisplay::DGD_PARKING_BRAKE_ACTIVATED
        );
        assert_eq!(
            DriverGuideDisplay::from(&DriverBeltUnfastened),
            DriverGuideDisplay::DGD_DISCONNECTION_LEVEL1
        );
        assert_eq!(
            DriverGuideDisplay::from(&OneDoorOpen),
            DriverGuideDisplay::DGD_DISCONNECTION_LEVEL2
        );
        assert_eq!(
            DriverGuideDisplay::from(&AebRequested),
            DriverGuideDisplay::DGD_DISCONNECTION_LEVEL2
        );
        assert_eq!(
            DriverGuideDisplay::from(&ManeuverRejected),
            DriverGuideDisplay::DGD_DISCONNECTION_LEVEL2
        );
        assert_eq!(
            DriverGuideDisplay::from(&NotActive),
            DriverGuideDisplay::DGD_NO_DISPLAY
        );
    }

    #[test]
    fn test_suspend_conditions_to_driver_guide_display() {
        use SuspendConditions::*;
        assert_eq!(
            DriverGuideDisplay::from(&DriverBrakePressed),
            DriverGuideDisplay::DGD_NO_DISPLAY
        );
        assert_eq!(
            DriverGuideDisplay::from(&InvalidGear),
            DriverGuideDisplay::DGD_DISCONNECTION_LEVEL1
        );
        assert_eq!(
            DriverGuideDisplay::from(&Overspeed),
            DriverGuideDisplay::DGD_DISCONNECTION_LEVEL2
        );
        assert_eq!(
            DriverGuideDisplay::from(&OutOfLateralAccelThresholds),
            DriverGuideDisplay::DGD_DISCONNECTION_LEVEL2
        );
        assert_eq!(
            DriverGuideDisplay::from(&ParkingBrakeTightened),
            DriverGuideDisplay::DGD_DISCONNECTION_LEVEL1
        );
        assert_eq!(
            DriverGuideDisplay::from(&NotActive),
            DriverGuideDisplay::DGD_NO_DISPLAY
        );
    }

    #[test]
    fn test_off_conditions_to_driver_guide_display() {
        use OffConditions::*;
        assert_eq!(
            DriverGuideDisplay::from(&VdcDisabledAndOffState),
            DriverGuideDisplay::DGD_VDC_DEACTIVATED
        );
        assert_eq!(
            DriverGuideDisplay::from(&VdcDisabled),
            DriverGuideDisplay::DGD_DEACTIVATION_LEVEL1
        );
        assert_eq!(
            DriverGuideDisplay::from(&NotActive),
            DriverGuideDisplay::DGD_NO_DISPLAY
        );
    }

    #[test]
    fn test_deactivation_cause_to_driver_guide() {
        use DeactivationCause::*;
        assert_eq!(
            DriverGuide::from(&ManualRestart).driver_guide_display,
            DriverGuideDisplay::DGD_ACC_ACT_TO_START_REG
        );
        assert_eq!(
            DriverGuide::from(&AutomaticRestart).driver_guide_display,
            DriverGuideDisplay::DGD_ACC_REG_READY_TO_GO
        );
    }

    #[test]
    fn test_deactivation_cause_nested_conditions() {
        use DeactivationCause::*;
        use OffConditions::*;
        use SuspendConditions::*;
        use WaitConditions::*;

        let wait_condition = Wait(DriverBeltUnfastened);
        assert_eq!(
            DriverGuide::from(&wait_condition).driver_guide_display,
            DriverGuideDisplay::DGD_DISCONNECTION_LEVEL1
        );

        let suspend_condition = Suspend(DriverBrakePressed);
        assert_eq!(
            DriverGuide::from(&suspend_condition).driver_guide_display,
            DriverGuideDisplay::DGD_NO_DISPLAY
        );

        let off_condition = Off(VdcDisabledAndOffState);
        assert_eq!(
            DriverGuide::from(&off_condition).driver_guide_display,
            DriverGuideDisplay::DGD_VDC_DEACTIVATED
        );
    }

    #[test]
    fn test_corner_cases() {
        // Test with default values
        let not_active = DeactivationCause::NotActive;
        assert_eq!(
            DriverGuide::from(&not_active).driver_guide_display,
            DriverGuideDisplay::DGD_NO_DISPLAY
        );
        let invalid_wait_condition = WaitConditions::default();
        assert_eq!(
            DriverGuideDisplay::from(&invalid_wait_condition),
            DriverGuideDisplay::DGD_NO_DISPLAY
        );

        let invalid_suspend_condition = SuspendConditions::default();
        assert_eq!(
            DriverGuideDisplay::from(&invalid_suspend_condition),
            DriverGuideDisplay::DGD_NO_DISPLAY
        );

        let invalid_off_condition = OffConditions::default();
        assert_eq!(
            DriverGuideDisplay::from(&invalid_off_condition),
            DriverGuideDisplay::DGD_NO_DISPLAY
        );
    }

    #[test]
    fn test_deactivation_cause_conditions() {
        use DeactivationCause::*;
        use OffConditions::*;
        use SuspendConditions::*;
        use WaitConditions::*;

        let wait_condition = Wait(DriverBeltUnfastened);
        assert_eq!(
            DriverGuide::from(&wait_condition).driver_guide_display,
            DriverGuideDisplay::DGD_DISCONNECTION_LEVEL1
        );

        let suspend_condition = Suspend(DriverBrakePressed);
        assert_eq!(
            DriverGuide::from(&suspend_condition).driver_guide_display,
            DriverGuideDisplay::DGD_NO_DISPLAY
        );

        let off_condition = Off(VdcDisabledAndOffState);
        assert_eq!(
            DriverGuide::from(&off_condition).driver_guide_display,
            DriverGuideDisplay::DGD_VDC_DEACTIVATED
        );
    }

    #[test]
    fn test_negative_index_handling() {
        // Ensure no panic if a negative index is accidentally cast (e.g., from an unsigned overflow)
        let negative_index = -1_i32 as usize;
        let fallback_condition = SuspendConditions::from_usize(negative_index).unwrap_or_default();
        assert_eq!(fallback_condition, SuspendConditions::NotActive);

        let deactivation_cause = DeactivationCause::Suspend(fallback_condition);
        assert_eq!(
            deactivation_cause,
            DeactivationCause::Suspend(SuspendConditions::NotActive)
        );
    }

    #[test]
    fn test_uninitialized_index_handling() {
        // Simulate an uninitialized index (default usize is 0)
        let uninitialized_index = usize::default();
        let fallback_condition =
            SuspendConditions::from_usize(uninitialized_index).unwrap_or_default();
        assert_eq!(fallback_condition, SuspendConditions::DriverBrakePressed);

        let deactivation_cause = DeactivationCause::Suspend(fallback_condition);
        assert_eq!(
            deactivation_cause,
            DeactivationCause::Suspend(SuspendConditions::DriverBrakePressed)
        );
    }
}
