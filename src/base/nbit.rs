use crate::{
  base::octet,
  utils::Utils,
  Contexting,
  CoreAtom,
  Parse,
  Streaming,
};

/// Used by nbit to represent n only if 0 > n > 8
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NBit {
  n: u8,
}

impl NBit {
  /// n = 5
  pub const FIVE: Self = Self { n: 5 };
  /// n = 4
  pub const FOUR: Self = Self { n: 4 };
  /// n = 1
  pub const ONE: Self = Self { n: 1 };
  /// n = 7
  pub const SEVEN: Self = Self { n: 7 };
  /// n = 6
  pub const SIX: Self = Self { n: 6 };
  /// n = 3
  pub const THREE: Self = Self { n: 3 };
  /// n = 2
  pub const TWO: Self = Self { n: 2 };

  /// Return a new NBit if n is valid
  pub const fn new(n: u8) -> Result<Self, u8> {
    if n > 0 && n < 8 {
      Ok(Self { n })
    } else {
      Err(n)
    }
  }
}

// this is MEH
/// Return a Parser that will split an octet in two into a tuple of octet.
/// For n = 3, 0b11000011u8 will give (0b00011000u8, 0b00000011u8)
pub fn nbit<Stream, Context>(n: NBit) -> impl Parse<Stream, Context, Token = (u8, u8)>
where
  Stream: Streaming,
  Stream::Item: Into<u8>,
  Context: Contexting<CoreAtom<Stream, Stream::Error>>,
{
  n
}

impl<Stream, Context> Parse<Stream, Context> for NBit
where
  Stream: Streaming,
  Stream::Item: Into<u8>,
  Context: Contexting<CoreAtom<Stream, Stream::Error>>,
{
  type Token = (u8, u8);

  #[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", name = "nbit", skip_all, ret(Display))
  )]
  fn parse(&mut self, stream: Stream) -> crate::Parsed<Self::Token, Stream, Context> {
    octet
      .map(|b| (b >> self.n, b & u8::MAX >> (8 - self.n)))
      .parse(stream)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    context::*,
    *,
  };

  #[test]
  fn nbit_simple() {
    let stream = 0b11000011u8.to_ne_bytes();
    assert_eq!(
      nbit::<_, Ignore>(NBit::THREE).parse(stream.as_slice()),
      Parsed::Success {
        token: (0b00011000u8, 0b00000011u8),
        stream: "".as_bytes(),
      }
    );
  }
}
