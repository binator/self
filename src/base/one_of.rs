use core::fmt::Debug;

use crate::{
  base::{
    any,
    BaseAtom,
  },
  utils::Utils,
  Contexting,
  CoreAtom,
  Parse,
  Parsed,
  Streaming,
};

/// Will check if next Item from Stream is partially equal
/// to one of T in the list.
pub fn one_of<Stream, Context, T>(list: &'static [T]) -> impl Parse<Stream, Context, Token = T>
where
  Stream: Streaming,
  Stream::Item: Into<T>,
  Context: Contexting<BaseAtom<T>>,
  Context: Contexting<CoreAtom<Stream>>,
  T: PartialEq<T> + Clone + Debug,
{
  OneOf { list }
}

struct OneOf<T: 'static> {
  list: &'static [T],
}

impl<Stream, Context, T> Parse<Stream, Context> for OneOf<T>
where
  Stream: Streaming,
  Stream::Item: Into<T>,
  Context: Contexting<BaseAtom<T>>,
  Context: Contexting<CoreAtom<Stream>>,
  T: PartialEq<T> + Clone + Debug,
{
  type Token = T;

  #[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", name = "one_of", skip_all, ret(Display))
  )]
  fn parse(&mut self, stream: Stream) -> Parsed<T, Stream, Context> {
    any
      .map(Into::into)
      .add_atom(|| BaseAtom::OneOf {
        found: None,
        expected: self.list,
      })
      .try_map(|item: T| {
        if self.list.iter().any(|t| t == &item) {
          Ok(item)
        } else {
          Err(Context::new(BaseAtom::OneOf {
            found: Some(item),
            expected: self.list,
          }))
        }
      })
      .parse(stream)
  }
}

/// Will check if next Item from Stream is not partially equal
/// to one of T in the list.
pub fn none_of<Stream, Context, T>(list: &'static [T]) -> impl Parse<Stream, Context, Token = T>
where
  Stream: Streaming,
  Stream::Item: Into<T>,
  Context: Contexting<BaseAtom<T>>,
  Context: Contexting<CoreAtom<Stream>>,
  T: PartialEq<T> + Clone + Debug,
{
  NoneOf { list }
}

struct NoneOf<T: 'static> {
  list: &'static [T],
}

impl<Stream, Context, T> Parse<Stream, Context> for NoneOf<T>
where
  Stream: Streaming,
  Stream::Item: Into<T>,
  Context: Contexting<BaseAtom<T>>,
  Context: Contexting<CoreAtom<Stream>>,
  T: PartialEq<T> + Clone + Debug,
{
  type Token = T;

  #[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", name = "none_of", skip_all, ret(Display))
  )]
  fn parse(&mut self, stream: Stream) -> Parsed<T, Stream, Context> {
    any
      .map(Into::into)
      .add_atom(|| BaseAtom::NoneOf {
        found: None,
        not_expected: self.list,
      })
      .try_map(|t: T| {
        if self.list.iter().all(|i| i != &t) {
          Ok(t)
        } else {
          Err(Context::new(BaseAtom::NoneOf {
            found: Some(t),
            not_expected: self.list,
          }))?
        }
      })
      .parse(stream)
  }
}
