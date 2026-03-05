//! A simple UOM implementation
//!

pub mod si;

#[allow(unused_imports)]
use num_traits::Float;

use crate::io::{rad, Angle};

/// Converts slope percentage to radian
pub fn percentage_to_rad(slope_percent: f32) -> Angle {
    Angle::new::<rad>((slope_percent / 100.0).atan())
}

#[cfg(test)]
#[cfg(not(feature = "caros"))]
mod test {
    #![allow(unused_imports)]
    use super::si::{
        acceleration::mps2,
        angle::{deg, rad},
        f32::{Acceleration, Angle, Length, Time, Velocity},
        length::meter,
        time::{msec, sec},
        velocity::{kph, mps},
    };
    use crate::io::uom::si::velocity::mph;
    use core::f32;
    use uom::si::{
        acceleration::meter_per_second_squared,
        angle::{degree, radian},
        f32::{
            Acceleration as UAccel, Angle as UAngle, Length as ULength, Time as UTime,
            Velocity as UVelocity,
        },
        length::meter as umeter,
        time::{millisecond, second},
        velocity::{kilometer_per_hour, meter_per_second, mile_per_hour},
    };
    #[test]
    fn test_accel() {
        let v0 = Acceleration::new::<mps2>(15.0);
        let v1 = UAccel::new::<meter_per_second_squared>(15.0);
        assert_eq!(v0.get::<mps2>(), v1.get::<meter_per_second_squared>());
    }
    #[test]
    fn test_angle() {
        let v0 = Angle::new::<rad>(15.0);
        let v1 = UAngle::new::<radian>(15.0);
        assert_eq!(v0.get::<rad>(), v1.get::<radian>());
        assert_eq!(v0.get::<deg>(), v1.get::<degree>());
    }
    #[test]
    fn test_length() {
        let v0 = Length::new::<meter>(15.0);
        let v1 = ULength::new::<umeter>(15.0);
        assert_eq!(v0.get::<meter>(), v1.get::<umeter>());
    }
    #[test]
    fn test_time() {
        let v0 = Time::new::<sec>(15.0);
        let v1 = UTime::new::<second>(15.0);
        assert_eq!(v0.get::<sec>(), v1.get::<second>());
        assert_eq!(v0.get::<msec>(), v1.get::<millisecond>());
    }
    #[test]
    fn test_velocity() {
        let v0 = Velocity::new::<mps>(15.0);
        let v1 = UVelocity::new::<meter_per_second>(15.0);
        assert_eq!(v0.get::<mps>(), v1.get::<meter_per_second>());
        assert!(
            (v0.get::<kph>() - v1.get::<kilometer_per_hour>())
                .abs()
                .powi(2)
                < f32::EPSILON
        );
        assert!((v0.get::<mph>() - v1.get::<mile_per_hour>()).abs().powi(2) < f32::EPSILON);
    }
}
