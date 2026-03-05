//! This modules implements interpolation for 1D and 2D

mod common;
mod lookup_1d;
mod lookup_2d;

pub use common::InterpolationError;
pub use common::Result as InterpResult;
pub use lookup_1d::LookUp1DTable;
pub use lookup_2d::LookUp2DTable;
