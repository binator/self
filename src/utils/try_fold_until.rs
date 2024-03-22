use core::ops::{
  FromResidual,
  Try,
};

use crate::{
  core::{
    Contexting,
    Parse,
    Parsed,
    Streaming,
  },
  utils::UtilsAtom,
};

/// Implementation of [crate::utils::Utils::try_fold_until]
#[derive(Clone)]
pub struct TryFoldUntil<Parser, Until, Init, F> {
  parser: Parser,
  until: Until,
  init: Init,
  f: F,
}

/// Function style version of [crate::utils::Utils::try_fold_until]
pub fn try_fold_until<Stream, Context, Acc, Parser, Until, Init, Ret, F>(
  parser: Parser, until: Until, init: Init, f: F,
) -> TryFoldUntil<Parser, Until, Init, F>
where
  Stream: Streaming,
  Context: Contexting<UtilsAtom<Stream>>,
  Parser: Parse<Stream, Context>,
  Until: Parse<Stream, Context>,
  Init: Fn() -> Ret,
  F: Fn(Acc, Parser::Token) -> Ret,
  Ret: Try<Output = Acc>,
  Parsed<(Acc, Until::Token), Stream, Context>: FromResidual<Ret::Residual>,
{
  TryFoldUntil {
    parser,
    until,
    init,
    f,
  }
}

impl<Stream, Context, Acc, Parser, Until, Init, Ret, F> Parse<Stream, Context>
  for TryFoldUntil<Parser, Until, Init, F>
where
  Stream: Streaming,
  Context: Contexting<UtilsAtom<Stream>>,
  Parser: Parse<Stream, Context>,
  Until: Parse<Stream, Context>,
  Init: Fn() -> Ret,
  F: Fn(Acc, Parser::Token) -> Ret,
  Ret: Try<Output = Acc>,
  Parsed<(Acc, Until::Token), Stream, Context>: FromResidual<Ret::Residual>,
{
  type Token = (Acc, Until::Token);

  fn parse(&mut self, mut stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    let mut acc = (self.init)()?;
    loop {
      match self.until.parse(stream.clone()) {
        Parsed::Success { token, stream } => {
          break Parsed::Success {
            token: (acc, token),
            stream,
          };
        }
        Parsed::Failure(until_context) => match self.parser.parse(stream) {
          Parsed::Success {
            token,
            stream: next,
          } => {
            acc = (self.f)(acc, token)?;
            stream = next;
          }
          Parsed::Failure(context) => {
            break Parsed::Failure(context.bitor(until_context).add(UtilsAtom::UntilNotReach));
          }
          Parsed::Error(context) => {
            break Parsed::Error(context);
          }
        },
        Parsed::Error(context) => {
          break Parsed::Error(context);
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {}
