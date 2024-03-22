use crate::{
  core::{
    Contexting,
    Parse,
    Parsed,
  },
  utils::UtilsAtom,
};

/// Implementation of [crate::utils::Utils::filter]
#[derive(Clone)]
pub struct Filter<Parser, F> {
  parser: Parser,
  f: F,
}

impl<Stream, Context, Parser, F> Parse<Stream, Context> for Filter<Parser, F>
where
  Parser: Parse<Stream, Context>,
  F: Fn(&Parser::Token) -> bool,
  Context: Contexting<UtilsAtom<Stream>>,
{
  type Token = Parser::Token;

  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    let success = self.parser.parse(stream)?;
    if (self.f)(&success.token) {
      success.into()
    } else {
      Parsed::new_failure(Context::new(UtilsAtom::Filter))
    }
  }
}

/// Function style version of [crate::utils::Utils::filter]
pub fn filter<Stream, Context, Parser, F>(parser: Parser, f: F) -> Filter<Parser, F>
where
  Parser: Parse<Stream, Context>,
  F: Fn(&Parser::Token) -> bool,
  Context: Contexting<UtilsAtom<Stream>>,
{
  Filter { parser, f }
}
