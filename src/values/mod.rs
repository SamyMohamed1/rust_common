//! This module implement possible values, Delayed value and Hysterisis values
//!

pub mod delay;
pub mod hysterisis;
pub mod on_change;
pub mod range;
pub use delay::Delay;
pub use hysterisis::Hysterisis;
pub use range::RangeValue;

#[cfg(test)]
mod tests {}
