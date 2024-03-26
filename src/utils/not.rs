use crate::{
  Contexting,
  Parse,
  Parsed,
};

/// Implementation of [crate::utils::Utils::not]
#[derive(Clone)]
pub struct Not<Parser> {
  parser: Parser,
}

/// Not Atom
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotAtom<Token, Stream> {
  /// Contains Success from the inner parser
  ParserParsedSuccess(Token, Stream),
  /// When inner parser Error
  ParserParsedInvalid,
}

impl<Stream, Context, Parser> Parse<Stream, Context> for Not<Parser>
where
  Stream: Clone,
  Context: Contexting<NotAtom<Parser::Token, Stream>>,
  Parser: Parse<Stream, Context>,
{
  type Token = Context;

  fn parse(&mut self, stream: Stream) -> Parsed<Context, Stream, Context> {
    match self.parser.parse(stream.clone()) {
      Parsed::Success { token, stream } => {
        Parsed::Failure(Context::new(NotAtom::ParserParsedSuccess(token, stream)))
      }
      Parsed::Failure(context) => Parsed::Success {
        token: context,
        stream,
      },
      Parsed::Error(context) => Parsed::Error(context.add(NotAtom::ParserParsedInvalid)),
    }
  }
}

// evil
/// Function style version of [crate::utils::Utils::not]
pub fn not<Stream, Context, Parser>(parser: Parser) -> Not<Parser>
where
  Stream: Clone,
  Context: Contexting<NotAtom<Parser::Token, Stream>>,
  Parser: Parse<Stream, Context>,
{
  Not { parser }
}
