use core::{
  convert::Infallible,
  fmt::Debug,
};

use crate::{
  Contexting,
  Parse,
  Parsed,
  Streaming,
};

/// Always return a Failure
pub fn failure<Stream, Context, Atom>(atom: Atom) -> impl Parse<Stream, Context, Token = Infallible>
where
  Stream: Streaming,
  Atom: Clone + Debug,
  Context: Contexting<Atom>,
{
  Failure { atom }
}

struct Failure<Atom> {
  atom: Atom,
}

impl<Stream, Context, Atom> Parse<Stream, Context> for Failure<Atom>
where
  Stream: Streaming,
  Atom: Clone + Debug,
  Context: Contexting<Atom>,
{
  type Token = Infallible;

  #[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", name = "failure", skip_all, ret(Display))
  )]
  fn parse(&mut self, _stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    Parsed::Failure(Context::new(self.atom.clone()))
  }
}
