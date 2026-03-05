//! Define HMI I/O for ACC Application
//!
//! ![HMI](../../../docs/assests/hmi.png)

use crate::values::on_change::OnChange;

use super::{mps, Velocity};
use acc_interface::datatypes::{
    ApplicationState, CruiseSettingRequest_t, DistanceSetting, DriverActionRequest_t,
    DriverSelection, FeatureID, FollowTimeLevel, SpeedControlSettingRequest_t,
};
#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;
use num_derive::{FromPrimitive, ToPrimitive};

/// ACC activation request from HMI
/// <table><thead><tr><th>Application State</th><th>Description</th></tr></thead>
/// <tbody><tr><td>Off</td><td> ACC is in a stage where an initialization phase is needed before it 'runs'</td></tr><tr>
/// <td>Init</td>
/// <td>The application is in a phase of initialization. Only after this phase, it can go to Operational, Resume or Failsafe mode</td></tr>
/// <tr><td>On</td>
/// <td>The application is running, meaning for the ACC that the speed of the vehicle is set according to driver request or it adjust its speed to the speed of the vehicle in front of ego vehicle</td></tr>
/// <tr>
/// <td>Resume</td>
/// <td>The application is in a mode where it is not applying any braking or acceleration, but can be set to operational without any initialization phase</td>
/// </tr>
/// <tr>
/// <td>Fail</td>
/// <td>The application is in failsafe mode/degraded mode due to some errors provided by  any error detection</td>
/// </tr>
/// </tbody>
/// </table>
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ActivationRequest {
    /// Initialization state
    Init,
    /// Off request
    #[default]
    Off,
    /// On request
    On,
    /// Resume request
    Resume,
    /// Fail safe request
    Fail,
}

/// SetTimeOfColision permits to the driver to determine how near his/her vehicle remains away from the vehicle in front of the ego car.
/// The shorter the distance is, the nearer the vehicle will stand.
#[derive(Debug, Clone, Copy, Default, PartialEq, ToPrimitive, FromPrimitive, PartialOrd)]
pub enum DistanceControl {
    /// Closest distance Type to the Target
    Short = 0,
    /// Middle distance Type to the Target
    Middle = 1,
    /// Reglementary distance Type to the Target
    #[default]
    Reglementary = 2,
    /// Long distance Type to the Target
    Long = 3,
    /// not defined
    Undefined = 4,
}

impl From<DistanceControl> for FollowTimeLevel {
    fn from(value: DistanceControl) -> Self {
        match value {
            DistanceControl::Short => Self::FTL_SHORT,
            DistanceControl::Middle => Self::FTL_MIDDLE,
            DistanceControl::Reglementary => Self::FTL_REGLEMENTARY,
            DistanceControl::Long => Self::FTL_LONG,
            DistanceControl::Undefined => Self::FTL_UNDEFINED,
        }
    }
}

#[cfg(feature = "caros")]
impl From<DistanceControl> for oem::sdv_adas_maneuver_request::FollowTimeLevel {
    fn from(value: DistanceControl) -> Self {
        match value {
            DistanceControl::Short => Self::FTL_SHORT,
            DistanceControl::Middle => Self::FTL_MIDDLE,
            DistanceControl::Reglementary => Self::FTL_REGLEMENTARY,
            DistanceControl::Long => Self::FTL_LONG,
            DistanceControl::Undefined => Self::FTL_UNDEFINED,
        }
    }
}

/// Interdistance Acc Activation Request
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum InterdistanceReq {
    /// Interdistance is Actif
    On,
    /// Interdistance is Off
    #[default]
    Off,
}

impl From<DriverSelection> for InterdistanceReq {
    fn from(value: DriverSelection) -> Self {
        match value {
            DriverSelection::DS_ACTIVATION => Self::On,
            DriverSelection::DS_DEACTIVATION | DriverSelection::DS_UNSPECIFIED => Self::Off,
        }
    }
}

#[cfg(feature = "caros")]
impl From<catalog_ampere_adas::sdv_adas_hmi_manager_types::DriverSelection> for InterdistanceReq {
    fn from(value: catalog_ampere_adas::sdv_adas_hmi_manager_types::DriverSelection) -> Self {
        use catalog_ampere_adas::sdv_adas_hmi_manager_types::DriverSelection::*;
        match value {
            DS_ACTIVATION => Self::On,
            DS_DEACTIVATION | DS_UNSPECIFIED => Self::Off,
        }
    }
}

/// HMI Settings
#[derive(Debug, Clone, Default)]
pub struct Hmi {
    /// ACC state request
    pub state: ActivationRequest,
    /// ACC Distance settings request
    pub distance_control: OnChange<DistanceControl>,
    /// Interdistance Request
    pub iacc_req: InterdistanceReq,
    /// ACC Set speed control request
    pub speed_control: OnChange<Velocity>,
    /// Contextual button
    pub contextual_btn: ContextualBtnState,
    /// Driver action: set speed or activate/deactivate
    pub driver_action: bool,
}

impl Hmi {
    /// Requested set speed by HMI
    #[inline(always)]
    pub fn get_set_speed(&self) -> Velocity {
        *self.speed_control
    }
    /// Requested set speed by HMI
    #[inline(always)]
    pub fn get_set_speed_mps(&self) -> f32 {
        (*self.speed_control).get::<mps>()
    }
    /// check if set_speed received
    #[inline(always)]
    pub fn has_vset(&self) -> bool {
        self.speed_control.is_set()
    }
    /// check if Iacc Is activated
    #[inline(always)]
    pub fn iacc_req_on(&self) -> bool {
        self.iacc_req.eq(&InterdistanceReq::On)
    }
    /// Acc is Off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        matches!(self.state, ActivationRequest::Off)
    }
    /// True if there is a new activation request
    #[inline(always)]
    pub fn new_activation_req(&self) -> bool {
        matches!(self.state, ActivationRequest::On | ActivationRequest::Init) && self.driver_action
    }
}

impl From<ApplicationState> for ActivationRequest {
    fn from(value: ApplicationState) -> Self {
        match value {
            ApplicationState::AS_INITIALIZATION => Self::Init,
            ApplicationState::AS_OFF => Self::Off,
            ApplicationState::AS_OPERATIONAL => Self::On,
            ApplicationState::AS_SUSPENDED => Self::Resume,
            ApplicationState::AS_FAILSAFE => Self::Fail,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_hmi_manager_types::ApplicationState> for ActivationRequest {
    fn from(value: oem::sdv_adas_hmi_manager_types::ApplicationState) -> Self {
        use oem::sdv_adas_hmi_manager_types::ApplicationState;
        match value {
            ApplicationState::AS_INITIALIZATION => Self::Init,
            ApplicationState::AS_OFF => Self::Off,
            ApplicationState::AS_OPERATIONAL => Self::On,
            ApplicationState::AS_SUSPENDED => Self::Resume,
            ApplicationState::AS_FAILSAFE => Self::Fail,
        }
    }
}

impl From<DistanceSetting> for DistanceControl {
    fn from(value: DistanceSetting) -> Self {
        match value {
            DistanceSetting::DS_SHORT => Self::Short,
            DistanceSetting::DS_MIDDLE => Self::Middle,
            DistanceSetting::DS_REGLEMENTARY => Self::Reglementary,
            DistanceSetting::DS_LONG => Self::Long,
            DistanceSetting::DS_NO_DISPLAY => Self::Undefined, // NO_DISPLAY is only used at HMI side, normally acc will never receive this value
        }
    }
}
#[cfg(feature = "caros")]
impl From<oem::sdv_adas_hmi_manager_types::DistanceSetting> for DistanceControl {
    fn from(value: oem::sdv_adas_hmi_manager_types::DistanceSetting) -> Self {
        use oem::sdv_adas_hmi_manager_types::DistanceSetting;
        match value {
            DistanceSetting::DS_SHORT => Self::Short,
            DistanceSetting::DS_MIDDLE => Self::Middle,
            DistanceSetting::DS_REGLEMENTARY => Self::Reglementary,
            DistanceSetting::DS_LONG => Self::Long,
            DistanceSetting::DS_NO_DISPLAY => Self::Undefined,
        }
    }
}

impl From<&DistanceControl> for DistanceSetting {
    fn from(value: &DistanceControl) -> Self {
        match value {
            DistanceControl::Short => Self::DS_SHORT,
            DistanceControl::Middle => Self::DS_MIDDLE,
            DistanceControl::Reglementary => Self::DS_REGLEMENTARY,
            DistanceControl::Long => Self::DS_LONG,
            DistanceControl::Undefined => Self::DS_NO_DISPLAY,
        }
    }
}

#[cfg(feature = "caros")]
impl From<&DistanceControl> for oem::sdv_adas_hmi_manager_types::DistanceSetting {
    fn from(value: &DistanceControl) -> Self {
        match value {
            DistanceControl::Short => Self::DS_SHORT,
            DistanceControl::Middle => Self::DS_MIDDLE,
            DistanceControl::Reglementary => Self::DS_REGLEMENTARY,
            DistanceControl::Long => Self::DS_LONG,
            DistanceControl::Undefined => Self::DS_NO_DISPLAY,
        }
    }
}

/// Wrapper for Cruise Setting Request
#[derive(Debug, Default, PartialEq)]
pub struct CruiseWrapper(pub DistanceControl, pub InterdistanceReq);
// TODO: Check if we need the IVI Offsets information
impl TryFrom<&CruiseSettingRequest_t> for CruiseWrapper {
    type Error = super::error::IoError;
    fn try_from(value: &CruiseSettingRequest_t) -> Result<Self, Self::Error> {
        if !matches!(value.FeatID, FeatureID::FID_ACC) {
            return Err(super::error::IoError::InvalidFeatId);
        }
        Ok(Self(
            value.SetTimeOfCollision.into(),
            value.AccIntelligentDistanceActivation.into(),
        ))
    }
}
#[cfg(feature = "caros")]
impl TryFrom<&oem::sdv_adas_hmi_manager_types::CruiseSettingRequest_t> for CruiseWrapper {
    type Error = super::error::IoError;
    fn try_from(
        value: &oem::sdv_adas_hmi_manager_types::CruiseSettingRequest_t,
    ) -> Result<Self, Self::Error> {
        use oem::sdv_adas_hmi_manager_types::FeatureID;
        match value.feat_id.enum_value() {
            Ok(FeatureID::FID_ACC) => {}
            Ok(_) => return Err(super::error::IoError::InvalidFeatId),
            Err(_) => return Err(super::error::IoError::InvalidProtoEnum),
        }
        let Ok(time_of_collision) = value.set_time_of_collision.enum_value() else {
            return Err(super::error::IoError::InvalidProtoEnum);
        };
        let Ok(iacc_dist_act) = value.acc_intelligent_distance_activation.enum_value() else {
            return Err(super::error::IoError::InvalidProtoEnum);
        };
        Ok(Self(time_of_collision.into(), iacc_dist_act.into()))
    }
}
#[derive(Default, Clone, Debug, PartialEq)]
/// Contextual btn state
pub enum ContextualBtnState {
    /// Contextual btn pressed
    On,
    #[default]
    /// Btn Not pressed
    Off,
}
impl From<DriverActionRequest_t> for ContextualBtnState {
    fn from(value: DriverActionRequest_t) -> Self {
        use ContextualBtnState::*;
        use DriverActionRequest_t::*;
        match value {
            DAIT_UNSPECIFIED | DAIT_CANCEL => Off,
            DAIT_HOLD_ON => On,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_hmi_manager_types::DriverActionRequest_t> for ContextualBtnState {
    fn from(value: oem::sdv_adas_hmi_manager_types::DriverActionRequest_t) -> Self {
        use oem::sdv_adas_hmi_manager_types::DriverActionRequest_t::*;
        use ContextualBtnState::*;
        match value {
            DAIT_UNSPECIFIED | DAIT_CANCEL => Off,
            DAIT_HOLD_ON => On,
        }
    }
}

/// Wrapper for Hmi speed control request
#[derive(PartialEq, Debug, Clone, Default)]
pub struct SpeedRequest {
    /// hmi set speed
    pub speed: Velocity,
    /// Contexttual btn state
    pub contextual_btn: ContextualBtnState,
}

impl TryFrom<&SpeedControlSettingRequest_t> for SpeedRequest {
    type Error = super::error::IoError;
    fn try_from(value: &SpeedControlSettingRequest_t) -> Result<Self, Self::Error> {
        if !matches!(value.FeatID, FeatureID::FID_ACC) {
            return Err(super::error::IoError::InvalidFeatId);
        }
        Ok(SpeedRequest {
            speed: Velocity::new::<mps>(value.SetVehicleSpeed),
            contextual_btn: value.driverActionRequest.into(),
        })
    }
}

#[cfg(feature = "caros")]
impl TryFrom<&oem::sdv_adas_hmi_manager_types::SpeedControlSettingRequest_t> for SpeedRequest {
    type Error = super::error::IoError;
    fn try_from(
        value: &oem::sdv_adas_hmi_manager_types::SpeedControlSettingRequest_t,
    ) -> Result<Self, Self::Error> {
        use oem::sdv_adas_hmi_manager_types::FeatureID;
        let Ok(FeatureID::FID_ACC) = value.feat_id.enum_value() else {
            return Err(super::error::IoError::InvalidFeatId);
        };
        Ok(SpeedRequest {
            speed: Velocity::new::<mps>(value.set_vehicle_speed),
            contextual_btn: value.driver_action_request.enum_value_or_default().into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use acc_interface::datatypes::{
        ApplicationState, CruiseSettingRequest_t, DistanceSetting, FeatureID,
        SpeedControlSettingRequest_t,
    };

    use crate::io::{
        hmi::{CruiseWrapper, SpeedRequest},
        mps, Velocity,
    };

    use super::{ActivationRequest, DistanceControl, Hmi};

    #[test]
    fn test_get_set_speed() {
        let mut hmi = Hmi::default();
        hmi.speed_control.set(Velocity::new::<mps>(2.6));
        assert_eq!(hmi.get_set_speed(), Velocity::new::<mps>(2.6));
        assert_eq!(hmi.get_set_speed_mps(), 2.6);
        assert!(hmi.has_vset());
    }
    #[test]
    fn test_from_application_state_to_activation_request() {
        let app_state = ApplicationState::AS_OPERATIONAL;
        let activ_req: ActivationRequest = app_state.into();
        assert_eq!(activ_req, ActivationRequest::On);
        let app_state = ApplicationState::AS_INITIALIZATION;
        let activ_req: ActivationRequest = app_state.into();
        assert_eq!(activ_req, ActivationRequest::Init);
        let app_state = ApplicationState::AS_OFF;
        let activ_req: ActivationRequest = app_state.into();
        assert_eq!(activ_req, ActivationRequest::Off);
        let app_state = ApplicationState::AS_SUSPENDED;
        let activ_req: ActivationRequest = app_state.into();
        assert_eq!(activ_req, ActivationRequest::Resume);
        let app_state = ApplicationState::AS_FAILSAFE;
        let activ_req: ActivationRequest = app_state.into();
        assert_eq!(activ_req, ActivationRequest::Fail);
    }
    #[test]
    fn test_from_distance_setting_to_distance_control() {
        let distance_setting = DistanceSetting::DS_LONG;
        let distance_control: DistanceControl = distance_setting.into();
        assert_eq!(distance_control, DistanceControl::Long);
        let distance_setting = DistanceSetting::DS_SHORT;
        let distance_control: DistanceControl = distance_setting.into();
        assert_eq!(distance_control, DistanceControl::Short);
        let distance_setting = DistanceSetting::DS_REGLEMENTARY;
        let distance_control: DistanceControl = distance_setting.into();
        assert_eq!(distance_control, DistanceControl::Reglementary);
    }
    #[test]
    fn test_from_distance_control_to_distance_setting() {
        let distance_control = DistanceControl::Long;
        let distance_setting: DistanceSetting = (&distance_control).into();
        assert_eq!(distance_setting, DistanceSetting::DS_LONG);
        let distance_control = DistanceControl::Short;
        let distance_setting: DistanceSetting = (&distance_control).into();
        assert_eq!(distance_setting, DistanceSetting::DS_SHORT);
        let distance_control = DistanceControl::Middle;
        let distance_setting: DistanceSetting = (&distance_control).into();
        assert_eq!(distance_setting, DistanceSetting::DS_MIDDLE);
        let distance_control = DistanceControl::Reglementary;
        let distance_setting: DistanceSetting = (&distance_control).into();
        assert_eq!(distance_setting, DistanceSetting::DS_REGLEMENTARY);
    }
    #[test]
    fn test_from_cruisesettingreq_to_distance_control() {
        let cruise_set_req = CruiseSettingRequest_t {
            FeatID: FeatureID::FID_ACC,
            SetTimeOfCollision: DistanceSetting::DS_MIDDLE,
            ..Default::default()
        };
        let dist_control: CruiseWrapper = (&cruise_set_req).try_into().unwrap();
        assert_eq!(dist_control.0, DistanceControl::Middle);
    }
    #[test]
    fn test_from_cruisesettingreq_to_distance_control_failure() {
        let cruise_set_req = CruiseSettingRequest_t {
            SetTimeOfCollision: DistanceSetting::DS_MIDDLE,
            ..Default::default()
        };
        let dist_control: Result<CruiseWrapper, _> = (&cruise_set_req).try_into();
        assert_eq!(dist_control, Err(crate::io::error::IoError::InvalidFeatId));
    }
    #[test]
    fn test_from_speedctrlsetreq_to_setspeed() {
        let speed_ctrl_set_req = SpeedControlSettingRequest_t {
            FeatID: FeatureID::FID_ACC,
            SetVehicleSpeed: 35.0,
            ..Default::default()
        };
        let set_speed: SpeedRequest = (&speed_ctrl_set_req).try_into().unwrap();
        assert_eq!(set_speed.speed, Velocity::new::<mps>(35.0));
    }
    #[test]
    fn test_from_speedctrlsetreq_to_setspeed_failure() {
        let speed_ctrl_set_req = SpeedControlSettingRequest_t::default();
        let sp_req: Result<SpeedRequest, _> = (&speed_ctrl_set_req).try_into();
        assert_eq!(sp_req, Err(crate::io::error::IoError::InvalidFeatId));
    }
}
