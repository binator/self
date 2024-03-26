use crate::{
  utils::UtilsAtom,
  Contexting,
  Parse,
  Parsed,
  Streaming,
};

/// Implementation of [crate::utils::Utils::fold_until]
#[derive(Clone)]
pub struct FoldUntil<Parser, Until, Init, F> {
  parser: Parser,
  until: Until,
  init: Init,
  f: F,
}

/// Function style version of [crate::utils::Utils::fold_until]
pub fn fold_until<Stream, Context, Acc, Parser, Until, Init, F>(
  parser: Parser, until: Until, init: Init, f: F,
) -> FoldUntil<Parser, Until, Init, F>
where
  Stream: Streaming,
  Context: Contexting<UtilsAtom<Stream>>,
  Parser: Parse<Stream, Context>,
  Until: Parse<Stream, Context>,
  Init: FnMut() -> Acc,
  F: FnMut(Acc, Parser::Token) -> Acc,
{
  FoldUntil {
    parser,
    until,
    init,
    f,
  }
}

impl<Stream, Context, Parser, Until, Acc, Init, F> Parse<Stream, Context>
  for FoldUntil<Parser, Until, Init, F>
where
  Stream: Streaming,
  Context: Contexting<UtilsAtom<Stream>>,
  Parser: Parse<Stream, Context>,
  Until: Parse<Stream, Context>,
  Init: FnMut() -> Acc,
  F: FnMut(Acc, Parser::Token) -> Acc,
{
  type Token = (Acc, Until::Token);

  fn parse(&mut self, mut stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    let mut acc = (self.init)();
    loop {
      match self.until.parse(stream.clone()) {
        Parsed::Success { token, stream } => {
          break Parsed::Success {
            token: (acc, token),
            stream,
          };
        }
        Parsed::Failure(until_context) => match self.parser.parse(stream) {
          Parsed::Success {
            token,
            stream: next,
          } => {
            acc = (self.f)(acc, token);
            stream = next;
          }
          Parsed::Failure(context) => {
            break Parsed::Failure(context.bitor(until_context).add(UtilsAtom::UntilNotReach));
          }
          Parsed::Error(context) => {
            break Parsed::Error(context);
          }
        },
        Parsed::Error(context) => {
          break Parsed::Error(context);
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {}
