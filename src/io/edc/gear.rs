//! Define ego Gear Status for ACC
//!
use acc_interface::datatypes::{gear_t, GearLeverPosition};
#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;

#[derive(Debug, Clone, Default)]
/// Ego Gear Status
pub struct Gear {
    /// Global standstill status
    pub is_rear_gear_engaged: bool,
    /// Park brake req
    pub gear_level_position: GearLvlPos,
}

/// Ego Gear Level Position
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum GearLvlPos {
    #[default]
    /// Parking
    Parking,
    /// Reverse
    Reverse,
    /// Neutral
    Neutral,
    /// Drive
    Drive,
    /// Manual Mode
    ManualMode,
    /// lowest gear when driving at a slow speed
    L,
    /// Ds Gear Level pos
    DS,
    /// Confirmed Level Pos
    ConfirmedLvlPos,
    /// Brake
    Brake,
    /// Not Used
    NotUsed,
}

impl From<GearLeverPosition> for GearLvlPos {
    fn from(value: GearLeverPosition) -> Self {
        match value {
            GearLeverPosition::GLP_PARKING => Self::Parking,
            GearLeverPosition::GLP_REVERSE => Self::Reverse,
            GearLeverPosition::GLP_NEUTRAL => Self::Neutral,
            GearLeverPosition::GLP_DRIVE => Self::Drive,
            GearLeverPosition::GLP_MANUAL_MODE => Self::ManualMode,
            GearLeverPosition::GLP_L => Self::L,
            GearLeverPosition::GLP_CONFIRMED_LEVER_POSITION => Self::ConfirmedLvlPos,
            GearLeverPosition::GLP_DS => Self::DS,
            GearLeverPosition::GLP_BRAKE => Self::Brake,
            _ => Self::NotUsed,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_gear::GearLeverPosition> for GearLvlPos {
    fn from(value: oem::sdv_adas_ego_ego_gear::GearLeverPosition) -> Self {
        use oem::sdv_adas_ego_ego_gear::GearLeverPosition;
        match value {
            GearLeverPosition::GLP_PARKING => Self::Parking,
            GearLeverPosition::GLP_REVERSE => Self::Reverse,
            GearLeverPosition::GLP_NEUTRAL => Self::Neutral,
            GearLeverPosition::GLP_DRIVE => Self::Drive,
            GearLeverPosition::GLP_MANUAL_MODE => Self::ManualMode,
            GearLeverPosition::GLP_L => Self::L,
            GearLeverPosition::GLP_CONFIRMED_LEVER_POSITION => Self::ConfirmedLvlPos,
            GearLeverPosition::GLP_DS => Self::DS,
            GearLeverPosition::GLP_BRAKE => Self::Brake,
            _ => Self::NotUsed,
        }
    }
}

impl From<&gear_t> for Gear {
    fn from(value: &gear_t) -> Self {
        Self {
            is_rear_gear_engaged: value.rear,
            gear_level_position: value.gearLeverPosition.into(),
        }
    }
}

#[cfg(feature = "caros")]
impl From<&oem::sdv_adas_ego_ego_gear::Gear_t> for Gear {
    fn from(value: &oem::sdv_adas_ego_ego_gear::Gear_t) -> Self {
        Self {
            is_rear_gear_engaged: value.rear,
            gear_level_position: value
                .gear_lever_position
                .enum_value()
                .unwrap_or_default()
                .into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_conversion() {
        let gear_t = gear_t {
            rear: true,
            gearLeverPosition: GearLeverPosition::GLP_DRIVE,
            ..Default::default()
        };

        let gear: Gear = (&gear_t).into();

        assert!(gear.is_rear_gear_engaged);
        assert_eq!(gear.gear_level_position, GearLvlPos::Drive);
    }

    #[test]
    fn test_gear_lvl_pos_conversion() {
        assert_eq!(
            GearLvlPos::from(GearLeverPosition::GLP_PARKING),
            GearLvlPos::Parking
        );
        assert_eq!(
            GearLvlPos::from(GearLeverPosition::GLP_REVERSE),
            GearLvlPos::Reverse
        );
        assert_eq!(
            GearLvlPos::from(GearLeverPosition::GLP_NEUTRAL),
            GearLvlPos::Neutral
        );
        assert_eq!(
            GearLvlPos::from(GearLeverPosition::GLP_DRIVE),
            GearLvlPos::Drive
        );
        assert_eq!(
            GearLvlPos::from(GearLeverPosition::GLP_MANUAL_MODE),
            GearLvlPos::ManualMode
        );
        assert_eq!(GearLvlPos::from(GearLeverPosition::GLP_L), GearLvlPos::L);
        assert_eq!(
            GearLvlPos::from(GearLeverPosition::GLP_CONFIRMED_LEVER_POSITION),
            GearLvlPos::ConfirmedLvlPos
        );
        assert_eq!(GearLvlPos::from(GearLeverPosition::GLP_DS), GearLvlPos::DS);
        assert_eq!(
            GearLvlPos::from(GearLeverPosition::GLP_BRAKE),
            GearLvlPos::Brake
        );
        assert_eq!(
            GearLvlPos::from(GearLeverPosition::GLP_NOT_AVAILABLE_INIT_OR_LEVER_SWITCHED_FAILURE),
            GearLvlPos::NotUsed
        );
        assert_eq!(
            GearLvlPos::from(GearLeverPosition::GLP_NOT_USED1),
            GearLvlPos::NotUsed
        );
    }
}
