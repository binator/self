use crate::{
  Parse,
  Parsed,
  Success,
};

/// Implementation of [crate::utils::Utils::and]
#[derive(Clone)]
pub struct And<ParserA, ParserB> {
  parser_a: ParserA,
  parser_b: ParserB,
}

impl<TokenA, Stream, Context, ParserA, TokenB, ParserB> Parse<Stream, Context>
  for And<ParserA, ParserB>
where
  ParserA: Parse<Stream, Context, Token = TokenA>,
  ParserB: Parse<Stream, Context, Token = TokenB>,
{
  type Token = (TokenA, TokenB);

  fn parse(&mut self, stream: Stream) -> Parsed<(TokenA, TokenB), Stream, Context> {
    let Success { token, stream } = self.parser_a.parse(stream)?;

    self
      .parser_b
      .parse(stream)
      .map_token(|token_b| (token, token_b))
  }
}

/// Function style version of [crate::utils::Utils::and]
pub fn and<TokenA, Stream, Context, ParserA, TokenB, ParserB>(
  parser_a: ParserA, parser_b: ParserB,
) -> And<ParserA, ParserB>
where
  ParserA: Parse<Stream, Context, Token = TokenA>,
  ParserB: Parse<Stream, Context, Token = TokenB>,
{
  And { parser_a, parser_b }
}
