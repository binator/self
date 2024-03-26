use core::ops::{
  FromResidual,
  Try,
};

use crate::{
  utils::UtilsAtom,
  Contexting,
  Parse,
  Parsed,
  Streaming,
  Success,
};

/// Implementation of [crate::utils::Utils::try_fold_iter]
#[derive(Clone)]
pub struct TryFoldIter<Parser, IntoIter, Init, F> {
  parser: Parser,
  iter: IntoIter,
  init: Init,
  f: F,
}

/// Function style version of [crate::utils::Utils::try_fold_iter]
pub fn try_fold_iter<Stream, Context, Parser, IntoIter, Acc, Init, Ret, F>(
  parser: Parser, iter: IntoIter, init: Init, f: F,
) -> TryFoldIter<Parser, IntoIter, Init, F>
where
  Parser: Parse<Stream, Context>,
  Stream: Streaming,
  Context: Contexting<UtilsAtom<Stream>>,
  IntoIter: IntoIterator + Clone,
  Init: Fn() -> Ret,
  F: Fn(Acc, Parser::Token, IntoIter::Item) -> Ret,
  Ret: Try<Output = Acc>,
  Parsed<Acc, Stream, Context>: FromResidual<Ret::Residual>,
{
  TryFoldIter {
    parser,
    iter,
    init,
    f,
  }
}

impl<Stream, Context, Parser, IntoIter, Acc, Init, Ret, F> Parse<Stream, Context>
  for TryFoldIter<Parser, IntoIter, Init, F>
where
  Parser: Parse<Stream, Context>,
  Stream: Streaming,
  Context: Contexting<UtilsAtom<Stream>>,
  IntoIter: IntoIterator + Clone,
  Init: Fn() -> Ret,
  F: Fn(Acc, Parser::Token, IntoIter::Item) -> Ret,
  Ret: Try<Output = Acc>,
  Parsed<Acc, Stream, Context>: FromResidual<Ret::Residual>,
{
  type Token = Acc;

  fn parse(&mut self, mut stream: Stream) -> Parsed<Acc, Stream, Context> {
    let mut acc = (self.init)()?;

    for i in self.iter.clone().into_iter() {
      let Success {
        token,
        stream: next,
      } = self.parser.parse(stream)?;

      acc = (self.f)(acc, token, i)?;
      stream = next;
    }

    Parsed::Success { token: acc, stream }
  }
}

#[cfg(test)]
mod tests {}
