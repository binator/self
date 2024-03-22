use core::{
  fmt::{
    Debug,
    Display,
  },
  ops::{
    Add,
    BitOr,
  },
};

use crate::core::{
  Acc,
  Contexting,
  ProvideElement,
};

/// Will keep the full tree of elements feed to it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tree<Element> {
  /// End of the Tree, content one Element
  Element(Element),
  /// Stack part of the Tree
  And(alloc::vec::Vec<Self>),
  /// Branch part of the Tree
  Or(alloc::vec::Vec<Self>),
}

impl<Context> Tree<Context> {
  fn unwrap_last_and_context(and: &[Self]) -> &Context {
    match and.last().unwrap() {
      Tree::Element(context) => context,
      Tree::And(_) => unreachable!(),
      Tree::Or(_) => unreachable!(),
    }
  }

  fn unwrap_last_or_context(or: &[Self]) -> &Context {
    match or.last().unwrap() {
      Tree::Element(context) => context,
      Tree::And(and) => Self::unwrap_last_and_context(and),
      Tree::Or(or) => Self::unwrap_last_or_context(or),
    }
  }
}

impl<Context: Display, Atom: Into<Context>> Contexting<Atom> for Tree<Context> {
  fn new(context: Atom) -> Self {
    Self::Element(context.into())
  }
}

impl<Element: Display> ProvideElement for Tree<Element> {
  type Element = Element;

  fn last(&self) -> &Element {
    match self {
      Tree::Element(context) => context,
      Tree::And(and) => Self::unwrap_last_and_context(and),
      Tree::Or(or) => Self::unwrap_last_or_context(or),
    }
  }
}

impl<Context, Atom: Into<Context>> Add<Atom> for Tree<Context> {
  type Output = Self;

  fn add(self, context: Atom) -> Self {
    let rhs = context.into();
    match self {
      one @ Tree::Element(..) => Self::And(alloc::vec![one, Self::Element(rhs)]),
      Tree::And(stack) => Self::And(stack.acc(Self::Element(rhs))),
      tree @ Tree::Or(_) => Self::And(alloc::vec![tree, Self::Element(rhs)]),
    }
  }
}

impl<Context> BitOr for Tree<Context> {
  type Output = Self;

  fn bitor(self, other: Self) -> Self {
    match self {
      one @ Tree::Element(..) => Self::Or(alloc::vec![one, other]),
      stack @ Tree::And(..) => Self::Or(alloc::vec![stack, other]),
      Tree::Or(tree) => Self::Or(tree.acc(other)),
    }
  }
}

struct DisplayTree<'a, Context> {
  tree: &'a Tree<Context>,
  and: usize,
  or: usize,
}

impl<'a, Context> DisplayTree<'a, Context> {
  pub fn new(tree: &'a Tree<Context>) -> Self {
    Self {
      tree,
      and: 0,
      or: 0,
    }
  }
}

impl<'a, Context> Display for DisplayTree<'a, Context>
where
  Context: Display,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self.tree {
      Tree::Element(one) => {
        for _ in 0..self.or {
          write!(f, "+")?;
        }
        for _ in self.or..self.and {
          write!(f, "-")?;
        }
        writeln!(f, "{}", one)
      }
      Tree::And(stack) => {
        for (i, context) in stack.iter().rev().enumerate() {
          write!(
            f,
            "{}",
            DisplayTree {
              tree: context,
              and: self.and + i,
              or: self.or,
            }
          )?;
        }

        Ok(())
      }
      Tree::Or(tree) => {
        for context in tree {
          write!(
            f,
            "{}",
            DisplayTree {
              tree: context,
              and: self.and,
              or: self.and,
            }
          )?;
        }

        Ok(())
      }
    }
  }
}

impl<Context> Display for Tree<Context>
where
  Context: Display,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", DisplayTree::new(self))
  }
}
