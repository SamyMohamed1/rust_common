//! Define a Range Value: based on current value and min/max check if value is valid

use super::delay::DelayedValue;

/// Range Value
#[derive(Debug, Clone, Copy)]
pub struct RangeValue<T, U>
where
    T: Copy + PartialOrd,
    U: Copy + PartialOrd,
{
    /// Current Value
    pub value: T,
    /// minimum value for activation
    min_act: U,
    /// maximum value for activation
    max_act: U,
    /// minimum value for de-activation
    min_deact: U,
    /// maximum value for de-activation
    max_deact: U,
}

impl<T, U> RangeValue<T, U>
where
    T: Copy + PartialOrd,
    U: Copy + PartialOrd,
{
    /// [RangeValue] constructor
    pub const fn new(value: T, min_act: U, max_act: U, min_deact: U, max_deact: U) -> Self {
        Self {
            value,
            min_act,
            max_act,
            min_deact,
            max_deact,
        }
    }
}

impl<T, U> DelayedValue for RangeValue<T, U>
where
    T: Copy + PartialOrd + Into<U>,
    U: Copy + PartialOrd,
{
    fn on(&self) -> bool {
        let v: U = self.value.into();
        (self.min_act <= v) && (v <= self.max_act)
    }
    fn off(&self) -> bool {
        let v: U = self.value.into();
        (self.min_deact > v) || (v > self.max_deact)
    }
}

#[cfg(test)]
mod test {
    use crate::values::delay::DelayedValue;

    use super::RangeValue;

    #[test]
    fn test_new() {
        let value = 5;
        let min_act = 0;
        let max_act = 15;
        let min_deact = 10;
        let max_deact = 30;
        let range_value = RangeValue::new(value, min_act, max_act, min_deact, max_deact);
        assert_eq!(range_value.value, value);
        assert_eq!(range_value.min_act, min_act);
        assert_eq!(range_value.max_act, max_act);
        assert_eq!(range_value.min_deact, min_deact);
        assert_eq!(range_value.max_deact, max_deact);
        assert!(range_value.on());
        assert!(range_value.off());
    }
}
