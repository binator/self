use crate::{
  Parse,
  Parsed,
};

/// Implementation of [crate::utils::Utils::opt]
#[derive(Clone)]
pub struct Optional<Parser> {
  parser: Parser,
}

impl<Stream, Context, Parser> Parse<Stream, Context> for Optional<Parser>
where
  Stream: Clone,
  Parser: Parse<Stream, Context>,
{
  type Token = Option<Parser::Token>;

  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    match self.parser.parse(stream.clone()) {
      Parsed::Success { token, stream } => Parsed::Success {
        token: Some(token),
        stream,
      },
      Parsed::Failure(_context) => Parsed::Success {
        token: None,
        stream,
      },
      Parsed::Error(context) => Parsed::Error(context),
    }
  }
}

/// Function style version of [crate::utils::Utils::opt]
pub fn opt<Stream, Context, Parser>(parser: Parser) -> Optional<Parser>
where
  Stream: Clone,
  Parser: Parse<Stream, Context>,
{
  Optional { parser }
}
