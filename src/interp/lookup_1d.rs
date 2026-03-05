//! Implementation of 1D LookUp Table interpolation
//! Using Linear Point-Slop implementation with Binaray search for indexes
//!

use super::{
    common::{check_is_finite, check_sorted, find_closest_indices, InterpolationError, IsFinite},
    InterpResult,
};

/// This type implement a 1D LookUp table interpolation
/// Interpolation alghorithm used:
///    - Interpolation method: Linear Point-Slop
///    - Extrapolation method: Clip
///    - Index Search method: Binary Search
#[derive(Debug, Clone, PartialEq)]
pub struct LookUp1DTable<'a, const N: usize, T> {
    /// xs
    xs: &'a [T; N],
    /// ys
    ys: &'a [T; N],
}

impl<'a, const N: usize, T> LookUp1DTable<'a, N, T>
where
    T: Copy
        + core::fmt::Debug
        + PartialEq
        + Default
        + PartialOrd
        + IsFinite
        + core::ops::Sub<Output = T>
        + core::ops::Add<Output = T>
        + core::ops::Mul<Output = T>
        + core::ops::Div<Output = T>,
{
    /// Create new 1D LookUp table interpolation
    ///
    /// # Panics
    ///
    /// This panics if xs/ys are not sorted or contains non finite numbers
    ///
    #[inline]
    pub fn new(xs: &'a [T; N], ys: &'a [T; N]) -> InterpResult<Self> {
        if N == 0 {
            return Err(InterpolationError::Empty);
        }
        if !check_is_finite(xs) {
            return Err(InterpolationError::NonFiniteNumber(
                "xs contains non finite elements",
            ));
        }
        if !check_is_finite(ys) {
            return Err(InterpolationError::NonFiniteNumber(
                "ys contains non finite elements",
            ));
        }
        if !check_sorted(xs) {
            return Err(InterpolationError::UnsortedInput("xs is not sorted"));
        }
        Ok(Self { xs, ys })
    }

    /// Interpolate
    #[inline]
    pub fn interpolate(&self, x: T) -> InterpResult<T> {
        if !x.is_finite() {
            return Err(InterpolationError::NonFiniteNumber("x non finite number"));
        }
        let (il, ir) = find_closest_indices(self.xs, x);
        if il == ir {
            Ok(self.ys.get(il).copied().unwrap_or_default())
        } else {
            let coef = (x - self.xs.get(il).copied().unwrap_or_default())
                / (self.xs.get(ir).copied().unwrap_or_default()
                    - self.xs.get(il).copied().unwrap_or_default());
            Ok(self.ys.get(il).copied().unwrap_or_default()
                + coef
                    * (self.ys.get(ir).copied().unwrap_or_default()
                        - self.ys.get(il).copied().unwrap_or_default()))
        }
    }

    /// this method will be used to change the ys Value without creating a new instance each time since the xs rmain same
    pub fn update_lut(&mut self, ys: &'a [T; N]) -> InterpResult<&mut Self> {
        if N == 0 {
            return Err(InterpolationError::Empty);
        }
        if !check_is_finite(ys) {
            return Err(InterpolationError::NonFiniteNumber(
                "ys contains non finite elements",
            ));
        }
        self.ys = ys;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;

    use crate::interp::common::InterpolationError;

    use super::LookUp1DTable;

    #[test]
    fn test_binary_search() {
        let xs = [1.0, 2.0, 4.0, 5.0];
        let x_ind = xs
            .binary_search_by(|x| x.partial_cmp(&2.0).unwrap_or(Ordering::Less))
            .unwrap_or_else(|e| e);
        assert_eq!(x_ind, 1);
        let x_ind = xs
            .binary_search_by(|x| x.partial_cmp(&3.0).unwrap_or(Ordering::Less))
            .unwrap_or_else(|e| e);
        assert_eq!(x_ind, 2);
        let x_ind = xs
            .binary_search_by(|x| x.partial_cmp(&10.0).unwrap_or(Ordering::Less))
            .unwrap_or_else(|e| e);
        assert_eq!(x_ind, xs.len());
    }

    #[test]
    fn test_lkpup() {
        let xs = [2.0_f32, 3.0, 3.0, 5.0];
        let ys = [1.0_f32, 1.5, 1.5, 2.5];
        let lut = LookUp1DTable::new(&xs, &ys).unwrap();
        // Check out of range
        assert_eq!(lut.interpolate(2.0).unwrap(), 1.0);
        assert_eq!(lut.interpolate(5.0).unwrap(), 2.5);
        assert_eq!(lut.interpolate(0.0).unwrap(), 1.0);
        assert_eq!(lut.interpolate(7.0).unwrap(), 2.5);
        // Check in range
        assert_eq!(lut.interpolate(2.5).unwrap(), 1.25);
        // Check infinite
        assert_eq!(
            lut.interpolate(f32::INFINITY),
            Err(InterpolationError::NonFiniteNumber("x non finite number"))
        );
    }
    #[test]
    fn test_lkpup_one_element() {
        let xs = [2.0_f32];
        let ys = [1.0_f32];
        let lut = LookUp1DTable::new(&xs, &ys).unwrap();
        // Check out of range
        assert_eq!(lut.interpolate(2.0).unwrap(), 1.0);
        assert_eq!(lut.interpolate(0.0).unwrap(), 1.0);
        assert_eq!(lut.interpolate(7.0).unwrap(), 1.0);
    }
    #[test]
    fn test_lkpup_unsorted() {
        let mut xs = [2.0_f32, 3.0, 4.0, 5.0];
        let ys = [1.0_f32, 1.5, 2.0, 2.5];
        xs.reverse();
        let lut = LookUp1DTable::new(&xs, &ys);
        assert!(matches!(lut, Err(InterpolationError::UnsortedInput(_))));
    }
    #[test]
    fn test_lkpup_non_finite() {
        let xs = [2.0_f32, f32::NAN, 4.0, 5.0];
        let ys = [1.0_f32, 1.5, 2.0, 2.5];
        let lut = LookUp1DTable::new(&xs, &ys);
        assert!(matches!(lut, Err(InterpolationError::NonFiniteNumber(_))));
        let xs = [2.0_f32, 3.0, 4.0, 5.0];
        let ys = [1.0_f32, f32::INFINITY, 2.0, 2.5];
        let lut = LookUp1DTable::new(&xs, &ys);
        assert!(matches!(lut, Err(InterpolationError::NonFiniteNumber(_))));
    }
    #[test]
    fn test_update_lut_valid_values() {
        let xs = [1.0, 2.0, 3.0];
        let ys_initial = [4.0, 5.0, 6.0];
        let ys_new = [7.0, 8.0, 9.0];

        let mut lut = LookUp1DTable::new(&xs, &ys_initial).unwrap();
        assert!(lut.update_lut(&ys_new).is_ok());
        assert_eq!(lut.ys, &ys_new);
    }
    #[test]
    fn test_new_empty() {
        let xs: [f64; 0] = [];
        let ys: [f64; 0] = [];
        let empty_lut = LookUp1DTable::new(&xs, &ys);
        assert_eq!(empty_lut, Err(InterpolationError::Empty));
        let xs: [f64; 3] = [1.24, 4.65, 7.65];
        let ys: [f64; 3] = [0.21, 5.064, 8.14];
        let mut lut = LookUp1DTable::new(&xs, &ys).unwrap();
        let y: [f64; 3] = [2.0, f64::NAN, 4.0];
        assert_eq!(
            lut.update_lut(&y),
            Err(InterpolationError::NonFiniteNumber(
                "ys contains non finite elements"
            ))
        );
    }
}
