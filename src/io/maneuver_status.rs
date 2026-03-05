//! Maneuver Request status received from Path Planner
//!

use acc_interface::datatypes::ManeuverReqStatus;

/// Maneuver Request status feedback
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ManeuverStatus {
    /// Undefined state
    #[default]
    Undefined,
    /// Maneuver accpeted
    Accepted,
    /// ramp down, maneuver is cancelled but delay is added
    Canceled,
    /// Maneuver rejected
    Error(ManeuverIssue),
    /// Maneuver accepted but with erros
    Warning(ManeuverIssue),
}

impl ManeuverStatus {
    /// check if the maneuver status is error
    pub fn is_error(&self) -> bool {
        matches!(self, ManeuverStatus::Error(_))
    }
    /// check if the maneuver status is warning
    pub fn is_warning(&self) -> bool {
        matches!(self, ManeuverStatus::Warning(_))
    }
    /// check if the maneuver status is warning
    pub fn is_speed_warning(&self) -> bool {
        matches!(self, ManeuverStatus::Warning(ManeuverIssue::Speed))
    }
    /// Maneuver Canceled and waiting for it to be done
    pub fn is_waiting(&self) -> bool {
        matches!(self, ManeuverStatus::Canceled)
    }
    /// Maneuver is undefined
    pub fn is_undefined(&self) -> bool {
        matches!(self, ManeuverStatus::Undefined)
    }
}

/// Maneuver Request status errors and warnings
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ManeuverIssue {
    /// Undefined state
    #[default]
    Undefined,
    /// Maneuver rejected
    Rejected,
    /// Unfeasible postion
    Position,
    /// Unfeasible/discrepancy Speed
    Speed,
    /// Unfeasible acceleration
    Acceleration,
    /// Unfeasible jerk
    Jerk,
    /// Potentially moving object not found
    Object,
    /// Road object not found
    Road,
}

impl From<ManeuverReqStatus> for ManeuverStatus {
    fn from(value: ManeuverReqStatus) -> Self {
        match value {
            ManeuverReqStatus::MRS_UNKNOWN => Self::Undefined,
            ManeuverReqStatus::MRS_REJECTED => Self::Error(ManeuverIssue::Rejected),
            ManeuverReqStatus::MRS_ACCEPTED => Self::Accepted,
            ManeuverReqStatus::MRS_ERROR_UNDEFINED => Self::Error(ManeuverIssue::Undefined),
            ManeuverReqStatus::MRS_WARNING_UNFEASIBLE_POS => Self::Warning(ManeuverIssue::Position),
            ManeuverReqStatus::MRS_WARNING_UNFEASIBLE_ACCEL => {
                Self::Warning(ManeuverIssue::Acceleration)
            }
            ManeuverReqStatus::MRS_WARNING_UNFEASIBLE_JERK => Self::Warning(ManeuverIssue::Jerk),
            ManeuverReqStatus::MRS_ERROR_ROAD_OBJECT_NOT_FOUND => Self::Error(ManeuverIssue::Road),
            ManeuverReqStatus::MRS_ERROR_PMO_OBJECT_NOT_FOUND => Self::Error(ManeuverIssue::Object),
            ManeuverReqStatus::MRS_WARNING_ROAD_OBJECT_NOT_FOUND => {
                Self::Warning(ManeuverIssue::Road)
            }
            ManeuverReqStatus::MRS_WARNING_PMO_OBJECT_NOT_FOUND => {
                Self::Warning(ManeuverIssue::Object)
            }
            ManeuverReqStatus::MRS_CUT_OFF_RAMP_DOWN => Self::Canceled,
            ManeuverReqStatus::MRS_ERROR_UNFEASIBLE_SPEED => Self::Error(ManeuverIssue::Speed),
            ManeuverReqStatus::MRS_WARNING_DISCREPANCY_SPEED => Self::Warning(ManeuverIssue::Speed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maneuverreqstatus_conversion() {
        let mut maneuver_req_status: ManeuverReqStatus = ManeuverReqStatus::MRS_UNKNOWN;
        let mut man_status: ManeuverStatus = maneuver_req_status.into();
        assert_eq!(man_status, ManeuverStatus::Undefined);

        maneuver_req_status = ManeuverReqStatus::MRS_REJECTED;
        man_status = maneuver_req_status.into();
        assert_eq!(man_status, ManeuverStatus::Error(ManeuverIssue::Rejected));

        maneuver_req_status = ManeuverReqStatus::MRS_ACCEPTED;
        man_status = maneuver_req_status.into();
        assert_eq!(man_status, ManeuverStatus::Accepted);

        maneuver_req_status = ManeuverReqStatus::MRS_ERROR_UNDEFINED;
        man_status = maneuver_req_status.into();
        assert_eq!(man_status, ManeuverStatus::Error(ManeuverIssue::Undefined));

        maneuver_req_status = ManeuverReqStatus::MRS_WARNING_UNFEASIBLE_POS;
        man_status = maneuver_req_status.into();
        assert_eq!(man_status, ManeuverStatus::Warning(ManeuverIssue::Position));

        maneuver_req_status = ManeuverReqStatus::MRS_ERROR_ROAD_OBJECT_NOT_FOUND;
        man_status = maneuver_req_status.into();
        assert_eq!(man_status, ManeuverStatus::Error(ManeuverIssue::Road));

        maneuver_req_status = ManeuverReqStatus::MRS_WARNING_ROAD_OBJECT_NOT_FOUND;
        man_status = maneuver_req_status.into();
        assert_eq!(man_status, ManeuverStatus::Warning(ManeuverIssue::Road));

        maneuver_req_status = ManeuverReqStatus::MRS_CUT_OFF_RAMP_DOWN;
        man_status = maneuver_req_status.into();
        assert_eq!(man_status, ManeuverStatus::Canceled);

        maneuver_req_status = ManeuverReqStatus::MRS_WARNING_DISCREPANCY_SPEED;
        man_status = maneuver_req_status.into();
        assert_eq!(man_status, ManeuverStatus::Warning(ManeuverIssue::Speed));
    }
}
