//! Define ego Lights Status for ACC
//!
use acc_interface::datatypes::{
    lighting_t, BlinkersStatus, FlashingIndicatorStatus, TurnSignalCmdSte,
};
#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;

/// Turn Signal Cmd State
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TurnSignalCmdState {
    /// Off
    #[default]
    Off,
    /// Left
    Left,
    /// Right
    Right,
}

/// Flashing Indicator Status
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum OnOffState {
    /// Unknonw state
    #[default]
    Unknown,
    /// OFF
    AllOff,
    /// Right ON
    RightOn,
    /// left On
    LeftOn,
    /// Left and right on
    AllOn,
}

/// Define Ego Lights Status
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Lights {
    /// Turn Signal Cmd State
    pub turn_signal_cmd: TurnSignalCmdState,
    /// Flashing Indicator Status
    pub flashing_indicator: OnOffState,
    /// blinker Status
    pub blinker_status: OnOffState,
    /// Overtaking Blinker Status
    pub overtaking_confirmation: bool,
}

impl From<&lighting_t> for Lights {
    fn from(value: &lighting_t) -> Self {
        Self {
            turn_signal_cmd: value.flashingIndicatorPosition.into(),
            flashing_indicator: value.flashingIndicatorStatus.into(),
            blinker_status: value.blinkersStatus.into(),
            ..Default::default()
        }
    }
}

#[cfg(feature = "caros")]
impl From<&oem::sdv_adas_ego_ego_lights::Lighting_t> for Lights {
    fn from(value: &oem::sdv_adas_ego_ego_lights::Lighting_t) -> Self {
        Self {
            turn_signal_cmd: value
                .flashing_indicator_position
                .enum_value()
                .unwrap_or_default()
                .into(),
            flashing_indicator: value
                .flashing_indicator_status
                .enum_value()
                .unwrap_or_default()
                .into(),
            blinker_status: value
                .blinkers_status
                .enum_value()
                .unwrap_or_default()
                .into(),
            ..Default::default()
        }
    }
}

impl From<TurnSignalCmdSte> for TurnSignalCmdState {
    fn from(value: TurnSignalCmdSte) -> Self {
        match value {
            TurnSignalCmdSte::TSCS_LEFT => Self::Left,
            TurnSignalCmdSte::TSCS_RIGHT => Self::Right,
            TurnSignalCmdSte::TSCS_OFF => Self::Off,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_lights::TurnSignalCmdSte> for TurnSignalCmdState {
    fn from(value: oem::sdv_adas_ego_ego_lights::TurnSignalCmdSte) -> Self {
        use oem::sdv_adas_ego_ego_lights::TurnSignalCmdSte;
        match value {
            TurnSignalCmdSte::TSCS_LEFT => Self::Left,
            TurnSignalCmdSte::TSCS_RIGHT => Self::Right,
            TurnSignalCmdSte::TSCS_OFF => Self::Off,
        }
    }
}

impl From<FlashingIndicatorStatus> for OnOffState {
    fn from(value: FlashingIndicatorStatus) -> Self {
        match value {
            FlashingIndicatorStatus::FIS_LEFT_ON_RIGHT_ON => Self::AllOn,
            FlashingIndicatorStatus::FIS_LEFT_OFF_RIGHT_ON => Self::RightOn,
            FlashingIndicatorStatus::FIS_LEFT_ON_RIGHT_OFF => Self::LeftOn,
            FlashingIndicatorStatus::FIS_LEFT_AND_RIGHT_FLASHING_INDICATORS_OFF => Self::AllOff,
            _ => Self::Unknown,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_lights::FlashingIndicatorStatus> for OnOffState {
    fn from(value: oem::sdv_adas_ego_ego_lights::FlashingIndicatorStatus) -> Self {
        use oem::sdv_adas_ego_ego_lights::FlashingIndicatorStatus;
        match value {
            FlashingIndicatorStatus::FIS_LEFT_ON_RIGHT_ON => Self::AllOn,
            FlashingIndicatorStatus::FIS_LEFT_OFF_RIGHT_ON => Self::RightOn,
            FlashingIndicatorStatus::FIS_LEFT_ON_RIGHT_OFF => Self::LeftOn,
            FlashingIndicatorStatus::FIS_LEFT_AND_RIGHT_FLASHING_INDICATORS_OFF => Self::AllOff,
            _ => Self::Unknown,
        }
    }
}

impl From<BlinkersStatus> for OnOffState {
    fn from(value: BlinkersStatus) -> Self {
        match value {
            BlinkersStatus::BLINKERS_STATUS_LEFT_ON_RIGHT_ON => Self::AllOn,
            BlinkersStatus::BLINKERS_STATUS_LEFT_OFF_RIGHT_ON => Self::RightOn,
            BlinkersStatus::BLINKERS_STATUS_LEFT_ON_RIGHT_OFF => Self::LeftOn,
            BlinkersStatus::BLINKERS_STATUS_LEFT_OFF_RIGHT_OFF => Self::AllOff,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_lights::BlinkersStatus> for OnOffState {
    fn from(value: oem::sdv_adas_ego_ego_lights::BlinkersStatus) -> Self {
        use oem::sdv_adas_ego_ego_lights::BlinkersStatus;
        match value {
            BlinkersStatus::BLINKERS_STATUS_LEFT_ON_RIGHT_ON => Self::AllOn,
            BlinkersStatus::BLINKERS_STATUS_LEFT_OFF_RIGHT_ON => Self::RightOn,
            BlinkersStatus::BLINKERS_STATUS_LEFT_ON_RIGHT_OFF => Self::LeftOn,
            BlinkersStatus::BLINKERS_STATUS_LEFT_OFF_RIGHT_OFF => Self::AllOff,
        }
    }
}

#[cfg(test)]
mod test {
    use acc_interface::datatypes::{BlinkersStatus, FlashingIndicatorStatus, TurnSignalCmdSte};

    use super::{OnOffState, TurnSignalCmdState};

    #[test]
    fn test_from_turn_signal_cmd_ste() {
        let mut turn_signal_cmd_ste = TurnSignalCmdSte::TSCS_LEFT;
        let turn_signal_cmd_state: TurnSignalCmdState = turn_signal_cmd_ste.into();
        assert_eq!(turn_signal_cmd_state, TurnSignalCmdState::Left);
        turn_signal_cmd_ste = TurnSignalCmdSte::TSCS_RIGHT;
        let turn_signal_cmd_state: TurnSignalCmdState = turn_signal_cmd_ste.into();
        assert_eq!(turn_signal_cmd_state, TurnSignalCmdState::Right);
        turn_signal_cmd_ste = TurnSignalCmdSte::TSCS_OFF;
        let turn_signal_cmd_state: TurnSignalCmdState = turn_signal_cmd_ste.into();
        assert_eq!(turn_signal_cmd_state, TurnSignalCmdState::Off);
    }
    #[test]
    fn test_from_flashing_indicator_status() {
        let mut flashing_indicator_status = FlashingIndicatorStatus::FIS_LEFT_ON_RIGHT_ON;
        let on_off_state: OnOffState = flashing_indicator_status.into();
        assert_eq!(on_off_state, OnOffState::AllOn);
        flashing_indicator_status = FlashingIndicatorStatus::FIS_LEFT_OFF_RIGHT_ON;
        let on_off_state: OnOffState = flashing_indicator_status.into();
        assert_eq!(on_off_state, OnOffState::RightOn);
        flashing_indicator_status = FlashingIndicatorStatus::FIS_LEFT_ON_RIGHT_OFF;
        let on_off_state: OnOffState = flashing_indicator_status.into();
        assert_eq!(on_off_state, OnOffState::LeftOn);
        flashing_indicator_status =
            FlashingIndicatorStatus::FIS_LEFT_AND_RIGHT_FLASHING_INDICATORS_OFF;
        let on_off_state: OnOffState = flashing_indicator_status.into();
        assert_eq!(on_off_state, OnOffState::AllOff);
        flashing_indicator_status = FlashingIndicatorStatus::FIS_NOT_AVAILABLE;
        let on_off_state: OnOffState = flashing_indicator_status.into();
        assert_eq!(on_off_state, OnOffState::Unknown);
    }
    #[test]
    fn test_from_blinker_status() {
        let mut blinker_status = BlinkersStatus::BLINKERS_STATUS_LEFT_ON_RIGHT_ON;
        let on_off_state: OnOffState = blinker_status.into();
        assert_eq!(on_off_state, OnOffState::AllOn);
        blinker_status = BlinkersStatus::BLINKERS_STATUS_LEFT_OFF_RIGHT_ON;
        let on_off_state: OnOffState = blinker_status.into();
        assert_eq!(on_off_state, OnOffState::RightOn);
        blinker_status = BlinkersStatus::BLINKERS_STATUS_LEFT_ON_RIGHT_OFF;
        let on_off_state: OnOffState = blinker_status.into();
        assert_eq!(on_off_state, OnOffState::LeftOn);
        blinker_status = BlinkersStatus::BLINKERS_STATUS_LEFT_OFF_RIGHT_OFF;
        let on_off_state: OnOffState = blinker_status.into();
        assert_eq!(on_off_state, OnOffState::AllOff);
    }
}
