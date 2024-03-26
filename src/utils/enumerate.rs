use core::mem;

use crate::{
  Parse,
  Parsed,
};

/// Implementation of [crate::utils::Utils::enumerate]
#[derive(Clone)]
pub struct Enumerate<Parser> {
  parser: Parser,
  i: usize,
}

impl<Stream, Context, Parser> Parse<Stream, Context> for Enumerate<Parser>
where
  Parser: Parse<Stream, Context>,
{
  type Token = (usize, Parser::Token);

  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    self.parser.parse(stream).map_token(|token| {
      let next = self.i + 1;
      (mem::replace(&mut self.i, next), token)
    })
  }
}

/// Function style version of [crate::utils::Utils::enumerate]
pub fn enumerate<Stream, Context, Parser>(parser: Parser) -> Enumerate<Parser>
where
  Parser: Parse<Stream, Context>,
{
  Enumerate { parser, i: 0 }
}
