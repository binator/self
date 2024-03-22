//! To learn how to use binator read [crate::core] doc.

#![doc = include_str!("../readme.md")]
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

extern crate core as stdcore;

pub mod base;
pub mod context;
pub mod core;
pub mod stream;
pub mod utils;
