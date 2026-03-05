//! International System of Units (SI) and International System of Quantities (ISQ) implementations
#![allow(non_camel_case_types)]
macro_rules! impl_type {
    ($name:ident, $trait:tt) => {
        impl $name {
            /// Create a new Value based on Unit
            pub fn new<T: $crate::io::uom::si::Unit + $trait>(value: f32) -> Self {
                Self(value * T::FACTOR)
            }
            /// Get a value based on the Unit
            pub fn get<T: $crate::io::uom::si::Unit + $trait>(&self) -> f32 {
                self.0 / T::FACTOR
            }
        }
    };
}
macro_rules! impl_unit {
    ($n:ident, $f:expr) => {
        impl $crate::io::uom::si::Unit for $n {
            const FACTOR: f32 = $f;
        }
    };
}
/// Unit trait definition
pub trait Unit {
    /// Used factor de convert from base unit to another unit
    const FACTOR: f32;
}
/// Storage module for f32 implementation
pub mod f32 {
    use crate::io::{meter, mps, sec};

    use super::{
        acceleration::AccelBound, angle::AngleBound, length::LengthBound, time::TimeBound,
        velocity::VelocityBound,
    };

    /// Acceleration
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
    pub struct Acceleration(f32);
    /// Angle
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
    pub struct Angle(f32);
    /// Length
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
    pub struct Length(f32);
    /// Time
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
    pub struct Time(f32);
    /// Velocity
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
    pub struct Velocity(f32);
    impl_type!(Acceleration, AccelBound);
    impl_type!(Angle, AngleBound);
    impl_type!(Length, LengthBound);
    impl_type!(Time, TimeBound);
    impl_type!(Velocity, VelocityBound);
    impl core::ops::Div<Velocity> for Length {
        type Output = Time;
        fn div(self, rhs: Velocity) -> Self::Output {
            Time::new::<sec>(self.get::<meter>() / rhs.get::<mps>())
        }
    }
    impl core::ops::Add for Velocity {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self(self.get::<mps>() + rhs.get::<mps>())
        }
    }
    impl core::ops::Sub for Velocity {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Self(self.get::<mps>() - rhs.get::<mps>())
        }
    }
    impl core::ops::Add for Length {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self(self.get::<meter>() + rhs.get::<meter>())
        }
    }
    impl core::ops::Sub for Length {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Self(self.get::<meter>() - rhs.get::<meter>())
        }
    }
}
/// Acceleration
pub mod acceleration {
    /// Bound trait for Accel
    pub trait AccelBound {}
    /// meter per square second Unit
    #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
    pub struct mps2;
    impl_unit!(mps2, 1.0);
    impl AccelBound for mps2 {}
}
/// Angle
pub mod angle {
    /// Bound trait for Angle
    pub trait AngleBound {}
    /// radian Unit
    #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
    pub struct rad;
    /// degree Unit
    #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
    pub struct deg;
    impl_unit!(rad, 1.0);
    impl_unit!(deg, core::f32::consts::PI / 180.0);
    impl AngleBound for rad {}
    impl AngleBound for deg {}
}
/// Length
pub mod length {
    /// Bound trait for Length
    pub trait LengthBound {}
    /// meter Unit
    #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
    pub struct meter;
    /// meter Unit
    #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
    pub struct cm;
    impl_unit!(meter, 1.0);
    impl_unit!(cm, 1e-2);
    impl LengthBound for meter {}
    impl LengthBound for cm {}
}
/// Time
pub mod time {
    /// Bound trait for Time
    pub trait TimeBound {}
    /// milliseconds Unit
    #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
    pub struct msec;
    /// seconds Unit
    #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
    pub struct sec;
    impl_unit!(sec, 1.0);
    impl_unit!(msec, 1e-3);
    impl TimeBound for sec {}
    impl TimeBound for msec {}
}
/// Velocity
pub mod velocity {
    /// Bound trait for Velocity
    pub trait VelocityBound {}
    /// meter per second Unit
    #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
    pub struct mps;
    /// kilometer per hour Unit
    #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
    pub struct kph;
    /// miles per hour Unit
    #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
    pub struct mph;
    impl_unit!(mps, 1.0);
    impl_unit!(kph, 1.0 / 3.6);
    impl_unit!(mph, 4.470_4_e-1);
    impl VelocityBound for mps {}
    impl VelocityBound for kph {}
    impl VelocityBound for mph {}
}
