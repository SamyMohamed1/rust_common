//! This module implement On Change value
//!

use core::ops::Deref;

/// On change value
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct OnChange<T> {
    /// Previous value
    prev: Option<T>,
    /// Current value
    curr: T,
}

impl<T> Deref for OnChange<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.curr
    }
}

impl<T: PartialEq> OnChange<T> {
    /// [`OnChange`] constructor
    pub const fn new(value: T) -> Self {
        Self {
            prev: None,
            curr: value,
        }
    }
    /// set value
    pub fn set(&mut self, value: T) {
        let prev = core::mem::replace(&mut self.curr, value);
        self.prev = Some(prev);
    }
    /// Check if value was set at least once
    #[inline]
    pub fn is_set(&self) -> bool {
        self.prev.is_some()
    }
    /// check if value changed
    #[inline]
    pub fn changed(&self) -> bool {
        self.prev
            .as_ref()
            .map(|x| x.ne(&self.curr))
            .unwrap_or_default()
    }
}
impl<T: Clone + PartialEq> OnChange<T> {
    /// check if value changed and reset
    #[inline]
    pub fn changed_and_reset(&mut self) -> bool {
        let ret = self.changed();
        self.prev = Some(self.curr.clone());
        ret
    }
}
