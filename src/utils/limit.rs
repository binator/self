use crate::{
  core::{
    Contexting,
    Parse,
    Parsed,
  },
  utils::UtilsAtom,
};

/// Implementation of [crate::utils::Utils::limit]
#[derive(Clone)]
pub struct Limit<Parser> {
  parser: Parser,
  i: usize,
  n: usize,
}

impl<Stream, Context, Parser> Parse<Stream, Context> for Limit<Parser>
where
  Parser: Parse<Stream, Context>,
  Context: Contexting<UtilsAtom<Stream>>,
{
  type Token = Parser::Token;

  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    if self.i < self.n {
      self.i += 1;
      self.parser.parse(stream)
    } else {
      Parsed::Failure(Context::new(UtilsAtom::Max(self.n)))
    }
  }
}

/// Function style version of [crate::utils::Utils::limit]
pub fn limit<Stream, Context, Parser>(parser: Parser, n: usize) -> Limit<Parser>
where
  Parser: Parse<Stream, Context>,
  Context: Contexting<UtilsAtom<Stream>>,
{
  Limit { parser, i: 0, n }
}
