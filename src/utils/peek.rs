use crate::core::{
  Parse,
  Parsed,
  Success,
};

/// Implementation of [crate::utils::Utils::peek]
#[derive(Clone)]
pub struct Peek<Parser> {
  parser: Parser,
}

impl<Stream, Context, Parser> Parse<Stream, Context> for Peek<Parser>
where
  Stream: Clone,
  Parser: Parse<Stream, Context>,
{
  type Token = (Parser::Token, Stream);

  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    self
      .parser
      .parse(stream.clone())
      .map_success(|success| Success {
        token: (success.token, success.stream),
        stream,
      })
  }
}

/// Function style version of [crate::utils::Utils::peek]
pub fn peek<Stream, Context, Parser>(parser: Parser) -> Peek<Parser>
where
  Stream: Clone,
  Parser: Parse<Stream, Context>,
{
  Peek { parser }
}
