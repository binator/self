use crate::core::{
  Parse,
  Parsed,
  Success,
};

/// Implementation of [crate::utils::Utils::and_drop]
#[derive(Clone)]
pub struct AndDrop<ParserA, ParserB> {
  parser_a: ParserA,
  parser_b: ParserB,
}

impl<Stream, Context, ParserA, ParserB> Parse<Stream, Context> for AndDrop<ParserA, ParserB>
where
  ParserA: Parse<Stream, Context>,
  ParserB: Parse<Stream, Context>,
{
  type Token = ParserA::Token;

  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    let Success {
      token: token_a,
      stream,
    } = self.parser_a.parse(stream)?;

    self.parser_b.parse(stream).map_token(|_| token_a)
  }
}

/// Function style version of [crate::utils::Utils::and_drop]
pub fn and_drop<Stream, Context, ParserA, ParserB>(
  parser_a: ParserA, parser_b: ParserB,
) -> AndDrop<ParserA, ParserB>
where
  ParserA: Parse<Stream, Context>,
  ParserB: Parse<Stream, Context>,
{
  AndDrop { parser_a, parser_b }
}
