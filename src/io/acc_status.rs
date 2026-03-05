//! ACC status as defined in SWEET200
//!

use num_derive::{FromPrimitive, ToPrimitive};

/// Acc status as defined in SWEET200
#[repr(C)]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq)]
pub enum AccStatus {
    /// ACC OFF
    Off = 0b0000,
    /// ACC in take off
    TakeOff = 0b0001,
    /// ACC in stop
    Stop = 0b0010,
    /// ACC waiting
    Waiting = 0b0011,
    /// ACC suspended
    Suspended = 0b0100,
    /// ACC in brake only mode
    BrakeOnlyMode = 0b0101,
    /// ACC regulation
    Regulation = 0b0110,
    /// ACC driver override
    DriverOverride = 0b0111,
    /// ACC in failure State
    Failure = 0b1000,
}

impl From<&AccStatus> for acc_interface::datatypes::AccStatus {
    fn from(value: &AccStatus) -> Self {
        use acc_interface::datatypes::AccStatus::*;
        match value {
            AccStatus::Off => ACC_OFF,
            AccStatus::TakeOff => ACC_TAKE_OFF,
            AccStatus::Stop => ACC_STOP,
            AccStatus::Waiting => ACC_WAITING,
            AccStatus::Suspended => ACC_SUSPENDED,
            AccStatus::BrakeOnlyMode => ACC_BRAKE_ONLY_MODE,
            AccStatus::Regulation => ACC_REGULATION,
            AccStatus::DriverOverride => ACC_DRIVER_OVERRIDE,
            AccStatus::Failure => ACC_FAILURE,
        }
    }
}
