use core::{
  fmt::Display,
  marker::PhantomData,
  ops::{
    Add,
    BitOr,
  },
};

use crate::{
  context::{
    First,
    Last,
  },
  core::{
    Contexting,
    ProvideElement,
  },
};

/// Will keep only the first or the last Element that was feed to it.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Keep<Behavior, Element> {
  behavior: PhantomData<Behavior>,
  element: Element,
}

impl<Behavior, Element: Display, Atom: Into<Element>> Contexting<Atom> for Keep<Behavior, Element>
where
  Self: Add<Atom, Output = Self> + BitOr<Output = Self>,
{
  fn new(context: Atom) -> Self {
    Self {
      behavior: PhantomData::default(),
      element: context.into(),
    }
  }
}

impl<Behavior, Element: Display> ProvideElement for Keep<Behavior, Element> {
  type Element = Element;

  fn last(&self) -> &Self::Element {
    &self.element
  }
}

impl<Element, Atom: Into<Element>> Add<Atom> for Keep<First, Element> {
  type Output = Self;

  fn add(self, _context: Atom) -> Self {
    self
  }
}

impl<Element> BitOr for Keep<First, Element> {
  type Output = Self;

  fn bitor(self, _other: Self) -> Self {
    self
  }
}

impl<Context, Atom: Into<Context>> Add<Atom> for Keep<Last, Context> {
  type Output = Self;

  fn add(self, context: Atom) -> Self {
    Self {
      behavior: PhantomData::default(),
      element: context.into(),
    }
  }
}

impl<Context> BitOr for Keep<Last, Context> {
  type Output = Self;

  fn bitor(self, other: Self) -> Self {
    other
  }
}
