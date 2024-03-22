use crate::{
  core::{
    Contexting,
    Parse,
    Parsed,
    Streaming,
    Success,
  },
  utils::UtilsAtom,
};

/// Implementation of [crate::utils::Utils::span]
#[derive(Clone)]
pub struct Span<Parser> {
  parser: Parser,
}

impl<Stream, Context, Parser> Parse<Stream, Context> for Span<Parser>
where
  Stream: Streaming,
  Context: Contexting<UtilsAtom<Stream>>,
  Parser: Parse<Stream, Context>,
{
  // fix me this sux
  type Token = Success<Parser::Token, Stream::Span>;

  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    match self.parser.parse(stream.clone()) {
      Parsed::Success {
        token,
        stream: stream_success,
      } => match stream.diff(&stream_success) {
        Ok(span) => Parsed::Success {
          token: Success {
            token,
            stream: span,
          },
          stream: stream_success,
        },
        Err(stream) => Parsed::Error(Context::new(UtilsAtom::Diff {
          stream,
          stream_success,
        })),
      },
      Parsed::Failure(context) => Parsed::Failure(context),
      Parsed::Error(context) => Parsed::Error(context),
    }
  }
}

/// Function style version of [crate::utils::Utils::span]
pub fn span<Stream, Context, Parser>(parser: Parser) -> Span<Parser>
where
  Stream: Streaming,
  Context: Contexting<UtilsAtom<Stream>>,
  Parser: Parse<Stream, Context>,
{
  Span { parser }
}
