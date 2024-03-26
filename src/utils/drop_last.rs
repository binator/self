use crate::{
  Parse,
  Parsed,
};

/// Implementation of [crate::utils::Utils::drop_first]
#[derive(Clone)]
pub struct DropLastParser<Parser> {
  parser: Parser,
}

impl<Stream, A, B, Context, Parser> Parse<Stream, Context> for DropLastParser<Parser>
where
  Parser: Parse<Stream, Context, Token = (A, B)>,
{
  type Token = A;

  fn parse(&mut self, stream: Stream) -> Parsed<A, Stream, Context> {
    self.parser.parse(stream).map_token(|(first, _last)| first)
  }
}

/// Function style version of [crate::utils::Utils::drop_last]
pub fn drop_last<Stream, A, B, Context, Parser>(parser: Parser) -> DropLastParser<Parser>
where
  Parser: Parse<Stream, Context, Token = (A, B)>,
{
  DropLastParser { parser }
}
