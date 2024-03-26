use core::mem::MaybeUninit;

use crate::{
  utils::UtilsAtom,
  Contexting,
  Parse,
  Parsed,
  Streaming,
};

/// Implementation of [crate::utils::Utils::fill]
#[derive(Clone)]
pub struct Fill<Parser, const N: usize> {
  parser: Parser,
}

/// Function style version of [crate::utils::Utils::fill]
pub fn fill<Stream, Context, Parser, const N: usize>(parser: Parser) -> Fill<Parser, N>
where
  Stream: Streaming,
  Context: Contexting<UtilsAtom<Stream>>,
  Parser: Parse<Stream, Context>,
{
  Fill { parser }
}

impl<Stream, Context, Parser, const N: usize> Parse<Stream, Context> for Fill<Parser, N>
where
  Stream: Streaming,
  Context: Contexting<UtilsAtom<Stream>>,
  Parser: Parse<Stream, Context>,
{
  type Token = [Parser::Token; N];

  fn parse(&mut self, mut stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    unsafe {
      let mut tokens: [MaybeUninit<Parser::Token>; N] = MaybeUninit::uninit().assume_init();

      for (i, t) in tokens.iter_mut().enumerate() {
        match self.parser.parse(stream) {
          Parsed::Success {
            token,
            stream: next,
          } => {
            t.write(token);
            stream = next;
          }
          Parsed::Failure(context) => {
            return Parsed::Failure(context.add(UtilsAtom::MinNotReach { i, min: N }));
          }
          Parsed::Error(context) => {
            return Parsed::Error(context);
          }
        }
      }

      // FIXME https://github.com/rust-lang/rust/issues/61956
      let ptr = &mut tokens as *mut _ as *mut [Parser::Token; N];
      let token = ptr.read();
      core::mem::forget(tokens);
      Parsed::Success { token, stream }
    }
  }
}

#[cfg(test)]
mod tests {
  use core::convert::Infallible;

  use derive_more::{
    Display,
    From,
  };

  use super::*;
  use crate::{
    base::any,
    context::{
      Ignore,
      Keep,
      Last,
    },
    utils::Utils,
    CoreAtom,
  };

  type HandleAtom<Stream> = Keep<Last, Context<Stream>>;

  #[derive(Display, Debug, Clone, PartialEq, Eq, From)]
  enum Context<Stream> {
    Fold(UtilsAtom<Stream>),
    Any(CoreAtom<Stream, Infallible>),
    Stream,
  }

  #[test]
  fn fill_array() {
    let stream = &b"abcdefghijklmnopqrstuvwxyz"[..];
    let result: Parsed<[u8; 26], _, HandleAtom<_>> = any.fill().parse(stream);
    let expected = Parsed::Success {
      token: *b"abcdefghijklmnopqrstuvwxyz",
      stream: &b""[..],
    };
    assert_eq!(result, expected);
  }

  #[test]
  fn fill_integer() {
    let stream = &[0x12, 0x34, 0x56, 0x78][..];
    let result: Parsed<_, _, Ignore> = any.fill().map(u32::from_be_bytes).parse(stream);
    let expected = Parsed::Success {
      token: 0x12345678,
      stream: &b""[..],
    };
    assert_eq!(result, expected);
  }
}
