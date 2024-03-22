use core::{
  fmt::{
    Display,
    Formatter,
  },
  ops::{
    Add,
    BitOr,
  },
};

use crate::core::{
  Contexting,
  ProvideElement,
};

/// A Context container that ignore all Atom
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ignore;

impl Display for Ignore {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "Unknown")
  }
}

impl<Atom> Contexting<Atom> for Ignore {
  fn new(_atom: Atom) -> Self {
    Ignore
  }
}

impl<Atom> Add<Atom> for Ignore {
  type Output = Self;

  fn add(self, _rhs: Atom) -> Self::Output {
    self
  }
}

impl BitOr for Ignore {
  type Output = Self;

  fn bitor(self, _rhs: Self) -> Self::Output {
    self
  }
}

impl ProvideElement for Ignore {
  type Element = Ignore;

  fn last(&self) -> &Self::Element {
    &Ignore
  }
}
