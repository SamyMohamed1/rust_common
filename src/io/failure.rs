//! Define FMS (Failure Management System) I/O for ACC Application
//!

use acc_interface::datatypes::{FailureType, FeatureInhibitionRequest};

/// Failure Type from FMS
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FailType {
    /// No failure received
    #[default]
    NoFailure,
    /// Temporary failure received
    Temporary,
    /// Permanent failure received
    Permanent,
    /// Degraded mode 1 received
    Degraded1,
    /// Degraded mode 2 received
    Degraded2,
    /// Silent Inhibition received
    SilentInhibition,
}

impl From<FailureType> for FailType {
    fn from(value: FailureType) -> Self {
        use FailureType::*;
        match value {
            FT_NO_FAILURE => Self::NoFailure,
            FT_TEMPORARY_FAILURE => Self::Temporary,
            FT_PERMANENT_FAILURE => Self::Permanent,
            FT_DEGRADED_MODE1 => Self::Degraded1,
            FT_DEGRADED_MODE2 => Self::Degraded2,
            FT_SILENT_INHIBITION => Self::SilentInhibition,
        }
    }
}

#[cfg(feature = "caros")]
impl From<catalog_ampere_adas::sdv_adas_hmi_manager_types::FailureType> for FailType {
    fn from(value: catalog_ampere_adas::sdv_adas_hmi_manager_types::FailureType) -> Self {
        use catalog_ampere_adas::sdv_adas_hmi_manager_types::FailureType::*;
        match value {
            FT_NO_FAILURE => Self::NoFailure,
            FT_TEMPORARY_FAILURE => Self::Temporary,
            FT_PERMANENT_FAILURE => Self::Permanent,
            FT_DEGRADED_MODE1 => Self::Degraded1,
            FT_DEGRADED_MODE2 => Self::Degraded2,
            FT_SILENT_INHIBITION => Self::SilentInhibition,
        }
    }
}

impl From<FailType> for FailureType {
    fn from(value: FailType) -> Self {
        use FailType::*;
        match value {
            NoFailure => Self::FT_NO_FAILURE,
            Temporary => Self::FT_TEMPORARY_FAILURE,
            Permanent => Self::FT_PERMANENT_FAILURE,
            Degraded1 => Self::FT_DEGRADED_MODE1,
            Degraded2 => Self::FT_DEGRADED_MODE2,
            SilentInhibition => Self::FT_SILENT_INHIBITION,
        }
    }
}

#[cfg(feature = "caros")]
impl From<FailType> for catalog_ampere_adas::sdv_adas_hmi_manager_types::FailureType {
    fn from(value: FailType) -> Self {
        use FailType::*;
        match value {
            NoFailure => Self::FT_NO_FAILURE,
            Temporary => Self::FT_TEMPORARY_FAILURE,
            Permanent => Self::FT_PERMANENT_FAILURE,
            Degraded1 => Self::FT_DEGRADED_MODE1,
            Degraded2 => Self::FT_DEGRADED_MODE2,
            SilentInhibition => Self::FT_SILENT_INHIBITION,
        }
    }
}

/// Failure information
#[derive(Debug, Clone, Default)]
pub struct Failure {
    /// FMS Failure received
    pub failure: FailType,
    /// Previous Failure status
    pub previous_failure: FailType,
}

impl Failure {
    /// Check if we received a failure (Temporary or Permanent)
    pub fn is_failure(&self) -> bool {
        matches!(self.failure, FailType::Temporary | FailType::Permanent)
    }
    /// Check if we receive a degraded mode
    pub fn is_degraded(&self) -> bool {
        matches!(self.failure, FailType::Degraded1)
    }
}

impl From<FeatureInhibitionRequest> for Failure {
    fn from(value: FeatureInhibitionRequest) -> Self {
        Self {
            failure: value.failure.into(),
            previous_failure: FailType::NoFailure,
        }
    }
}

impl From<FailureType> for Failure {
    fn from(value: FailureType) -> Self {
        Self {
            failure: value.into(),
            previous_failure: FailType::NoFailure,
        }
    }
}

#[cfg(feature = "caros")]
impl From<catalog_ampere_adas::sdv_adas_hmi_manager_types::FailureType> for Failure {
    fn from(value: catalog_ampere_adas::sdv_adas_hmi_manager_types::FailureType) -> Self {
        Self {
            failure: value.into(),
            previous_failure: FailType::NoFailure,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::failure::{FailType, Failure};
    use acc_interface::datatypes::FailureType;
    #[test]
    fn test_from_failuretype_to_failtype() {
        let mut failure_type = FailureType::FT_NO_FAILURE;
        let mut fail_type: FailType = failure_type.into();
        assert_eq!(fail_type, FailType::NoFailure);
        failure_type = FailureType::FT_DEGRADED_MODE1;
        fail_type = failure_type.into();
        assert_eq!(fail_type, FailType::Degraded1);
        failure_type = FailureType::FT_DEGRADED_MODE2;
        fail_type = failure_type.into();
        assert_eq!(fail_type, FailType::Degraded2);
        failure_type = FailureType::FT_PERMANENT_FAILURE;
        fail_type = failure_type.into();
        assert_eq!(fail_type, FailType::Permanent);
        failure_type = FailureType::FT_SILENT_INHIBITION;
        fail_type = failure_type.into();
        assert_eq!(fail_type, FailType::SilentInhibition);
        failure_type = FailureType::FT_TEMPORARY_FAILURE;
        fail_type = failure_type.into();
        assert_eq!(fail_type, FailType::Temporary);
    }

    #[test]
    fn test_from_failtype_to_failuretype() {
        let mut fail_type = FailType::NoFailure;
        let mut failure_type: FailureType = fail_type.into();
        assert_eq!(failure_type, FailureType::FT_NO_FAILURE);
        fail_type = FailType::Degraded1;
        failure_type = fail_type.into();
        assert_eq!(failure_type, FailureType::FT_DEGRADED_MODE1);
        fail_type = FailType::Degraded2;
        failure_type = fail_type.into();
        assert_eq!(failure_type, FailureType::FT_DEGRADED_MODE2);
        fail_type = FailType::Permanent;
        failure_type = fail_type.into();
        assert_eq!(failure_type, FailureType::FT_PERMANENT_FAILURE);
        fail_type = FailType::SilentInhibition;
        failure_type = fail_type.into();
        assert_eq!(failure_type, FailureType::FT_SILENT_INHIBITION);
        fail_type = FailType::Temporary;
        failure_type = fail_type.into();
        assert_eq!(failure_type, FailureType::FT_TEMPORARY_FAILURE);
    }

    #[test]
    fn test_is_failure() {
        let mut fms = Failure {
            failure: FailType::Degraded1,
            ..Default::default()
        };
        assert!(!fms.is_failure());
        fms.failure = FailType::Degraded2;
        assert!(!fms.is_failure());
        fms.failure = FailType::NoFailure;
        assert!(!fms.is_failure());
        fms.failure = FailType::Permanent;
        assert!(fms.is_failure());
        fms.failure = FailType::Temporary;
        assert!(fms.is_failure());
    }

    #[test]
    fn test_from_failuretype_to_failure() {
        let failuretype = FailureType::FT_PERMANENT_FAILURE;
        let failure: Failure = failuretype.into();
        assert_eq!(failure.failure, FailType::Permanent);
        assert_eq!(failure.previous_failure, FailType::NoFailure);
    }
}
