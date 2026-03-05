//! IO conversion errors
//!

/// IO conversion errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IoError {
    /// Invalid Feature ID provided
    InvalidFeatId,
    /// Invalid protobuf enum conversion
    InvalidProtoEnum,
}

impl core::fmt::Display for IoError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(any(feature = "error_in_core", feature = "caros"))]
impl core::error::Error for IoError {}
