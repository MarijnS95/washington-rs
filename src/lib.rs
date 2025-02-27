#![doc(html_no_source)]

extern crate windows;

#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::derivable_impls,
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::extra_unused_lifetimes,
    clippy::useless_transmute
)]
mod Microsoft;

pub use Microsoft::States::*;
