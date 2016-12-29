#![recursion_limit="200"]

// Begin serde

#![cfg_attr(feature = "serde_macros", feature(plugin, custom_derive))]

#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate serde;

#[cfg(feature = "serde_macros")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

// End serde

pub use serialization::*;
