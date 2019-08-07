#![allow(clippy::redundant_field_names, clippy::new_without_default)]
#![allow(clippy::len_zero, clippy::needless_return, clippy::redundant_closure)]
#![allow(dead_code, non_camel_case_types)]

#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod models;
