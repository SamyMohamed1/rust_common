//! Define ego Engine Status for ACC
//!
use acc_interface::datatypes::{engine_t, EnginePerformanceStatus, PwtStatus};
#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;

#[derive(Debug, Clone, Default)]
/// Engine Status
pub struct Engine {
    /// Engine performance status
    pub performance_status: EnginePerfStatus,
    /// Traction mode
    pub power_train_status: PWTState,
    /// Driver override
    pub driver_override: bool,
    /// Estimated PWT wheel torque
    pub pwt_wheel_torque: f32,
}

/// PWT Status
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum PWTState {
    #[default]
    /// No Display | Not Available
    Unknown,
    /// Zev Mode Activated
    Stopped,
    /// Automatique Mode activated
    Starting,
    /// Battery In charge
    Running,
}

impl From<PwtStatus> for PWTState {
    fn from(value: PwtStatus) -> Self {
        match value {
            PwtStatus::PWT_STATUS_STOPPED => Self::Stopped,
            PwtStatus::PWT_STATUS_STARTING => Self::Starting,
            PwtStatus::PWT_STATUS_RUNNING => Self::Running,
            _ => Self::Unknown,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_engine::PwtStatus> for PWTState {
    fn from(value: oem::sdv_adas_ego_ego_engine::PwtStatus) -> Self {
        use oem::sdv_adas_ego_ego_engine::PwtStatus;
        match value {
            PwtStatus::PWT_STATUS_STOPPED => Self::Stopped,
            PwtStatus::PWT_STATUS_STARTING => Self::Starting,
            PwtStatus::PWT_STATUS_RUNNING => Self::Running,
            _ => Self::Unknown,
        }
    }
}

/// Engine Performance Status
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum EnginePerfStatus {
    #[default]
    /// Inhibit Request
    InhibitReq,
    /// Force Req Not Available
    ForceReqNotAvailable,
    /// Force Req Applied
    ForceReqApplied,
    /// Downgraded
    Downgraded,
}
impl From<EnginePerformanceStatus> for EnginePerfStatus {
    fn from(value: EnginePerformanceStatus) -> Self {
        match value {
            EnginePerformanceStatus::EPS_ACC_FORCE_REQUEST_APPLIED => Self::ForceReqApplied,
            EnginePerformanceStatus::EPS_DOWNGRADED_ENGINE_PERFORMANCE => Self::Downgraded,
            EnginePerformanceStatus::EPS_INHIBIT_REQUEST => Self::InhibitReq,
            EnginePerformanceStatus::EPS_NO_FORCE_REQUESTED_NOT_AVAILABLE => {
                Self::ForceReqNotAvailable
            }
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_engine::EnginePerformanceStatus> for EnginePerfStatus {
    fn from(value: oem::sdv_adas_ego_ego_engine::EnginePerformanceStatus) -> Self {
        use oem::sdv_adas_ego_ego_engine::EnginePerformanceStatus;
        match value {
            EnginePerformanceStatus::EPS_ACC_FORCE_REQUEST_APPLIED => Self::ForceReqApplied,
            EnginePerformanceStatus::EPS_DOWNGRADED_ENGINE_PERFORMANCE => Self::Downgraded,
            EnginePerformanceStatus::EPS_INHIBIT_REQUEST => Self::InhibitReq,
            EnginePerformanceStatus::EPS_NO_FORCE_REQUESTED_NOT_AVAILABLE => {
                Self::ForceReqNotAvailable
            }
        }
    }
}

impl From<&engine_t> for Engine {
    fn from(value: &engine_t) -> Self {
        Self {
            performance_status: value.enginePerformanceStatus.into(),
            driver_override: value.driverOverride,
            power_train_status: value.powerTrainStatus.into(),
            pwt_wheel_torque: value.ecmEstimatedPwtWhlTq,
        }
    }
}

#[cfg(feature = "caros")]
impl From<&oem::sdv_adas_ego_ego_engine::Engine_t> for Engine {
    fn from(value: &oem::sdv_adas_ego_ego_engine::Engine_t) -> Self {
        Self {
            performance_status: value
                .engine_performance_status
                .enum_value()
                .unwrap_or_default()
                .into(),
            driver_override: value.driver_override,
            power_train_status: value
                .power_train_status
                .enum_value()
                .unwrap_or_default()
                .into(),
            pwt_wheel_torque: value.ecm_estimated_pwt_whl_tq,
        }
    }
}

#[cfg(test)]
mod test {
    use acc_interface::datatypes::{engine_t, EnginePerformanceStatus, PwtStatus};

    use super::{Engine, EnginePerfStatus, PWTState};

    #[test]
    fn test_fom_pwt_status() {
        let mut pwt_status = PwtStatus::PWT_STATUS_STOPPED;
        let pwt_state: PWTState = pwt_status.into();
        assert_eq!(pwt_state, PWTState::Stopped);
        pwt_status = PwtStatus::PWT_STATUS_STARTING;
        let pwt_state: PWTState = pwt_status.into();
        assert_eq!(pwt_state, PWTState::Starting);
        pwt_status = PwtStatus::PWT_STATUS_RUNNING;
        let pwt_state: PWTState = pwt_status.into();
        assert_eq!(pwt_state, PWTState::Running);
        pwt_status = PwtStatus::PWT_STATUS_UNAVAILABLE;
        let pwt_state: PWTState = pwt_status.into();
        assert_eq!(pwt_state, PWTState::Unknown);
    }

    #[test]
    fn test_from_engine_performance_status() {
        let mut engine_performance_status = EnginePerformanceStatus::EPS_ACC_FORCE_REQUEST_APPLIED;
        let engine_perf_status: EnginePerfStatus = engine_performance_status.into();
        assert_eq!(engine_perf_status, EnginePerfStatus::ForceReqApplied);
        engine_performance_status = EnginePerformanceStatus::EPS_DOWNGRADED_ENGINE_PERFORMANCE;
        let engine_perf_status: EnginePerfStatus = engine_performance_status.into();
        assert_eq!(engine_perf_status, EnginePerfStatus::Downgraded);
        engine_performance_status = EnginePerformanceStatus::EPS_INHIBIT_REQUEST;
        let engine_perf_status: EnginePerfStatus = engine_performance_status.into();
        assert_eq!(engine_perf_status, EnginePerfStatus::InhibitReq);
        engine_performance_status = EnginePerformanceStatus::EPS_NO_FORCE_REQUESTED_NOT_AVAILABLE;
        let engine_perf_status: EnginePerfStatus = engine_performance_status.into();
        assert_eq!(engine_perf_status, EnginePerfStatus::ForceReqNotAvailable);
    }
    #[test]
    fn test_from_engine_t() {
        let engine_t = engine_t {
            enginePerformanceStatus: EnginePerformanceStatus::EPS_INHIBIT_REQUEST,
            driverOverride: false,
            powerTrainStatus: PwtStatus::PWT_STATUS_STOPPED,
            ..Default::default()
        };
        let engine: Engine = (&engine_t).into();
        assert_eq!(engine.performance_status, EnginePerfStatus::InhibitReq);
        assert!(!engine.driver_override);
        assert_eq!(engine.power_train_status, PWTState::Stopped);
    }
}
