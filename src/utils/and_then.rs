use crate::{
  Parse,
  Parsed,
  Success,
};

/// Implementation of [crate::utils::Utils::and_then]
#[derive(Clone)]
pub struct AndThen<Parser, F> {
  parser: Parser,
  f: F,
}

impl<Stream, Context, ParserA, ParserB, F> Parse<Stream, Context> for AndThen<ParserA, F>
where
  ParserA: Parse<Stream, Context>,
  ParserB: Parse<Stream, Context>,
  F: Fn(ParserA::Token) -> ParserB,
{
  type Token = ParserB::Token;

  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    let Success { token, stream } = self.parser.parse(stream)?;

    (self.f)(token).parse(stream)
  }
}

/// Function style version of [crate::utils::Utils::and_then]
pub fn and_then<Stream, Context, ParserA, ParserB, F>(parser: ParserA, f: F) -> AndThen<ParserA, F>
where
  ParserA: Parse<Stream, Context>,
  ParserB: Parse<Stream, Context>,
  F: Fn(ParserA::Token) -> ParserB,
{
  AndThen { parser, f }
}
