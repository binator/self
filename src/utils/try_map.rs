use core::ops::{
  FromResidual,
  Try,
};

use crate::{
  Parse,
  Parsed,
};

/// Implementation of [crate::utils::Utils::try_map]
#[derive(Clone)]
pub struct TryMap<Parser, F> {
  parser: Parser,
  f: F,
}

impl<Stream, Context, Parser, B, Ret, F> Parse<Stream, Context> for TryMap<Parser, F>
where
  Parser: Parse<Stream, Context>,
  F: Fn(Parser::Token) -> Ret,
  Ret: Try<Output = B>,
  Parsed<B, Stream, Context>: FromResidual<Ret::Residual>,
{
  type Token = B;

  fn parse(&mut self, stream: Stream) -> Parsed<B, Stream, Context> {
    match self.parser.parse(stream) {
      Parsed::Success { token, stream } => {
        let token = (self.f)(token)?;

        Parsed::Success { token, stream }
      }
      Parsed::Failure(context) => Parsed::Failure(context),
      Parsed::Error(context) => Parsed::Error(context),
    }
  }
}

/// Function style version of [crate::utils::Utils::try_map]
pub fn try_map<Stream, Context, Parser, B, Ret, F>(parser: Parser, f: F) -> TryMap<Parser, F>
where
  Parser: Parse<Stream, Context>,
  F: Fn(Parser::Token) -> Ret,
  Ret: Try<Output = B>,
  Parsed<B, Stream, Context>: FromResidual<Ret::Residual>,
{
  TryMap { parser, f }
}
