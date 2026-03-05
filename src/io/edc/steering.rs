//! Define ego Steering Status for ACC
//!

#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;

use crate::io::{deg, Angle};
use acc_interface::datatypes::steering_t;
#[allow(unused_imports)]
use num_traits::Float;
#[derive(Debug, Clone, Default)]
/// Steering Status
pub struct Steering {
    /// steering wheel angle rate
    pub steering_wheel_angle_rate: f32,
    /// steering wheel angle
    pub steering_wheel_angle: Angle,
    /// driver steering wheel torque
    pub driver_steering_wheel_torque: f32,
}

impl From<&steering_t> for Steering {
    fn from(value: &steering_t) -> Self {
        Self {
            steering_wheel_angle_rate: value.steeringWheelAngleRate,
            steering_wheel_angle: Angle::new::<deg>(value.rackAngleSensorEps),
            driver_steering_wheel_torque: value.driverSteeringWheelTorque,
        }
    }
}

impl Steering {
    /// get absolute steering angle delta
    pub fn get_abs_steering_angle_delta_with(&self, init_angle: Angle) -> f32 {
        (self.steering_wheel_angle.get::<deg>() - init_angle.get::<deg>()).abs()
    }
}

#[cfg(feature = "caros")]
impl From<&oem::sdv_adas_ego_ego_steering::Steering_t> for Steering {
    fn from(value: &oem::sdv_adas_ego_ego_steering::Steering_t) -> Self {
        Self {
            steering_wheel_angle_rate: value.steering_wheel_angle_rate,
            steering_wheel_angle: Angle::new::<deg>(value.rack_angle_sensor_eps),
            driver_steering_wheel_torque: value.driver_steering_wheel_torque,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use acc_interface::datatypes::steering_t;
    use num_traits::abs;

    #[test]
    fn test_from_steering_t() {
        let steering_data = steering_t {
            steeringWheelAngleRate: 1.5,
            rackAngleSensorEps: 30.0,
            driverSteeringWheelTorque: 2.5,
            ..Default::default()
        };

        let steering = Steering::from(&steering_data);

        assert_eq!(steering.steering_wheel_angle_rate, 1.5);
        assert!(abs(steering.steering_wheel_angle.get::<deg>() - 30.0) < 1e-4);
        assert_eq!(steering.driver_steering_wheel_torque, 2.5);
    }

    #[test]
    fn test_get_abs_steering_angle_delta_with() {
        let steering = Steering {
            steering_wheel_angle: Angle::new::<deg>(30.00),
            ..Default::default()
        };
        let init_angle = Angle::new::<deg>(20.0);

        let delta = steering.get_abs_steering_angle_delta_with(init_angle);
        assert!(abs(delta - 10.0) < 1e-4);
    }

    #[test]
    fn test_default_steering() {
        let steering = Steering::default();

        assert_eq!(steering.steering_wheel_angle_rate, 0.0);
        assert_eq!(steering.steering_wheel_angle.get::<deg>(), 0.0);
        assert_eq!(steering.driver_steering_wheel_torque, 0.0);
    }
}
