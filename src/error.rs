//! # Erros
//! This module defines Some Possible Errors
//!
use core::fmt::Display;

use crate::interp::InterpolationError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Speed Limiter Possible Erros
pub enum IaccErrors {
    /// Row Index is out of range
    RowIndexOutOfBounds,
    /// Column index is out of range
    ColumnIndexOutOfBounds,
    /// Array is Empty
    EmptyCalib,
    /// Out Of range
    SpeedOutOfRange,
}

impl Display for IaccErrors {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(any(feature = "error_in_core", feature = "caros"))]
impl core::error::Error for IaccErrors {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Speed Limiter Possible Erros
pub enum Error {
    /// Errors due to interpolation
    Interp(InterpolationError),
    /// error due to comparisation
    Comparisation(&'static str),
    /// error due target out of Range
    NoTargetSelected,
    /// Iacc Errors
    Iacc(IaccErrors),
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(any(feature = "error_in_core", feature = "caros"))]
impl core::error::Error for Error {}

impl From<InterpolationError> for Error {
    fn from(value: InterpolationError) -> Self {
        Self::Interp(value)
    }
}

impl From<IaccErrors> for Error {
    fn from(value: IaccErrors) -> Self {
        Self::Iacc(value)
    }
}

/// pub alias type to be used in the lib
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::interp::InterpolationError;

    #[test]
    fn test_conversion_from_interpolation_error() {
        let interp_error = InterpolationError::NonFiniteNumber("Interpolation Error");
        let dw_error = Error::from(interp_error.clone());
        assert_eq!(Error::Interp(interp_error), dw_error);
    }

    #[test]
    fn test_display_error() {
        let comp_error = Error::Comparisation("Comparison Error");
        assert_eq!(
            format!("{comp_error:?}"),
            "Comparisation(\"Comparison Error\")"
        );
    }
}
