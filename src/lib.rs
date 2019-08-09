#![allow(
    clippy::redundant_field_names,
    clippy::new_without_default,
    clippy::large_enum_variant,
    clippy::len_zero,
    clippy::needless_return,
    clippy::redundant_closure,
    dead_code,
    non_camel_case_types
)]

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "async")]
extern crate futures;
#[cfg(feature = "async")]
extern crate hyper;
#[cfg(feature = "sync")]
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

#[cfg(feature = "async")]
pub mod async;
pub mod models;
#[cfg(feature = "sync")]
pub mod sync;

pub mod parameters;