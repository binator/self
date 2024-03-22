#![doc = include_str!("readme.md")]

mod ignore;
pub use ignore::*;
mod keep;
pub use keep::*;
#[cfg(feature = "stack")]
mod stack;
#[cfg(feature = "stack")]
pub use stack::*;
#[cfg(feature = "tree")]
mod tree;
#[cfg(feature = "tree")]
pub use tree::*;

/// Used to determine by Keep and Stack to determine their Behavior
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct First;

/// Used to determine by Keep and Stack to determine their Behavior
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Last;
