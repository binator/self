use core::{
  fmt::Display,
  marker::PhantomData,
  ops::{
    Add,
    BitOr,
  },
};

use smallvec::{
  smallvec,
  SmallVec,
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

/// Will keep the last Stack of elements feed to it.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Stack<Behavior, Context, const N: usize> {
  behavior: PhantomData<Behavior>,
  backtrace: SmallVec<[Context; N]>,
}

impl<Behavior, Context: Display, Atom: Into<Context>, const N: usize> Contexting<Atom>
  for Stack<Behavior, Context, N>
where
  Self: Add<Atom, Output = Self> + BitOr<Output = Self>,
{
  fn new(atom: Atom) -> Self {
    Self {
      behavior: PhantomData::default(),
      backtrace: smallvec![atom.into()],
    }
  }
}

impl<Behavior, Element: Display, const N: usize> ProvideElement for Stack<Behavior, Element, N> {
  type Element = Element;

  fn last(&self) -> &Self::Element {
    // can't be empty
    self.backtrace.last().unwrap()
  }
}

impl<Context, Atom: Into<Context>, const N: usize> Add<Atom> for Stack<First, Context, N> {
  type Output = Self;

  fn add(mut self, context: Atom) -> Self {
    self.backtrace.push(context.into());
    self
  }
}

impl<Context, const N: usize> BitOr for Stack<First, Context, N> {
  type Output = Self;

  fn bitor(self, _other: Self) -> Self {
    self
  }
}

impl<Context, Atom: Into<Context>, const N: usize> Add<Atom> for Stack<Last, Context, N> {
  type Output = Self;

  fn add(mut self, context: Atom) -> Self {
    self.backtrace.push(context.into());
    self
  }
}

impl<Context, const N: usize> BitOr for Stack<Last, Context, N> {
  type Output = Self;

  fn bitor(self, other: Self) -> Self {
    other
  }
}
