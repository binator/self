use crate::{
  Parse,
  Parsed,
};

/// Implementation of [crate::utils::Utils::map]
#[derive(Clone)]
pub struct Map<Parser, F> {
  parser: Parser,
  f: F,
}

impl<Stream, Context, Parser, TokenSecond, F> Parse<Stream, Context> for Map<Parser, F>
where
  Parser: Parse<Stream, Context>,
  F: Fn(Parser::Token) -> TokenSecond,
{
  type Token = TokenSecond;

  fn parse(&mut self, stream: Stream) -> Parsed<TokenSecond, Stream, Context> {
    self.parser.parse(stream).map_token(&self.f)
  }
}

/// Function style version of [crate::utils::Utils::map]
pub fn map<Stream, Context, Parser, TokenSecond, F>(parser: Parser, f: F) -> Map<Parser, F>
where
  Parser: Parse<Stream, Context>,
  F: Fn(Parser::Token) -> TokenSecond,
{
  Map { parser, f }
}
