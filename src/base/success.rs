use core::fmt::Debug;

use crate::{
  Parse,
  Parsed,
  ProvideElement,
  Streaming,
};

/// Take a token in parameter and return a Parser that don't
/// read the Stream and always return Success by Cloning the token.
pub fn success<Token, Stream, Context>(token: Token) -> impl Parse<Stream, Context, Token = Token>
where
  Stream: Streaming,
  Token: Clone + Debug,
  Context: ProvideElement,
{
  Success { token }
}

struct Success<Token> {
  token: Token,
}

impl<Token, Stream, Context> Parse<Stream, Context> for Success<Token>
where
  Stream: Streaming,
  Token: Clone + Debug,
  Context: ProvideElement,
{
  type Token = Token;

  #[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", name = "success", skip_all, ret(Display))
  )]
  fn parse(&mut self, stream: Stream) -> Parsed<Token, Stream, Context> {
    Parsed::Success {
      token: self.token.clone(),
      stream,
    }
  }
}
