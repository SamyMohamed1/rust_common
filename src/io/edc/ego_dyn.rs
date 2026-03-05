//! Define ego Dynamics for ACC
//!
use acc_interface::datatypes::{egodynamics_t, DataQualifier, EgoMovementStatus};

use crate::io::{mps, mps2, Acceleration, Velocity};

/// Ego Mouvement Status state
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum EgoMvtStatus {
    /// Ego moving Forward
    #[default]
    MvForward,
    /// Ego Stopped Moving
    StMoving,
    /// Ego Moving Backward
    MvBackward,
    /// Ego Moving Backward
    Unknown,
}

impl From<EgoMovementStatus> for EgoMvtStatus {
    fn from(value: EgoMovementStatus) -> Self {
        match value {
            EgoMovementStatus::EMS_MOVING_FORWARD => Self::MvForward,
            EgoMovementStatus::EMS_STOPPED_MOVING => Self::StMoving,
            EgoMovementStatus::EMS_MOVING_BACKWARD => Self::MvBackward,
            EgoMovementStatus::EMS_UNKNOWN => Self::Unknown,
        }
    }
}

/// Define Ego Dynamics
#[derive(Debug, Clone, Default)]
pub struct EgoDynamics {
    /// Longitudinal speed
    pub vx: Velocity,
    /// Lateral speed
    pub vy: Velocity,
    /// Longitudinal acceleration
    pub ax: Acceleration,
    /// Lateral acceleration
    pub ay: Acceleration,
    /// Movement status
    pub mv_status: EgoMvtStatus,
}

impl From<(&egodynamics_t, &DataQualifier)> for EgoDynamics {
    fn from(value: (&egodynamics_t, &DataQualifier)) -> Self {
        if !matches!(value.1, DataQualifier::DQ_NORMAL) {
            return Self::default();
        }
        let value = value.0;
        let vx = Velocity::new::<mps>(value.velocity[0]);
        let vy = Velocity::new::<mps>(value.velocity[1]);
        let ax = Acceleration::new::<mps2>(value.acceleration[0]);
        let ay = Acceleration::new::<mps2>(value.acceleration[1]);
        Self {
            vx,
            vy,
            ax,
            ay,
            mv_status: value.movementstatus.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::kph;

    use super::*;

    #[test]
    fn test_dynamics() {
        let ego = EgoDynamics {
            vx: Velocity::new::<mps>(3.5),
            vy: Velocity::new::<mps>(0.0),
            ..EgoDynamics::default()
        };
        let speed = ego.signed_speed();
        println!("{speed:?}");
        assert_eq!(speed.get::<kph>(), 3.5 * 3.6)
    }
    #[test]
    fn test_ego_mvt_status_conversion() {
        assert_eq!(
            EgoMvtStatus::from(EgoMovementStatus::EMS_MOVING_FORWARD),
            EgoMvtStatus::MvForward
        );
        assert_eq!(
            EgoMvtStatus::from(EgoMovementStatus::EMS_STOPPED_MOVING),
            EgoMvtStatus::StMoving
        );
        assert_eq!(
            EgoMvtStatus::from(EgoMovementStatus::EMS_MOVING_BACKWARD),
            EgoMvtStatus::MvBackward
        );
    }

    #[test]
    fn test_ego_dynamics_conversion() {
        let ed_t = egodynamics_t {
            velocity: [12.0, -3.0, 0.0],
            acceleration: [1.5, -0.5, 0.0],
            movementstatus: EgoMovementStatus::EMS_MOVING_FORWARD,
            ..Default::default()
        };

        let ed = EgoDynamics::from((&ed_t, &DataQualifier::DQ_NORMAL));

        assert_eq!(ed.vx.get::<mps>(), 12.0);
        assert_eq!(ed.vy.get::<mps>(), -3.0);
        assert_eq!(ed.ax.get::<mps2>(), 1.5);
        assert_eq!(ed.ay.get::<mps2>(), -0.5);
        assert_eq!(ed.mv_status, EgoMvtStatus::MvForward);
    }

    #[test]
    fn test_ego_dynamics_conversion_stopped() {
        let ed_t = egodynamics_t {
            velocity: [0.0, 0.0, 0.0],
            acceleration: [0.0, 0.0, 0.0],
            movementstatus: EgoMovementStatus::EMS_STOPPED_MOVING,
            ..Default::default()
        };

        let ed = EgoDynamics::from((&ed_t, &DataQualifier::DQ_NORMAL));

        assert_eq!(ed.vx.get::<mps>(), 0.0);
        assert_eq!(ed.vy.get::<mps>(), 0.0);
        assert_eq!(ed.ax.get::<mps2>(), 0.0);
        assert_eq!(ed.ay.get::<mps2>(), 0.0);
        assert_eq!(ed.mv_status, EgoMvtStatus::StMoving);
    }

    #[test]
    fn test_ego_dynamics_conversion_backward() {
        let ed_t = egodynamics_t {
            velocity: [-5.0, 1.0, 0.0],
            acceleration: [-2.0, 0.5, 0.0],
            movementstatus: EgoMovementStatus::EMS_MOVING_BACKWARD,
            ..Default::default()
        };

        let ed = EgoDynamics::from((&ed_t, &DataQualifier::DQ_NORMAL));

        assert_eq!(ed.vx.get::<mps>(), -5.0);
        assert_eq!(ed.vy.get::<mps>(), 1.0);
        assert_eq!(ed.ax.get::<mps2>(), -2.0);
        assert_eq!(ed.ay.get::<mps2>(), 0.5);
        assert_eq!(ed.mv_status, EgoMvtStatus::MvBackward);
    }
}
