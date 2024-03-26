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

/// Return Success if item from stream is partially equal to t.
pub fn is<Stream, Context, T: 'static>(expect: T) -> impl Parse<Stream, Context, Token = T>
where
  Stream: Streaming,
  Context: Contexting<BaseAtom<T>>,
  Context: Contexting<CoreAtom<Stream>>,
  Stream::Item: Into<T>,
  T: Clone + PartialEq<T> + Debug,
{
  Is { expect }
}

#[derive(Debug)]
struct Is<T> {
  expect: T,
}

impl<Stream, Context, T> Parse<Stream, Context> for Is<T>
where
  Stream: Streaming,
  Context: Contexting<BaseAtom<T>>,
  Context: Contexting<CoreAtom<Stream>>,
  Stream::Item: Into<T>,
  T: Clone + PartialEq<T> + Debug + 'static,
{
  type Token = T;

  #[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", name = "is", skip_all, ret(Display))
  )]
  fn parse(&mut self, stream: Stream) -> Parsed<T, Stream, Context> {
    any
      .map(Into::into)
      .add_atom(|| BaseAtom::Is {
        t: None,
        expect: self.expect.clone(),
      })
      .try_map(|item: T| {
        if self.expect == item {
          Ok(item)
        } else {
          Err(Context::new(BaseAtom::Is {
            t: Some(item),
            expect: self.expect.clone(),
          }))
        }
      })
      .parse(stream)
  }
}

struct IsNot<T> {
  not_expect: T,
}

/// Return Success if item from stream is not partially equal to t.
pub fn is_not<Stream, Context, T: 'static>(not_expect: T) -> impl Parse<Stream, Context, Token = T>
where
  Stream: Streaming,
  Context: Contexting<BaseAtom<T>>,
  Context: Contexting<CoreAtom<Stream>>,
  Stream::Item: Into<T>,
  T: Clone + PartialEq<T> + Debug,
{
  IsNot { not_expect }
}

impl<Stream, Context, T> Parse<Stream, Context> for IsNot<T>
where
  Stream: Streaming,
  Context: Contexting<BaseAtom<T>>,
  Context: Contexting<CoreAtom<Stream>>,
  Stream::Item: Into<T>,
  T: Clone + PartialEq<T> + Debug + 'static,
{
  type Token = T;

  #[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", name = "is_not", skip_all, ret(Display))
  )]
  fn parse(&mut self, stream: Stream) -> Parsed<T, Stream, Context> {
    any
      .map(Into::into)
      .add_atom(|| BaseAtom::IsNot {
        t: None,
        not_expect: self.not_expect.clone(),
      })
      .try_map(|found: T| {
        if found != self.not_expect {
          Ok(found)
        } else {
          Err(Context::new(BaseAtom::IsNot {
            t: Some(found),
            not_expect: self.not_expect.clone(),
          }))
        }
      })
      .parse(stream)
  }
}
