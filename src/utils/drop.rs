use crate::{
  Parse,
  Parsed,
};

/// Implementation of [crate::utils::Utils::drop]
#[derive(Clone)]
pub struct Drop<Parser> {
  parser: Parser,
}

impl<Stream, Context, Parser> Parse<Stream, Context> for Drop<Parser>
where
  Parser: Parse<Stream, Context>,
{
  type Token = ();

  fn parse(&mut self, stream: Stream) -> Parsed<(), Stream, Context> {
    self.parser.parse(stream).map_token(|_| ())
  }
}

/// Function style version of [crate::utils::Utils::drop]
pub fn drop<Stream, Context, Parser>(parser: Parser) -> Drop<Parser>
where
  Parser: Parse<Stream, Context>,
{
  Drop { parser }
}
