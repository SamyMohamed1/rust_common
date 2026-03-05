//! **ACC utilities** – implement common used algo for Lane Support

#![cfg_attr(not(test), no_std)]
#![deny(
    unsafe_code,
    missing_docs,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    clippy::indexing_slicing,
    clippy::index_refutable_slice,
    clippy::out_of_bounds_indexing,
    clippy::missing_asserts_for_indexing,
    clippy::unused_enumerate_index
)]

pub mod algo;
pub mod error;
pub mod interp;
pub mod io;
pub mod state;
pub mod values;
pub use acc_interface::{calibrations, configuration, ConCal};
pub use error::Result;
pub use num_traits;

/// Utility module
pub mod utils {
    use core::time::Duration;
    #[allow(unused_imports)]
    use num_traits::Float;

    /// convert f32 to Duration with a milli-second resolution
    pub fn to_duration(secs: f32) -> Duration {
        if !secs.is_finite() {
            return Duration::ZERO;
        }
        Duration::from_millis((secs * 1000.0) as u64)
    }
}

/// Re-export of needed datatypes
pub mod datatypes {
    // TODO: for now exporting all we need to filter this later
    pub use acc_interface::datatypes::*;
}
