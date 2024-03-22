use crate::core::{
  Parse,
  Parsed,
  Success,
};

/// Implementation of [crate::utils::Utils::drop_and]
#[derive(Clone)]
pub struct DropAnd<ParserA, ParserB> {
  parser_a: ParserA,
  parser_b: ParserB,
}

impl<Stream, Context, ParserA, ParserB> Parse<Stream, Context> for DropAnd<ParserA, ParserB>
where
  ParserA: Parse<Stream, Context>,
  ParserB: Parse<Stream, Context>,
{
  type Token = ParserB::Token;

  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    let Success { token: _, stream } = self.parser_a.parse(stream)?;

    self.parser_b.parse(stream)
  }
}

/// Function style version of [crate::utils::Utils::drop_and]
pub fn drop_and<Stream, Context, ParserA, ParserB>(
  parser_a: ParserA, parser_b: ParserB,
) -> DropAnd<ParserA, ParserB>
where
  ParserA: Parse<Stream, Context>,
  ParserB: Parse<Stream, Context>,
{
  DropAnd { parser_a, parser_b }
}
