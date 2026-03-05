//! This module implements common algorithm for ACC
//!

use crate::io::{kph, Velocity};

/// Trait to implement speed conversion between
/// displayed speed and real speed [Study](https://confluence.dt.renault.com/pages/viewpage.action?spaceKey=SDVSOL&title=SpeedDisplay_Study)
pub trait AdvanceLaw: Copy {
    /// Type used for factor and offset
    type Type;
    /// Factor
    const FACTOR: Self::Type;
    /// Offset
    const OFFSET: Self::Type;
    /// Convert to displayed speed
    fn displayed_speed(self) -> Self;
    /// Convert to real speed
    fn real_speed(self) -> Self;
}

impl AdvanceLaw for Velocity {
    type Type = f32;
    const FACTOR: Self::Type = 1.02;
    const OFFSET: Self::Type = 2.0;

    fn displayed_speed(self) -> Self {
        let v = self.get::<kph>();
        if v > 0.1 {
            return Velocity::new::<kph>((v * Self::FACTOR) + Self::OFFSET);
        }
        Velocity::new::<kph>(0.0)
    }
    fn real_speed(self) -> Self {
        let v = self.get::<kph>();
        if v > 2.0 {
            return Velocity::new::<kph>((v - Self::OFFSET) / Self::FACTOR);
        }
        Velocity::new::<kph>(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn advance_law() {
        let s = Velocity::new::<kph>(20.0);
        let ds = s.displayed_speed();
        assert_eq!(ds.real_speed(), s);

        let s1 = Velocity::new::<kph>(0.01);
        assert_eq!(s1.displayed_speed(), Velocity::new::<kph>(0.0));
        assert_eq!(s1.real_speed(), Velocity::new::<kph>(0.0));
    }
}
