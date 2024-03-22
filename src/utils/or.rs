use core::ops::BitOr;

use crate::core::{
  Parse,
  Parsed,
};

/// Implementation of [crate::utils::Utils::or]
#[derive(Clone)]
pub struct Or<ParserA, ParserB> {
  a: ParserA,
  b: ParserB,
}

impl<Stream, Context, ParserA, ParserB> Parse<Stream, Context> for Or<ParserA, ParserB>
where
  Stream: Clone,
  ParserA: Parse<Stream, Context>,
  ParserB: Parse<Stream, Context, Token = ParserA::Token>,
  Context: BitOr<Output = Context>,
{
  type Token = ParserA::Token;

  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    match self.a.parse(stream.clone()) {
      success @ Parsed::Success { .. } => success,
      Parsed::Failure(context_a) => self
        .b
        .parse(stream)
        .map_context(|context_b| context_a.bitor(context_b)),
      Parsed::Error(context) => Parsed::Error(context),
    }
  }
}

/// Function style version of [crate::utils::Utils::or]
pub fn or<Stream, Context, ParserA, ParserB>(a: ParserA, b: ParserB) -> Or<ParserA, ParserB>
where
  Stream: Clone,
  ParserA: Parse<Stream, Context>,
  ParserB: Parse<Stream, Context>,
  Context: BitOr,
{
  Or { a, b }
}

#[cfg(tests)]
mod tests {}
