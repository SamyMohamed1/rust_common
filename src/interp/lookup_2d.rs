//! Implementation of 2D LookUp Table interpolation
//! Using Linear Point-Slop implementation with Binaray search for indexes
//!

use super::{
    common::{check_is_finite, check_sorted, find_closest_indices, InterpolationError, IsFinite},
    InterpResult,
};

/// This type implement a 2D LookUp table interpolation
/// Interpolation alghorithm used:
///    - Interpolation method: Linear Point-Slop
///    - Extrapolation method: Clip
///    - Index Search method: Binary Search
#[derive(Debug, Clone, PartialEq)]
pub struct LookUp2DTable<'a, const M: usize, const N: usize, T> {
    /// xs
    xs: &'a [T; M],
    /// ys
    ys: &'a [T; N],
    /// data
    data: &'a [[T; N]; M],
}

impl<'a, const M: usize, const N: usize, T> LookUp2DTable<'a, M, N, T>
where
    T: Copy
        + core::fmt::Debug
        + IsFinite
        + PartialEq
        + Default
        + PartialOrd
        + core::ops::Sub<Output = T>
        + core::ops::Add<Output = T>
        + core::ops::Mul<Output = T>
        + core::ops::Div<Output = T>,
{
    /// create new 2D LookUp table interpolation
    ///
    /// # Panics
    ///
    /// This panics if xs/ys are not sorted or contains non finite numbers
    ///
    #[inline]
    pub fn new(xs: &'a [T; M], ys: &'a [T; N], data: &'a [[T; N]; M]) -> InterpResult<Self> {
        if N == 0 || M == 0 {
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
        if !check_sorted(ys) {
            return Err(InterpolationError::UnsortedInput("ys is not sorted"));
        }
        if !data.iter().all(|v| check_is_finite(v)) {
            return Err(InterpolationError::NonFiniteNumber(
                "data contains non finite elements",
            ));
        }
        Ok(Self { xs, ys, data })
    }

    /// Interpolate
    #[inline]
    pub fn interpolate(&self, x: T, y: T) -> InterpResult<T> {
        if !x.is_finite() {
            return Err(InterpolationError::NonFiniteNumber("x non finite number"));
        }
        if !y.is_finite() {
            return Err(InterpolationError::NonFiniteNumber("y non finite number"));
        }
        // X indices
        let (x_il, x_ir) = find_closest_indices(self.xs, x);
        // Y indices
        let (y_il, y_ir) = find_closest_indices(self.ys, y);
        // Interpolate
        if (x_il == x_ir) && (y_il == y_ir) {
            Ok(self
                .data
                .get(x_il)
                .and_then(|y| y.get(y_il))
                .copied()
                .unwrap_or_default())
        } else if x_il == x_ir {
            let v00 = self
                .data
                .get(x_il)
                .and_then(|y| y.get(y_il))
                .copied()
                .unwrap_or_default();
            let v01 = self
                .data
                .get(x_il)
                .and_then(|y| y.get(y_ir))
                .copied()
                .unwrap_or_default();
            // let v10 = self.data.get(x_ir).and_then(|y| y.get(y_il)).copied().unwrap_or_default();
            // let v11 = self.data.get(x_ir).and_then(|y| y.get(y_ir)).copied().unwrap_or_default();
            let coef = (y - self.ys.get(y_il).copied().unwrap_or_default())
                / (self.ys.get(y_ir).copied().unwrap_or_default()
                    - self.ys.get(y_il).copied().unwrap_or_default());
            Ok(v00 + coef * (v01 - v00))
        } else if y_il == y_ir {
            let v00 = self
                .data
                .get(x_il)
                .and_then(|y| y.get(y_il))
                .copied()
                .unwrap_or_default();
            // let v01 = self.data.get(x_il).and_then(|y| y.get(y_ir)).copied().unwrap_or_default();
            let v10 = self
                .data
                .get(x_ir)
                .and_then(|y| y.get(y_il))
                .copied()
                .unwrap_or_default();
            // let v11 = self.data.get(x_ir).and_then(|y| y.get(y_ir)).copied().unwrap_or_default();
            let coef = (x - self.xs.get(x_il).copied().unwrap_or_default())
                / (self.xs.get(x_ir).copied().unwrap_or_default()
                    - self.xs.get(x_il).copied().unwrap_or_default());
            Ok(v00 + coef * (v10 - v00))
        } else {
            let v00 = self
                .data
                .get(x_il)
                .and_then(|y| y.get(y_il))
                .copied()
                .unwrap_or_default();
            let v01 = self
                .data
                .get(x_il)
                .and_then(|y| y.get(y_ir))
                .copied()
                .unwrap_or_default();
            let v10 = self
                .data
                .get(x_ir)
                .and_then(|y| y.get(y_il))
                .copied()
                .unwrap_or_default();
            let v11 = self
                .data
                .get(x_ir)
                .and_then(|y| y.get(y_ir))
                .copied()
                .unwrap_or_default();
            let coef_x = (x - self.xs.get(x_il).copied().unwrap_or_default())
                / (self.xs.get(x_ir).copied().unwrap_or_default()
                    - self.xs.get(x_il).copied().unwrap_or_default());
            let coef_y = (y - self.ys.get(y_il).copied().unwrap_or_default())
                / (self.ys.get(y_ir).copied().unwrap_or_default()
                    - self.ys.get(y_il).copied().unwrap_or_default());
            let x00 = v00 + coef_x * (v10 - v00);
            let x01 = v01 + coef_x * (v11 - v01);
            Ok(x00 + coef_y * (x01 - x00))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interp::InterpolationError;
    use core::cmp::Ordering;

    use super::LookUp2DTable;

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
        let xs = [2.0_f32, 3.0];
        let ys = [1.0_f32, 1.5, 2.0, 2.5];
        let data = [[0.5_f32, 1.0, 1.5, 2.0], [2.0, 2.5, 3.0, 3.5]];
        let lut = LookUp2DTable::new(&xs, &ys, &data).unwrap();
        // Check both out of range
        assert_eq!(lut.interpolate(0.0, 0.0).unwrap(), 0.5);
        assert_eq!(lut.interpolate(2.0, 1.0).unwrap(), 0.5);
        assert_eq!(lut.interpolate(3.0, 2.5).unwrap(), 3.5);
        assert_eq!(lut.interpolate(13.0, 13.0).unwrap(), 3.5);
        // Check one is out of range
        assert_eq!(lut.interpolate(2.5, 0.0).unwrap(), 1.25);
        assert_eq!(lut.interpolate(2.5, 13.0).unwrap(), 2.75);
        assert_eq!(lut.interpolate(0.0, 2.25).unwrap(), 1.75);
        assert_eq!(lut.interpolate(13.0, 2.25).unwrap(), 3.25);

        assert_eq!(lut.interpolate(2.5, 2.25).unwrap(), 2.5);

        let xs = [2.0_f32, 3.0];
        let ys = [1.0_f32, 1.5, 1.5, 1.5];
        let data = [[0.5_f32, 1.0, 1.5, 2.0], [2.0, 2.5, 3.0, 3.5]]; // erreur
        let lut = LookUp2DTable::new(&xs, &ys, &data).unwrap();
        assert_eq!(lut.interpolate(2.5, 1.7).unwrap(), 2.75);
    }
    #[test]
    fn test_error_for_new() {
        let xs: [f64; 0] = [];
        let ys: [f64; 0] = [];
        let data: [[f64; 0]; 0] = [];
        let lut = LookUp2DTable::new(&xs, &ys, &data);
        assert_eq!(lut, Err(InterpolationError::Empty));
        let xs: [f64; 3] = [1.0, f64::NAN, 5.4];
        let ys: [f64; 2] = [4.5, 6.8];
        let data: [[f64; 2]; 3] = [[1.05, 8.6], [74.9, 4.6], [7.6, 41.6]];
        let lut = LookUp2DTable::new(&xs, &ys, &data);
        assert_eq!(
            lut,
            Err(InterpolationError::NonFiniteNumber(
                "xs contains non finite elements",
            ))
        );
        let xs: [f64; 3] = [1.0, 2.4, 5.4];
        let ys: [f64; 2] = [4.5, f64::NAN];
        let lut = LookUp2DTable::new(&xs, &ys, &data);
        assert_eq!(
            lut,
            Err(InterpolationError::NonFiniteNumber(
                "ys contains non finite elements",
            ))
        );
        let xs: [f64; 3] = [1.0, 5.4, 2.4];
        let ys: [f64; 2] = [4.5, 6.7];
        let lut = LookUp2DTable::new(&xs, &ys, &data);
        assert_eq!(
            lut,
            Err(InterpolationError::UnsortedInput("xs is not sorted"))
        );
        let xs: [f64; 3] = [1.0, 2.4, 5.4];
        let ys: [f64; 2] = [6.5, 4.7];
        let lut = LookUp2DTable::new(&xs, &ys, &data);
        assert_eq!(
            lut,
            Err(InterpolationError::UnsortedInput("ys is not sorted"))
        );
        let ys: [f64; 2] = [4.5, 6.7];
        let data: [[f64; 2]; 3] = [[1.05, 8.6], [74.9, f64::NAN], [7.6, 41.6]];
        let lut = LookUp2DTable::new(&xs, &ys, &data);
        assert_eq!(
            lut,
            Err(InterpolationError::NonFiniteNumber(
                "data contains non finite elements",
            ))
        );
    }
    #[test]
    fn test_interpolate_error() {
        let x: f64 = f64::NAN;
        let y: f64 = 4.1;
        let xs: [f64; 3] = [1.0, 2.4, 5.4];
        let ys: [f64; 2] = [4.5, 6.7];
        let data: [[f64; 2]; 3] = [[1.05, 8.6], [74.9, 4.6], [7.6, 41.6]];
        let lut = LookUp2DTable::new(&xs, &ys, &data).unwrap();
        let lut_update = lut.interpolate(x, y);
        assert_eq!(
            lut_update,
            Err(InterpolationError::NonFiniteNumber("x non finite number"))
        );
        let x: f64 = 4.1;
        let y: f64 = f64::NAN;
        let lut_update = lut.interpolate(x, y);
        assert_eq!(
            lut_update,
            Err(InterpolationError::NonFiniteNumber("y non finite number"))
        );
    }
}
