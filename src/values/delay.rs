//! This module implement Delayed value, based on its rising and falling edges
//!
use core::time::Duration;
#[allow(unused_imports)]
use num_traits::Float;
/// This trait define the state of a Delayed Value
pub trait DelayedValue: Clone + Copy {
    /// ON
    fn on(&self) -> bool;
    /// OFF
    fn off(&self) -> bool {
        !self.on()
    }
}

impl DelayedValue for bool {
    fn on(&self) -> bool {
        *self
    }
}

/// Delay
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Delay<T: DelayedValue> {
    /// Current Value
    pub value: T,
    flag: bool,
    /// Duration since value is ON
    pub duration_on: Duration,
    /// Duration since value is OFF
    pub duration_off: Duration,
}

impl<T: DelayedValue> Delay<T> {
    /// Create a new [`Delay`]
    pub const fn new(v: T) -> Self {
        Self {
            value: v,
            flag: false,
            duration_on: Duration::ZERO,
            duration_off: Duration::MAX,
        }
    }

    /// Create a new [`Delay`] and set initial state
    pub const fn new_with_flag(v: T, flag: bool) -> Self {
        let (duration_on, duration_off) = if flag {
            (Duration::MAX, Duration::ZERO)
        } else {
            (Duration::ZERO, Duration::MAX)
        };
        Self {
            value: v,
            flag,
            duration_on,
            duration_off,
        }
    }

    /// Step function and update internal state
    fn _step(
        &mut self,
        value: T,
        duration_on: Duration,
        duration_off: Duration,
        elapsed: Duration,
    ) -> &mut Self {
        // Handle special case of Duration::ZERO and rounding of conversion from floats to duration
        // Expected resolution is 1 ms
        let duration_on = duration_on.max(Duration::from_millis(2)) - Duration::from_millis(1);
        let duration_off = duration_off.max(Duration::from_millis(2)) - Duration::from_millis(1);
        if self.flag && value.off() {
            self.duration_off = self.duration_off.saturating_add(elapsed);
        } else if !self.flag && value.on() {
            self.duration_on = self.duration_on.saturating_add(elapsed);
        } else if self.flag {
            self.duration_on = self.duration_on.saturating_add(elapsed);
            self.duration_off = Duration::ZERO;
        } else {
            self.duration_on = Duration::ZERO;
            self.duration_off = self.duration_off.saturating_add(elapsed);
        }
        if self.flag && self.duration_off >= duration_off {
            self.flag = false;
            self.duration_on = Duration::ZERO;
        } else if !self.flag && self.duration_on >= duration_on {
            self.flag = true;
            self.duration_off = Duration::ZERO;
        }
        self.value = value;
        self
    }

    /// Reset Flags and Counters
    pub fn reset(&mut self) -> &mut Self {
        self.flag = false;
        self.duration_on = Duration::ZERO;
        self.duration_off = Duration::MAX;
        self
    }

    /// update state
    pub fn step(
        &mut self,
        value: T,
        duration_on: Duration,
        duration_off: Duration,
        elapsed: Duration,
    ) -> &mut Self {
        if elapsed.is_zero() {
            return self;
        }
        self._step(value, duration_on, duration_off, elapsed)
    }
    /// Check if the state is ON
    pub fn is_on(&self) -> bool {
        self.flag
    }
    /// Check if the state is OFF
    pub fn is_off(&self) -> bool {
        !self.flag
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    fn to_dur(v: u64) -> Duration {
        Duration::from_millis(v * 10)
    }
    #[test]
    fn test_delay_new() {
        let mut delay = Delay::new(false);
        assert!(!delay.is_on());
        assert!(delay.is_off());
        delay = Delay::new_with_flag(true, true);
        assert_eq!(delay.duration_on, Duration::MAX);
        assert_eq!(delay.duration_off, Duration::ZERO);
    }

    #[test]
    fn test_delay_off_on_off() {
        let mut delay = Delay::new(false);
        assert_eq!(delay.duration_on, to_dur(0));
        assert!(delay._step(true, to_dur(3), to_dur(2), to_dur(1)).is_off());
        assert_eq!(delay.duration_on, to_dur(1));
        assert!(delay._step(true, to_dur(3), to_dur(2), to_dur(1)).is_off());
        assert_eq!(delay.duration_on, to_dur(2));
        assert!(delay._step(true, to_dur(3), to_dur(2), to_dur(1)).is_on()); // rising false to true
        assert_eq!(delay.duration_on, to_dur(3));
        assert!(delay._step(true, to_dur(3), to_dur(2), to_dur(1)).is_on()); // rising false to true
        assert_eq!(delay.duration_on, to_dur(4));
        assert!(delay._step(true, to_dur(3), to_dur(2), to_dur(1)).is_on()); // rising false to true
        assert_eq!(delay.duration_on, to_dur(5));
        assert!(delay._step(true, to_dur(3), to_dur(2), to_dur(1)).is_on()); // rising false to true
        assert_eq!(delay.duration_on, to_dur(6));
        assert!(delay._step(false, to_dur(3), to_dur(2), to_dur(1)).is_on());
        assert_eq!(delay.duration_on, to_dur(6));
        assert!(delay._step(false, to_dur(3), to_dur(2), to_dur(1)).is_off());
        assert_eq!(delay.duration_on, to_dur(0));
    }

    #[test]
    fn test_delay_on_off() {
        let mut delay1 = Delay::new(true); // delay from true to false
        let mut delay2 = Delay::new(true); // delay  from false to true  overtime rise
        assert!(delay1._step(true, to_dur(2), to_dur(1), to_dur(1)).is_off());
        assert!(delay2._step(true, to_dur(4), to_dur(3), to_dur(1)).is_off());
        assert!(delay1._step(true, to_dur(2), to_dur(1), to_dur(1)).is_on()); // min 2 is reach
        assert!(delay2._step(true, to_dur(4), to_dur(3), to_dur(1)).is_off());
        assert!(delay1._step(true, to_dur(2), to_dur(1), to_dur(1)).is_on()); // min 2 is reach
        assert!(delay2._step(true, to_dur(4), to_dur(3), to_dur(1)).is_off());
        assert!(delay1._step(true, to_dur(2), to_dur(1), to_dur(1)).is_on()); // min 2 is reach
        assert!(delay2._step(true, to_dur(4), to_dur(3), to_dur(1)).is_on()); // min 4 is reach

        let mut event: Option<String> = None;
        let mut delay2 = Delay::new_with_flag(true, false);
        let mut overtime_flag = false;

        for n in 1..10 {
            if event.is_none() && delay2.is_off() {
                overtime_flag = delay2._step(true, to_dur(3), to_dur(2), to_dur(1)).is_on();
                println!(
                    " n {} overtime delay {} wait start my event is {:?}  delay for ON is {}",
                    n, overtime_flag, event, 3
                );
            } else if delay2.is_on() {
                event = Some("overtime".to_string());
                overtime_flag = delay2._step(false, to_dur(3), to_dur(2), to_dur(1)).is_on();
                println!(
                    " n {} overtime delay {} start and wait stop event is {:?} delay for OFF is {}",
                    n,
                    overtime_flag,
                    event.clone().unwrap().as_str(),
                    2
                );
            } else if event.is_some() {
                event = None;
                println!(" n {n} overtime delay {overtime_flag} stop my event is {event:?}");
            }
        }
    }
    #[test]
    fn test_reset() {
        let mut delay = Delay::new(true);
        delay.reset();
        assert!(!delay.flag);
        assert_eq!(delay.duration_on, Duration::ZERO);
        assert_eq!(delay.duration_off, Duration::MAX);
    }
}
