use crate::{
  Contexting,
  Parse,
  Parsed,
};

/// Implementation of [crate::utils::Utils::add_atom]
#[derive(Clone)]
pub struct AddAtom<Parser, F> {
  parser: Parser,
  f: F,
}

impl<Stream, Context, Parser, F, Atom> Parse<Stream, Context> for AddAtom<Parser, F>
where
  Parser: Parse<Stream, Context>,
  F: Fn() -> Atom,
  Context: Contexting<Atom>,
{
  type Token = Parser::Token;

  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    self
      .parser
      .parse(stream)
      .map_context(|context| context.add((self.f)()))
  }
}

/// Function style version of [crate::utils::Utils::add_atom]
pub fn add_atom<Stream, Context, Parser, F, Atom>(parser: Parser, f: F) -> AddAtom<Parser, F>
where
  Parser: Parse<Stream, Context>,
  F: Fn() -> Atom,
  Context: Contexting<Atom>,
{
  AddAtom { parser, f }
}
