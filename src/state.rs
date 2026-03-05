//! Implement ACC state
//!
//! According to the SWEET200 [ACC status](https://comet.tls.renault.fr/architectures/C1A_HS_2023_T4/signals/ACC_Status)
//!
//! ![State](../../docs/assests/state.png)
//!

use acc_interface::datatypes::{CancelAppReasonT, OverrideType, Overriding};

use crate::{
    datatypes::ApplicationState,
    io::{acc_status::AccStatus, failure::FailType, hmi::ActivationRequest, mps, Velocity},
};
#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;

/// ACC main state
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum State {
    /// ACC is either in Temporary or Permanant failure
    Failure(FailType),
    /// ACC is OFF
    #[default]
    Off,
    /// ACC is ON
    On(OnState),
}

impl State {
    /// check whether ACC is on regulation
    pub fn is_on_regulation(&self) -> bool {
        matches!(
            self,
            State::On(OnState::Operational(OperationalState::Regulation, ..))
        )
    }
    /// check whether ACC is on regulation
    pub fn is_on_take_off(&self) -> bool {
        matches!(
            self,
            State::On(OnState::Operational(OperationalState::Regulation, _, true))
        )
    }
    /// check whether ACC is on stop
    pub fn is_on_stop(&self) -> bool {
        matches!(
            self,
            State::On(OnState::Operational(OperationalState::Stop, ..))
        )
    }
    /// check wheter ACC is activated
    pub fn is_activated(&self) -> bool {
        matches!(self, State::On(_))
    }
    /// check whether ACC is waiting
    pub fn is_waiting(&self) -> bool {
        matches!(self, State::On(OnState::StandBy(StandByState::Waiting)))
    }
    /// check whether ACC is override
    pub fn is_override(&self) -> bool {
        matches!(
            self,
            State::On(OnState::Operational(OperationalState::Override, ..))
        )
    }
    /// check wether ACC is Operational
    pub fn is_operational(&self) -> bool {
        matches!(self, State::On(OnState::Operational(..)))
    }
    /// Check "Active conditions"
    /// Returns `true` if the current state is Override, Stop, Waiting or Regulation, otherwise `false`
    pub fn active_conditions(&self) -> bool {
        self.is_override()
            || self.is_on_regulation()
            || self.is_on_stop()
            || self.is_suspended()
            || self.is_waiting()
    }
    /// check if ACC is suspended
    pub fn is_suspended(&self) -> bool {
        matches!(self, State::On(OnState::StandBy(StandByState::Suspended)))
    }
    /// check if ACC is failure
    pub fn is_failure(&self) -> bool {
        matches!(self, State::Failure(_))
    }
    /// check if ACC is off
    pub fn is_off(&self) -> bool {
        matches!(self, State::Off)
    }
}

/// ACC is ON possible states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OnState {
    /// ACC is ON and in Stand by mode
    StandBy(StandByState),
    /// ACC is Operational and Warning is send
    Operational(OperationalState, Velocity, bool),
}

impl OnState {
    /// Construct an Operational state
    pub fn operational(state: OperationalState, speed: f32) -> Self {
        Self::Operational(state, Velocity::new::<mps>(speed), false)
    }
}

/// ACC Stand By state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StandByState {
    /// ACC in take off
    TakeOff,
    /// ACC waiting
    Waiting,
    /// ACC suspended
    Suspended,
}

/// ACC Operation state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperationalState {
    /// ACC in brake only mode
    BrakeOnlyMode,
    /// ACC in regulation
    Regulation,
    /// ACC driver override
    Override,
    /// ACC in stop
    Stop,
}

// Req: SCRS-1398960_2
impl From<State> for ApplicationState {
    fn from(value: State) -> Self {
        use ApplicationState::*;
        match value {
            State::Off => AS_OFF,
            State::Failure(_) => AS_FAILSAFE,
            State::On(OnState::StandBy(StandByState::Waiting)) => AS_INITIALIZATION,
            State::On(OnState::StandBy(_)) => AS_SUSPENDED,
            State::On(OnState::Operational(..)) => AS_OPERATIONAL,
        }
    }
}

impl TryFrom<(ActivationRequest, FailType)> for State {
    type Error = ();
    fn try_from(value: (ActivationRequest, FailType)) -> Result<Self, Self::Error> {
        use ActivationRequest::*;
        match value.0 {
            Off => Ok(Self::Off),
            Init => Err(()),
            On => Err(()),
            Resume => Err(()),
            Fail => Err(()),
        }
    }
}

impl From<State> for AccStatus {
    fn from(value: State) -> Self {
        match value {
            State::Off => Self::Off,
            State::Failure(_) => Self::Failure,
            State::On(OnState::Operational(OperationalState::Override, ..)) => Self::DriverOverride,
            State::On(OnState::Operational(OperationalState::Stop, ..)) => Self::Stop,
            State::On(OnState::Operational(..)) => Self::Regulation,
            State::On(OnState::StandBy(StandByState::Waiting)) => Self::Waiting,
            State::On(OnState::StandBy(StandByState::TakeOff)) => Self::TakeOff,
            State::On(OnState::StandBy(StandByState::Suspended)) => Self::Suspended,
        }
    }
}

impl From<&State> for Overriding {
    fn from(value: &State) -> Self {
        if value.is_override() {
            Overriding::OVERRIDING_SPEED_OVERRIDE
        } else {
            Overriding::OVERRIDING_NO_OVERRIDE
        }
    }
}

// Req: SCRS-2477501_2
impl From<&State> for OverrideType {
    fn from(value: &State) -> Self {
        if value.is_override() {
            Self::OT_SPEED_VOLUNTARY_OVERRIDE
        } else {
            Self::OT_NO_OVERRIDE
        }
    }
}

impl From<&State> for CancelAppReasonT {
    fn from(value: &State) -> Self {
        match value {
            State::Off => Self::CART_OFF,
            State::Failure(FailType::Permanent) | State::Failure(FailType::Temporary) => {
                Self::CART_FAIL
            }
            State::On(OnState::StandBy(StandByState::Suspended))
            | State::On(OnState::StandBy(StandByState::Waiting)) => Self::CART_STANDBY,
            _ => Self::CART_UNSPECIFIED,
        }
    }
}

#[cfg(feature = "caros")]
impl From<&State> for oem::sdv_adas_maneuver_request::CancelAppReasonT {
    fn from(value: &State) -> Self {
        match value {
            State::Off => Self::CART_OFF,
            State::Failure(FailType::Permanent) | State::Failure(FailType::Temporary) => {
                Self::CART_FAIL
            }
            State::On(OnState::StandBy(StandByState::Suspended))
            | State::On(OnState::StandBy(StandByState::Waiting)) => Self::CART_STANDBY,
            _ => Self::CART_UNSPECIFIED,
        }
    }
}

#[cfg(test)]
mod test {

    use crate::state::FailType;
    use crate::{
        datatypes::ApplicationState,
        io::{acc_status::AccStatus, hmi::ActivationRequest, kph, mps, Velocity},
    };

    use super::{OnState, OperationalState, StandByState, State};

    #[test]
    fn test_is_on_regulation() {
        let mut state = State::Off;
        assert!(!state.is_on_regulation());
        state = State::On(OnState::Operational(
            OperationalState::Regulation,
            Velocity::new::<kph>(154.0),
            false,
        ));
        assert!(state.is_on_regulation());
    }
    #[test]
    fn test_is_activated() {
        let mut state = State::Off;
        assert!(!state.is_activated());
        state = State::On(OnState::StandBy(StandByState::Suspended));
        assert!(state.is_activated());
    }
    #[test]
    fn test_from_application_state_to_state() {
        let state: State = State::Off;
        let application_state: ApplicationState = state.into();
        assert_eq!(application_state, ApplicationState::AS_OFF);
        let state: State = State::Failure(FailType::Permanent);
        let application_state: ApplicationState = state.into();
        assert_eq!(application_state, ApplicationState::AS_FAILSAFE);
        let state: State = State::On(OnState::StandBy(StandByState::Waiting));
        let application_state: ApplicationState = state.into();
        assert_eq!(application_state, ApplicationState::AS_INITIALIZATION);
        let state: State = State::On(OnState::StandBy(StandByState::TakeOff));
        let application_state: ApplicationState = state.into();
        assert_eq!(application_state, ApplicationState::AS_SUSPENDED);
        let state: State = State::On(OnState::StandBy(StandByState::Suspended));
        let application_state: ApplicationState = state.into();
        assert_eq!(application_state, ApplicationState::AS_SUSPENDED);
        let state: State = State::On(OnState::Operational(
            OperationalState::Stop,
            Velocity::new::<mps>(1.0),
            false,
        ));
        let application_state: ApplicationState = state.into();
        assert_eq!(application_state, ApplicationState::AS_OPERATIONAL);
        let state: State = State::On(OnState::Operational(
            OperationalState::Override,
            Velocity::new::<mps>(1.0),
            false,
        ));
        let application_state: ApplicationState = state.into();
        assert_eq!(application_state, ApplicationState::AS_OPERATIONAL);
        let state: State = State::On(OnState::Operational(
            OperationalState::BrakeOnlyMode,
            Velocity::new::<mps>(1.0),
            false,
        ));
        let application_state: ApplicationState = state.into();
        assert_eq!(application_state, ApplicationState::AS_OPERATIONAL);
    }
    #[test]
    fn test_try_from_activation_request() {
        let activation_request = ActivationRequest::Off;
        let failure = FailType::NoFailure;
        let state: Result<State, ()> = (activation_request, failure).try_into();
        assert_eq!(state, Ok(State::Off));
        let activation_request = ActivationRequest::Init;
        let state: Result<State, ()> = (activation_request, failure).try_into();
        assert_eq!(state, Err(()));
        let activation_request = ActivationRequest::On;
        let state: Result<State, ()> = (activation_request, failure).try_into();
        assert_eq!(state, Err(()));
        let activation_request = ActivationRequest::Resume;
        let state: Result<State, ()> = (activation_request, failure).try_into();
        assert!(state.is_err());
        let activation_request = ActivationRequest::Fail;
        let state: Result<State, ()> = (activation_request, failure).try_into();
        assert_eq!(state, Err(()));
    }
    #[test]
    fn test_from_state_to_acc_status() {
        let state = State::Off;
        let acc_status: AccStatus = state.into();
        assert_eq!(acc_status, AccStatus::Off);
        let state = State::Failure(FailType::Permanent);
        let acc_status: AccStatus = state.into();
        assert_eq!(acc_status, AccStatus::Failure);
        let state = State::On(OnState::Operational(
            OperationalState::BrakeOnlyMode,
            Velocity::new::<mps>(1.0),
            false,
        ));
        let acc_status: AccStatus = state.into();
        assert_eq!(acc_status, AccStatus::Regulation);
        let state = State::On(OnState::StandBy(StandByState::Waiting));
        let acc_status: AccStatus = state.into();
        assert_eq!(acc_status, AccStatus::Waiting);
        let state = State::On(OnState::StandBy(StandByState::TakeOff));
        let acc_status: AccStatus = state.into();
        assert_eq!(acc_status, AccStatus::TakeOff);
        let state = State::On(OnState::Operational(
            OperationalState::Stop,
            Velocity::new::<mps>(1.0),
            false,
        ));
        let acc_status: AccStatus = state.into();
        assert_eq!(acc_status, AccStatus::Stop);
        let state = State::On(OnState::StandBy(StandByState::Suspended));
        let acc_status: AccStatus = state.into();
        assert_eq!(acc_status, AccStatus::Suspended);
        let state = State::On(OnState::Operational(
            OperationalState::Override,
            Velocity::new::<mps>(1.0),
            false,
        ));
        let acc_status: AccStatus = state.into();
        assert_eq!(acc_status, AccStatus::DriverOverride);
    }
}
