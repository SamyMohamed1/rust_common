//! This module implement Hysterisis value
//!

/// todo
#[derive(Debug, Default, Clone)]
pub struct Hysterisis<T: Copy + PartialOrd> {
    /// Current value
    pub value: T,
    flag: bool,
}

impl<T: Copy + PartialOrd + core::ops::Add<Output = T>> Hysterisis<T> {
    /// [`Hysterisis`] constructor
    pub const fn new(value: T, is_high: bool) -> Self {
        Self {
            value,
            flag: is_high,
        }
    }
    /// Update using the low value and an offset
    pub fn update_with_offset(&mut self, value: T, low: T, offset: T) -> &mut Self {
        let high = low + offset;
        self.update(value, low, high)
    }
    /// Update the value based on new LOW/HIGH values
    pub fn update(&mut self, value: T, low: T, high: T) -> &mut Self {
        debug_assert!(low
            .partial_cmp(&high)
            .map(|v| v.is_le())
            .unwrap_or_default());
        self.value = value;
        if self.flag && value <= low {
            self.flag = false;
        } else if !self.flag && value >= high {
            self.flag = true;
        }
        self
    }
    /// Check if value is HIGH
    pub fn is_high(&self) -> bool {
        self.flag
    }
    /// Check if value is LOW
    pub fn is_low(&self) -> bool {
        !self.flag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyst_new() {
        assert!(Hysterisis::new(23.0_f32, false).is_low());
        assert!(Hysterisis::new(23.0_f32, true).is_high());
    }
    #[test]
    fn test_hyst_low_high_new() {
        let mut hyst = Hysterisis::new(23.0_f32, false);
        assert!(hyst.update(10.0, 15.0, 16.0).is_low());
        assert!(hyst.update(10.0, 5.0, 9.0).is_high());
    }
    #[test]
    #[should_panic]
    fn test_hyst_wrong_limits() {
        let mut hyst = Hysterisis::new(23.0_f32, false);
        hyst.update(10.0, 15.0, 6.0);
    }

    #[test]
    fn test_update_with_offset() {
        let mut hysteresis = Hysterisis::new(2, true);
        hysteresis.update_with_offset(5, 7, 2);
        assert!(!hysteresis.flag)
    }
}
