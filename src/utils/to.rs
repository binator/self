use crate::{
  Parse,
  Parsed,
};

/// Implementation of [crate::utils::Utils::to]
#[derive(Clone)]
pub struct To<Parser, OtherToken> {
  parser: Parser,
  to: OtherToken,
}

impl<Stream, Context, Parser, OtherToken> Parse<Stream, Context> for To<Parser, OtherToken>
where
  Parser: Parse<Stream, Context>,
  OtherToken: Clone,
{
  type Token = OtherToken;

  fn parse(&mut self, stream: Stream) -> Parsed<OtherToken, Stream, Context> {
    self.parser.parse(stream).map_token(|_| self.to.clone())
  }
}

/// Function style version of [crate::utils::Utils::to]
pub fn to<Stream, Context, Parser, OtherToken>(
  parser: Parser, to: OtherToken,
) -> To<Parser, OtherToken>
where
  Parser: Parse<Stream, Context>,
  OtherToken: Clone,
{
  To { parser, to }
}
