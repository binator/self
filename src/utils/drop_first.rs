use crate::core::{
  Parse,
  Parsed,
};

/// Implementation of [crate::utils::Utils::drop_first]
#[derive(Clone)]
pub struct DropFirstParser<Parser> {
  parser: Parser,
}

impl<A, B, Stream, Context, Parser> Parse<Stream, Context> for DropFirstParser<Parser>
where
  Parser: Parse<Stream, Context, Token = (A, B)>,
{
  type Token = B;

  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    self.parser.parse(stream).map_token(|(_first, last)| last)
  }
}

/// Function style version of [crate::utils::Utils::drop_first]
pub fn drop_first<A, B, Stream, Context, Parser>(parser: Parser) -> DropFirstParser<Parser>
where
  Parser: Parse<Stream, Context, Token = (A, B)>,
{
  DropFirstParser { parser }
}
