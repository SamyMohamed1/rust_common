//! This modules defines all inputs and outputs for ACC application
//!

pub mod uom;
pub use acc_interface::datatypes::CancelAppReasonT;
use core::time::Duration;
pub use uom::si::{
    acceleration::mps2,
    angle::{deg, rad},
    f32::{Acceleration, Angle, Length, Time, Velocity},
    length::meter,
    time::{msec, sec},
    velocity::{kph, mps},
};

pub mod acc_status;
pub mod config;
pub mod contextual;
pub mod driver_guide;
pub mod edc;
pub mod ego;
pub mod environmental;
pub mod error;
pub mod failure;
pub mod hmi;
pub mod lines;
pub mod maneuver_status;
#[cfg(feature = "caros")]
pub mod pbconvert;
pub mod road_type;
pub mod road_user;

/// Wrapper for cancel maneuver's cut off duration
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CancelArg {
    /// cut off duration
    pub duration: Duration,
    /// cancel reason
    pub reason: CancelAppReasonT,
}

impl CancelArg {
    /// constructor with cut off duration
    pub const fn new(duration: Duration, reason: CancelAppReasonT) -> Self {
        Self { duration, reason }
    }

    /// check if the Cancel arguments are valid
    pub fn is_valid(&self) -> bool {
        !matches!(self.reason, CancelAppReasonT::CART_UNSPECIFIED)
    }
}
