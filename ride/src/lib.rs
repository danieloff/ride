// Dan October-7-2017\
#![deny(non_camel_case_types)]
#![deny(unused_parens)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]
//#![warn(missing_docs)] // FIXME: should be denied.
#![deny(unused_results)]
#![allow(unused_unsafe)] // FIXME: should be denied
#![allow(missing_copy_implementations)]

#[macro_use]
extern crate scan_fmt_count;

pub mod core;
pub mod engine;
pub mod asset;
pub mod assets;
pub mod entity;
pub mod entities;
