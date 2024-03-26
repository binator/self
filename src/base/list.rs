use core::fmt::Debug;

use crate::{
  base::{
    any,
    BaseAtom,
  },
  utils::{
    Utils,
    UtilsAtom,
  },
  Contexting,
  CoreAtom,
  Parse,
  Parsed,
  Streaming,
};

/// Take a list of T and return a Parser that will partially Eq in order Item
/// produced by Stream with T in the list
pub fn list<Stream, Context, T>(
  list: &'static [T],
) -> impl Parse<Stream, Context, Token = &'static [T]>
where
  Stream: Streaming,
  Stream::Item: Into<T>,
  Context: Contexting<CoreAtom<Stream>>,
  Context: Contexting<BaseAtom<T>>,
  Context: Contexting<UtilsAtom<Stream>>,
  T: Clone + PartialEq<T> + Debug + 'static,
{
  List { list }
}

struct List<T: 'static> {
  list: &'static [T],
}

impl<Stream, Context, T> Parse<Stream, Context> for List<T>
where
  Stream: Streaming,
  Stream::Item: Into<T>,
  Context: Contexting<CoreAtom<Stream>>,
  Context: Contexting<BaseAtom<T>>,
  Context: Contexting<UtilsAtom<Stream>>,
  T: Clone + PartialEq<T> + Debug + 'static,
{
  type Token = &'static [T];

  #[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", name = "tag", skip_all, ret(Display))
  )]
  fn parse(&mut self, stream: Stream) -> Parsed<&'static [T], Stream, Context> {
    any
      .map(Into::into)
      .try_fold_iter(
        self.list.iter().cloned(),
        || Ok(self.list),
        |list, found, expect| {
          if expect == found {
            Ok(list)
          } else {
            Err(Context::new(BaseAtom::Is {
              t: Some(found),
              expect,
            }))
          }
        },
      )
      .add_atom(|| BaseAtom::List { list: self.list })
      .parse(stream)
  }
}
