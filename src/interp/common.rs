//! Common Types and functions used for Interpolation
//!

use core::{cmp::Ordering, fmt};

/// A type to define error possible errors for interpolation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterpolationError {
    /// Empty LUT
    Empty,
    /// Unsorted LUT
    UnsortedInput(&'static str),
    /// LUT contains non finite numbers
    NonFiniteNumber(&'static str),
}

impl fmt::Display for InterpolationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(any(feature = "error_in_core", feature = "caros"))]
impl core::error::Error for InterpolationError {}

/// An Alias Type for Interpolation Result
pub type Result<T> = core::result::Result<T, InterpolationError>;

/// Check if all elements of a slice are sorted
#[allow(clippy::indexing_slicing)]
pub(crate) fn check_sorted<T: PartialOrd>(x: &[T]) -> bool {
    x.windows(2).all(|v| {
        assert!(v.len() == 2);
        v[0].partial_cmp(&v[1])
            .map(Ordering::is_le)
            .unwrap_or_default()
    })
}

/// Private trait bound to avoid implementation of is_finite
/// for other types than specified ones
mod private {
    use core::fmt::Display;

    pub trait Sealed: Copy + Display {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}
/// A custom trait to check if a number is finite or not
pub trait IsFinite: private::Sealed {
    fn is_finite(&self) -> bool {
        true
    }
}

impl IsFinite for u32 {}
impl IsFinite for i32 {}
impl IsFinite for u64 {}
impl IsFinite for i64 {}
impl IsFinite for f32 {
    fn is_finite(&self) -> bool {
        f32::is_finite(*self)
    }
}
impl IsFinite for f64 {
    fn is_finite(&self) -> bool {
        f64::is_finite(*self)
    }
}

/// check if all elements of a slice are finite
pub(crate) fn check_is_finite<T: IsFinite>(x: &[T]) -> bool {
    x.iter().all(|v| v.is_finite())
}

/// Find closest
pub(crate) fn find_closest_indices<T>(xs: &[T], x: T) -> (usize, usize)
where
    T: Copy + PartialOrd,
{
    match xs.binary_search_by(|v| v.partial_cmp(&x).unwrap_or(Ordering::Less)) {
        Ok(i) => (i, i),
        Err(i) => {
            if i == 0 {
                (0, 0)
            } else {
                let sz: usize = xs.len() - 1;
                let il = i - 1;
                let ir = i.min(sz);
                debug_assert!((ir - il) <= 1);
                (il, ir)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InterpolationError, IsFinite};
    use crate::interp::common::check_is_finite;

    #[test]
    fn test_display_for_interp_error() {
        let error = InterpolationError::Empty;
        assert_eq!(format!("{error}"), "Empty");
    }
    #[test]
    fn test_is_finite() {
        let num: u32 = 14;
        assert!(num.is_finite());
        let num_f32: f32 = 0.25;
        assert!(num_f32.is_finite());
        let num_f64: f64 = f64::INFINITY;
        assert!(!num_f64.is_finite());
    }
    #[test]
    fn test_all_finite_f32() {
        let values = [1.0f32, 2.0, 3.0];
        assert!(check_is_finite(&values));
    }
}
