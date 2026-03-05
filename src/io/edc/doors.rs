//! Define ego Doors Input for ACC
//!

use acc_interface::datatypes::{doors_t, DoorState, TailGateStatus};

use crate::io::ego::OnOffState;
#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;

/// Ego Doors
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EgoDoors {
    /// Ego RL door state
    pub rl_door_state: OnOffState,
    /// Ego RR door state
    pub rr_door_state: OnOffState,
    /// Ego front doors (FL/FR)
    pub front_doors_state: OnOffState,
    /// unclosed door status
    pub unclosed_door_mode: OnOffState,
    /// tail gate status
    pub tail_gate_status: TailGateState,
}

/// Ego Doors
#[derive(Debug, Clone, Default, PartialEq)]
pub enum TailGateState {
    #[default]
    /// tailgate unvailable
    Unvailable,
    /// tailgate close
    Close,
    /// tailgate open
    Open,
}
impl From<TailGateStatus> for TailGateState {
    fn from(value: TailGateStatus) -> Self {
        match value {
            TailGateStatus::TGS_NOT_USED | TailGateStatus::TGS_UNAVAILABLE_VALUE_NOT_AVAILABLE => {
                Self::Unvailable
            }
            TailGateStatus::TGS_TAILGATE_IS_CLOSED => Self::Close,
            TailGateStatus::TGS_TAILGATE_IS_OPEN => Self::Open,
        }
    }
}
impl From<&doors_t> for EgoDoors {
    fn from(val: &doors_t) -> Self {
        Self {
            rl_door_state: match val.passengerDoorStateRL {
                DoorState::DOOR_STATE_DOOR_OPEN => OnOffState::On,
                _ => OnOffState::Off,
            },
            rr_door_state: match val.passengerDoorStateRR {
                DoorState::DOOR_STATE_DOOR_OPEN => OnOffState::On,
                _ => OnOffState::Off,
            },
            front_doors_state: if [&val.passengerDoorStateFL, &val.passengerDoorStateFR]
                .into_iter()
                .any(|v| matches!(v, DoorState::DOOR_STATE_DOOR_OPEN))
            {
                OnOffState::On
            } else {
                OnOffState::Off
            },
            unclosed_door_mode: val.unclosedDoorModeState.into(),
            tail_gate_status: val.tailGateStatus.into(),
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_doors_and_belts::TailGateStatus> for TailGateState {
    fn from(value: oem::sdv_adas_ego_ego_doors_and_belts::TailGateStatus) -> Self {
        use oem::sdv_adas_ego_ego_doors_and_belts::TailGateStatus;
        match value {
            TailGateStatus::TGS_NOT_USED | TailGateStatus::TGS_UNAVAILABLE_VALUE_NOT_AVAILABLE => {
                Self::Unvailable
            }
            TailGateStatus::TGS_TAILGATE_IS_CLOSED => Self::Close,
            TailGateStatus::TGS_TAILGATE_IS_OPEN => Self::Open,
        }
    }
}
#[cfg(feature = "caros")]
impl From<&oem::sdv_adas_ego_ego_doors_and_belts::Doors_t> for EgoDoors {
    fn from(val: &oem::sdv_adas_ego_ego_doors_and_belts::Doors_t) -> Self {
        use oem::sdv_adas_ego_ego_doors_and_belts::DoorState;
        Self {
            rl_door_state: match val.passenger_door_state_rl.enum_value_or_default() {
                DoorState::DOOR_STATE_DOOR_OPEN => OnOffState::On,
                _ => OnOffState::Off,
            },
            rr_door_state: match val.passenger_door_state_rr.enum_value_or_default() {
                DoorState::DOOR_STATE_DOOR_OPEN => OnOffState::On,
                _ => OnOffState::Off,
            },
            front_doors_state: if [&val.passenger_door_state_fl, &val.passenger_door_state_fr]
                .into_iter()
                .filter_map(|v| v.enum_value().ok())
                .any(|v| matches!(v, DoorState::DOOR_STATE_DOOR_OPEN))
            {
                OnOffState::On
            } else {
                OnOffState::Off
            },
            unclosed_door_mode: val.unclosed_door_mode_state.into(),
            tail_gate_status: val.tail_gate_status.enum_value_or_default().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tail_gate_state_from_status() {
        let status = TailGateStatus::TGS_NOT_USED;
        let tail_state: TailGateState = status.into();
        assert_eq!(tail_state, TailGateState::Unvailable);

        let status = TailGateStatus::TGS_UNAVAILABLE_VALUE_NOT_AVAILABLE;
        let tail_state: TailGateState = status.into();
        assert_eq!(tail_state, TailGateState::Unvailable);

        let status = TailGateStatus::TGS_TAILGATE_IS_CLOSED;
        let tail_state: TailGateState = status.into();
        assert_eq!(tail_state, TailGateState::Close);

        let status = TailGateStatus::TGS_TAILGATE_IS_OPEN;
        let tail_state: TailGateState = status.into();
        assert_eq!(tail_state, TailGateState::Open);
    }

    #[test]
    fn test_ego_doors_from_all_closed() {
        let doors = doors_t {
            driverDoorState: DoorState::DOOR_STATE_DOOR_CLOSED,
            passengerDoorStateFL: DoorState::DOOR_STATE_DOOR_CLOSED,
            passengerDoorStateFR: DoorState::DOOR_STATE_DOOR_CLOSED,
            passengerDoorStateFront: DoorState::DOOR_STATE_DOOR_CLOSED,
            passengerDoorStateRL: DoorState::DOOR_STATE_DOOR_CLOSED,
            passengerDoorStateRR: DoorState::DOOR_STATE_DOOR_CLOSED,
            unclosedDoorModeState: false,
            tailGateStatus: TailGateStatus::TGS_TAILGATE_IS_CLOSED,
        };
        let ego_doors: EgoDoors = (&doors).into();
        assert_eq!(ego_doors.rl_door_state, OnOffState::Off);
        assert_eq!(ego_doors.rr_door_state, OnOffState::Off);
        assert_eq!(ego_doors.front_doors_state, OnOffState::Off);
        assert_eq!(ego_doors.tail_gate_status, TailGateState::Close);
        assert_eq!(ego_doors.unclosed_door_mode, OnOffState::Off);
    }

    #[test]
    fn test_ego_doors_from_one_open_door() {
        let doors = doors_t {
            driverDoorState: DoorState::DOOR_STATE_DOOR_OPEN,
            passengerDoorStateFL: DoorState::DOOR_STATE_DOOR_OPEN,
            passengerDoorStateFR: DoorState::DOOR_STATE_DOOR_CLOSED,
            passengerDoorStateFront: DoorState::DOOR_STATE_DOOR_CLOSED,
            passengerDoorStateRL: DoorState::DOOR_STATE_DOOR_CLOSED,
            passengerDoorStateRR: DoorState::DOOR_STATE_DOOR_CLOSED,
            unclosedDoorModeState: true,
            tailGateStatus: TailGateStatus::TGS_TAILGATE_IS_OPEN,
        };
        let ego_doors: EgoDoors = (&doors).into();
        assert_eq!(ego_doors.front_doors_state, OnOffState::On);
        assert_eq!(ego_doors.rl_door_state, OnOffState::Off);
        assert_eq!(ego_doors.rr_door_state, OnOffState::Off);
        assert_eq!(ego_doors.tail_gate_status, TailGateState::Open);
        assert_eq!(ego_doors.unclosed_door_mode, OnOffState::On);
    }
}
