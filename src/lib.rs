mod avutil;

pub mod binding;

#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    improper_ctypes,
    clippy::all
)]
pub mod ffi {
    pub use super::binding::*;
    pub use crate::avutil::{_avutil::*, common::*, error::*, pixfmt::*, rational::*};
    //include!(concat!(env!("OUT_DIR"), "/binding.rs"));
}
