//! Define ego Brake Status for ACC
//!
use acc_interface::datatypes::{braking_t, BrakeInfoStatusEgo};

#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;

/// Brake Info Status
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum BrakeStatus {
    /// Not Pressed
    #[default]
    NotPressed,
    /// Pressed
    Pressed,
    /// Confirmed Pressed
    ConfirmedPressed,
    /// Unknown
    Unknown,
}

/// Define EgoBraking Status
#[derive(Debug, Default, Clone)]
pub struct Braking {
    /// Brake Info Status Ego
    pub info: BrakeStatus,
    /// Brake Wheel Torque Estimation
    pub whl_tq_estimation: u16,
    /// Brake pressure
    pub pressure: f32,
}

impl From<&braking_t> for Braking {
    fn from(value: &braking_t) -> Self {
        Self {
            info: value.infoStatus.into(),
            pressure: value.pressure,
            whl_tq_estimation: value.brakeWhlTqEstimation,
        }
    }
}

#[cfg(feature = "caros")]
impl From<&oem::sdv_adas_ego_ego_braking::Braking_t> for Braking {
    fn from(value: &oem::sdv_adas_ego_ego_braking::Braking_t) -> Self {
        Self {
            info: value.info_status.enum_value().unwrap_or_default().into(),
            pressure: value.pressure,
            whl_tq_estimation: value.brake_whl_tq_estimation as u16,
        }
    }
}

impl From<BrakeInfoStatusEgo> for BrakeStatus {
    fn from(value: BrakeInfoStatusEgo) -> Self {
        match value {
            BrakeInfoStatusEgo::BISE_BRAKE_PEDAL_CONFIRMED_PRESSED => Self::ConfirmedPressed,
            BrakeInfoStatusEgo::BISE_BRAKE_PEDAL_PRESSED => Self::Pressed,
            BrakeInfoStatusEgo::BISE_BRAKE_PEDAL_NOT_PRESSED => Self::NotPressed,
            BrakeInfoStatusEgo::BISE_NOT_AVAILABLE | BrakeInfoStatusEgo::BISE_UNSPECIFIED => {
                Self::Unknown
            }
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_braking::BrakeInfoStatusEgo> for BrakeStatus {
    fn from(value: oem::sdv_adas_ego_ego_braking::BrakeInfoStatusEgo) -> Self {
        use oem::sdv_adas_ego_ego_braking::BrakeInfoStatusEgo;
        match value {
            BrakeInfoStatusEgo::BISE_BRAKE_PEDAL_CONFIRMED_PRESSED => Self::ConfirmedPressed,
            BrakeInfoStatusEgo::BISE_BRAKE_PEDAL_PRESSED => Self::Pressed,
            BrakeInfoStatusEgo::BISE_BRAKE_PEDAL_NOT_PRESSED => Self::NotPressed,
            BrakeInfoStatusEgo::BISE_NOT_AVAILABLE | BrakeInfoStatusEgo::BISE_UNSPECIFIED => {
                Self::Unknown
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_braking_conversion() {
        let brake_t = braking_t {
            infoStatus: BrakeInfoStatusEgo::BISE_BRAKE_PEDAL_CONFIRMED_PRESSED,
            brakeWhlTqEstimation: 100,
            ..Default::default()
        };

        let braking: Braking = (&brake_t).into();

        assert_eq!(braking.info, BrakeStatus::ConfirmedPressed);
        assert_eq!(braking.whl_tq_estimation, 100);
    }

    #[test]
    fn test_brake_status_conversion() {
        assert_eq!(
            BrakeStatus::from(BrakeInfoStatusEgo::BISE_BRAKE_PEDAL_CONFIRMED_PRESSED),
            BrakeStatus::ConfirmedPressed
        );
        assert_eq!(
            BrakeStatus::from(BrakeInfoStatusEgo::BISE_BRAKE_PEDAL_PRESSED),
            BrakeStatus::Pressed
        );
        assert_eq!(
            BrakeStatus::from(BrakeInfoStatusEgo::BISE_BRAKE_PEDAL_NOT_PRESSED),
            BrakeStatus::NotPressed
        );
        assert_eq!(
            BrakeStatus::from(BrakeInfoStatusEgo::BISE_NOT_AVAILABLE),
            BrakeStatus::Unknown
        );
    }
}
