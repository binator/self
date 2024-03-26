use crate::{
  utils::UtilsAtom,
  Contexting,
  Parse,
  Parsed,
};

/// Implementation of [crate::utils::Utils::filter_map]
#[derive(Clone)]
pub struct FilterMap<Parser, F> {
  parser: Parser,
  f: F,
}

impl<Stream, Context, Parser, TokenSecond, F> Parse<Stream, Context> for FilterMap<Parser, F>
where
  Parser: Parse<Stream, Context>,
  F: Fn(Parser::Token) -> Option<TokenSecond>,
  Context: Contexting<UtilsAtom<Stream>>,
{
  type Token = TokenSecond;

  fn parse(&mut self, stream: Stream) -> Parsed<TokenSecond, Stream, Context> {
    let success = self.parser.parse(stream)?;
    if let Some(token) = (self.f)(success.token) {
      Parsed::new_success(token, success.stream)
    } else {
      Parsed::new_failure(Context::new(UtilsAtom::Filter))
    }
  }
}

/// Function style version of [crate::utils::Utils::filter_map]
pub fn filter_map<Stream, Context, Parser, TokenSecond, F>(
  parser: Parser, f: F,
) -> FilterMap<Parser, F>
where
  Parser: Parse<Stream, Context>,
  F: Fn(Parser::Token) -> Option<TokenSecond>,
  Context: Contexting<UtilsAtom<Stream>>,
{
  FilterMap { parser, f }
}
