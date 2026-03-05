//! Define ego Pedal Status for ACC
//!
use acc_interface::datatypes::{pedals_t, AccelPedalKickDown, BrakePedal};

use super::brake::BrakeStatus;
#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;

/// KickDown state
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum KickDownState {
    /// Deactivated state
    #[default]
    Deactivated,
    /// Activated state
    Activated,
    /// Undefined state
    Undefined,
}

/// Pedal Status
#[derive(Debug, Default, Clone)]
pub struct Pedal {
    /// kickdown state
    pub kickdown: KickDownState,
    /// brake pedal state
    pub brake_pedal: BrakeStatus,
    /// accel_pedal
    pub accel_pedal: f32,
}

impl From<&pedals_t> for Pedal {
    fn from(value: &pedals_t) -> Self {
        Self {
            kickdown: value.accelPedalKickDown.into(),
            brake_pedal: value.brakePedal.into(),
            accel_pedal: value.accelPedal,
        }
    }
}

#[cfg(feature = "caros")]
impl From<&oem::sdv_adas_ego_ego_pedals::Pedals_t> for Pedal {
    fn from(value: &oem::sdv_adas_ego_ego_pedals::Pedals_t) -> Self {
        Self {
            kickdown: value
                .accel_pedal_kick_down
                .enum_value()
                .unwrap_or_default()
                .into(),
            brake_pedal: value.brake_pedal.enum_value().unwrap_or_default().into(),
            accel_pedal: value.accel_pedal,
        }
    }
}

impl From<AccelPedalKickDown> for KickDownState {
    fn from(value: AccelPedalKickDown) -> Self {
        match value {
            AccelPedalKickDown::APKD_NOT_ACTIVATED => Self::Deactivated,
            AccelPedalKickDown::APKD_ACTIVATED => Self::Activated,
            AccelPedalKickDown::APKD_UNDEFINED => Self::Undefined,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_pedals::AccelPedalKickDown> for KickDownState {
    fn from(value: oem::sdv_adas_ego_ego_pedals::AccelPedalKickDown) -> Self {
        use oem::sdv_adas_ego_ego_pedals::AccelPedalKickDown;
        match value {
            AccelPedalKickDown::APKD_NOT_ACTIVATED => Self::Deactivated,
            AccelPedalKickDown::APKD_ACTIVATED => Self::Activated,
            AccelPedalKickDown::APKD_UNDEFINED => Self::Undefined,
        }
    }
}

impl From<BrakePedal> for BrakeStatus {
    fn from(value: BrakePedal) -> Self {
        match value {
            BrakePedal::BRAKE_PEDAL_PRESSED => Self::Pressed,
            BrakePedal::BRAKE_PEDAL_NOT_PRESSED => Self::NotPressed,
            BrakePedal::BRAKE_PEDAL_NOT_AVAILABLE | BrakePedal::BRAKE_PEDAL_NOT_USED => {
                Self::Unknown
            }
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_pedals::BrakePedal> for BrakeStatus {
    fn from(value: oem::sdv_adas_ego_ego_pedals::BrakePedal) -> Self {
        use oem::sdv_adas_ego_ego_pedals::BrakePedal;
        match value {
            BrakePedal::BRAKE_PEDAL_PRESSED => Self::Pressed,
            BrakePedal::BRAKE_PEDAL_NOT_PRESSED => Self::NotPressed,
            BrakePedal::BRAKE_PEDAL_NOT_AVAILABLE | BrakePedal::BRAKE_PEDAL_NOT_USED => {
                Self::Unknown
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pedal_conversion() {
        let pedals_t = pedals_t {
            accelPedalKickDown: AccelPedalKickDown::APKD_ACTIVATED,
            brakePedal: BrakePedal::BRAKE_PEDAL_PRESSED,
            ..Default::default()
        };

        let pedal: Pedal = (&pedals_t).into();

        assert_eq!(pedal.kickdown, KickDownState::Activated);
        assert_eq!(pedal.brake_pedal, BrakeStatus::Pressed);
    }

    #[test]
    fn test_kickdown_state_conversion() {
        assert_eq!(
            KickDownState::from(AccelPedalKickDown::APKD_NOT_ACTIVATED),
            KickDownState::Deactivated
        );
        assert_eq!(
            KickDownState::from(AccelPedalKickDown::APKD_ACTIVATED),
            KickDownState::Activated
        );
        assert_eq!(
            KickDownState::from(AccelPedalKickDown::APKD_UNDEFINED),
            KickDownState::Undefined
        );
    }

    #[test]
    fn test_brake_status_conversion() {
        assert_eq!(
            BrakeStatus::from(BrakePedal::BRAKE_PEDAL_PRESSED),
            BrakeStatus::Pressed
        );
        assert_eq!(
            BrakeStatus::from(BrakePedal::BRAKE_PEDAL_NOT_PRESSED),
            BrakeStatus::NotPressed
        );
        assert_eq!(
            BrakeStatus::from(BrakePedal::BRAKE_PEDAL_NOT_AVAILABLE),
            BrakeStatus::Unknown
        );
        assert_eq!(
            BrakeStatus::from(BrakePedal::BRAKE_PEDAL_NOT_USED),
            BrakeStatus::Unknown
        );
    }
}
