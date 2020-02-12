//! Holochain Json Api
//! This crate defines apis for extended json serde features, including
//! basic support for a `DefaultJson` deriving macro
#![feature(try_trait)]
#![feature(never_type)]
#![warn(unused_extern_crates)]

extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate holochain_json_derive;
#[macro_use]
extern crate shrinkwraprs;
pub mod error;
pub mod json;
