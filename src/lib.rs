#![doc = include_str!("../readme.md")]
#![doc = include_str!("readme.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::missing_const_for_fn)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_like_matches_macro)]
#![feature(try_trait_v2)]
#![feature(trait_alias)]
#![warn(missing_docs)]
#![deny(clippy::default_numeric_fallback)]
// #![feature(never_type)]
// #![feature(exhaustive_patterns)]
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

#[cfg(feature = "alloc")]
extern crate alloc;
extern crate core;

pub mod base;
pub mod context;
pub mod stream;
pub mod utils;

mod core_atom;
pub use core_atom::*;

mod contexting;
pub use contexting::*;
mod parse;
pub use parse::*;
mod success;
pub use success::*;
mod parsed_aux;
pub use parsed_aux::*;
mod parsed;
pub use parsed::*;

mod streaming;
pub use streaming::*;
