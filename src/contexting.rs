use core::{
  fmt::Display,
  ops::{
    Add,
    BitOr,
  },
};

/// Contexting is a trait used to report failure and error.
/// This idea is too have a tree of context that will help final user to
/// understand the error. It's can also help for debugging purpose.
pub trait Contexting<Atom>:
  Sized + Add<Atom, Output = Self> + BitOr<Output = Self> + ProvideElement
{
  /// Create a new Context from a Atom.
  /// Implementation should try to avoid allocate for only one Atom.
  fn new(atom: Atom) -> Self;
}

/// This is an utily trait
pub trait ProvideElement {
  /// Element associate with a Context.
  /// This is often an enum of different Atom.
  type Element: Display;

  /// return the last Element added to a Context
  fn last(&self) -> &Self::Element;
}
