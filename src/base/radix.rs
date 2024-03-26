use core::{
  fmt::{
    self,
    Debug,
    Display,
    Formatter,
  },
  marker::PhantomData,
};

use num_traits::{
  cast::AsPrimitive,
  identities::Zero,
  ops::checked::{
    CheckedAdd,
    CheckedMul,
  },
  sign::{
    Signed,
    Unsigned,
  },
  CheckedSub,
};

use crate::{
  base::{
    octet,
    sign,
    BaseAtom,
    Sign,
  },
  utils::{
    TryFoldBoundsParse,
    Utils,
    UtilsAtom,
  },
  Contexting,
  CoreAtom,
  Parse,
  Parsed,
  Streaming,
  Success,
};

/// Represent Radix, used to limit radix <= 36
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Radix {
  radix: u8,
}

impl Display for Radix {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.radix)
  }
}

impl Radix {
  /// Binary Radix
  pub const BIN: Self = Self { radix: 2 };
  /// Decimal Radix
  pub const DEC: Self = Self { radix: 10 };
  /// Hexadecimal Radix
  pub const HEX: Self = Self { radix: 16 };
  /// Octal Radix
  pub const OCTAL: Self = Self { radix: 8 };

  /// Return a Radix if radix <= 36
  pub const fn new(radix: u8) -> Result<Self, u8> {
    if radix <= 36 {
      Ok(Self { radix })
    } else {
      Err(radix)
    }
  }
}

impl From<Radix> for u8 {
  fn from(radix: Radix) -> Self {
    radix.radix
  }
}

/// Atom context for uint_radix
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntRadixAtom<Token> {
  /// if Parser encounter a no digit character when it expect one
  NotADigit {
    /// character found in the stream
    found: char,
    /// radix used
    radix: Radix,
  },
  /// if the number parsed would overflow the integer returned
  Overflow {
    /// sign
    sign: Option<Sign>,
    /// last digit found
    to_digit: u8,
    /// last valid value before overflow
    acc: Token,
    /// radix used
    radix: Radix,
  },
}

impl<Token> Display for IntRadixAtom<Token>
where
  Token: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      IntRadixAtom::NotADigit { found, radix } => {
        write!(f, "IntRadix: Not a to_digit {} radix {}", found, radix)
      }
      IntRadixAtom::Overflow {
        sign,
        to_digit,
        acc,
        radix,
      } => {
        write!(
          f,
          "IntRadix: Overflow sign {sign:?} acc {acc} last to_digit {to_digit} radix {radix}"
        )
      }
    }
  }
}

struct UIntRadixParser<Token, Bounds> {
  bounds: Bounds,
  radix: Radix,
  token: PhantomData<Token>,
}

struct IntRadixParser<Token, Bounds>(UIntRadixParser<Token, Bounds>);

/// Meta trait for int_radix
pub trait IntRadixParse<Stream, Context, Token: 'static> = where
  Stream: Streaming,
  <Stream as Streaming>::Item: Into<u8>,
  Token: CheckedAdd + CheckedMul + CheckedSub + Zero + Copy + Debug,
  Context: Contexting<IntRadixAtom<Token>>,
  Context: Contexting<BaseAtom<u8>>,
  Context: Contexting<CoreAtom<Stream>>,
  Context: Contexting<UtilsAtom<Stream>>,
  u8: AsPrimitive<Token>;

/// Take a bounds in parameter and a radix and return a Parser
/// that will parse a integer from the stream.
/// Will check for sign character
pub fn int_radix<Token: 'static, Stream, Context, Bounds>(
  bounds: Bounds, radix: Radix,
) -> impl Parse<Stream, Context, Token = Token>
where
  (): IntRadixParse<Stream, Context, Token>,
  Bounds: TryFoldBoundsParse + Clone,
  Token: Signed,
{
  IntRadixParser(UIntRadixParser {
    bounds,
    radix,
    token: PhantomData::default(),
  })
}

/// Take a bounds in parameter and a radix and return a Parser
/// that will parse a unsigned integer from the stream.
pub fn uint_radix<Token: 'static, Stream, Context, Bounds>(
  bounds: Bounds, radix: Radix,
) -> impl Parse<Stream, Context, Token = Token>
where
  (): IntRadixParse<Stream, Context, Token>,
  Bounds: TryFoldBoundsParse + Clone,
  Token: Unsigned,
{
  UIntRadixParser {
    bounds,
    radix,
    token: PhantomData::default(),
  }
}

impl<Token: 'static, Stream, Context, Bounds> Parse<Stream, Context>
  for IntRadixParser<Token, Bounds>
where
  (): IntRadixParse<Stream, Context, Token>,
  Bounds: TryFoldBoundsParse + Clone,
  Token: Signed,
{
  type Token = Token;

  #[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", name = "uint_radix", skip_all, ret(Display))
  )]
  fn parse(&mut self, stream: Stream) -> Parsed<Token, Stream, Context> {
    let Success {
      token: sign,
      stream,
    } = sign.opt().parse(stream)?;

    match sign {
      Some(Sign::Neg) => octet
        .try_map(|c| {
          let c = char::from(c);
          c.to_digit(u8::from(self.0.radix) as u32)
            .map(|d| d as u8)
            .ok_or_else(|| {
              Context::new(IntRadixAtom::NotADigit {
                found: c,
                radix: self.0.radix,
              })
            })
        })
        .try_fold_bounds(
          self.0.bounds.clone(),
          || Ok(Token::zero()),
          |acc, d| {
            if let Some(acc) = acc
              .checked_mul(&u8::from(self.0.radix).as_())
              .and_then(|acc| acc.checked_sub(&d.as_()))
            {
              Ok(acc)
            } else {
              Err(Context::new(IntRadixAtom::Overflow {
                sign,
                to_digit: d,
                acc,
                radix: self.0.radix,
              }))
            }
          },
        )
        .parse(stream),
      _ => self.0.parse(stream),
    }
  }
}

impl<Token: 'static, Stream, Context, Bounds> Parse<Stream, Context>
  for UIntRadixParser<Token, Bounds>
where
  (): IntRadixParse<Stream, Context, Token>,
  Bounds: TryFoldBoundsParse + Clone,
{
  type Token = Token;

  #[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", name = "uint_radix", skip_all, ret(Display))
  )]
  fn parse(&mut self, stream: Stream) -> Parsed<Token, Stream, Context> {
    octet
      .try_map(|c| {
        let c = char::from(c);
        c.to_digit(u8::from(self.radix) as u32)
          .map(|d| d as u8)
          .ok_or_else(|| {
            Context::new(IntRadixAtom::NotADigit {
              found: c,
              radix: self.radix,
            })
          })
      })
      .try_fold_bounds(
        self.bounds.clone(),
        || Ok(Token::zero()),
        |acc, d| {
          if let Some(acc) = acc
            .checked_mul(&u8::from(self.radix).as_())
            .and_then(|acc| acc.checked_add(&d.as_()))
          {
            Ok(acc)
          } else {
            Err(Context::new(IntRadixAtom::Overflow {
              sign: None,
              to_digit: d,
              acc,
              radix: self.radix,
            }))
          }
        },
      )
      .parse(stream)
  }
}

#[cfg(test)]
mod tests {
  use core::{
    convert::Infallible,
    mem::discriminant,
  };

  use derive_more::{
    Display,
    From,
  };
  use rand::Rng;

  use super::{
    int_radix,
    uint_radix,
    IntRadixAtom,
    Radix,
  };
  use crate::{
    base::BaseAtom,
    context::Tree,
    utils::UtilsAtom,
    CoreAtom,
    Parse,
    Parsed,
    Streaming,
  };

  #[derive(Display, Debug, Clone, From)]
  enum Context<Stream: Streaming> {
    UInt8(IntRadixAtom<u8>),
    UInt64(IntRadixAtom<u64>),
    Int8(IntRadixAtom<i8>),
    Int64(IntRadixAtom<i64>),
    Any(CoreAtom<Stream, Infallible>),
    Utils(UtilsAtom<Stream>),
    Is(BaseAtom<u8>),
  }

  impl<Stream: Streaming> PartialEq for Context<Stream> {
    fn eq(&self, other: &Self) -> bool {
      discriminant(self) == discriminant(other)
    }
  }

  type HandleAtom<Stream> = Tree<Context<Stream>>;

  fn test_uint_radix(n: u64, radix: Radix) {
    let stream = n.to_string();
    let stream = stream.as_bytes();

    let result: Parsed<_, _, HandleAtom<_>> = uint_radix(.., radix).parse(stream);
    let expected = Parsed::Success {
      token: n,
      stream: "".as_bytes(),
    };
    println!("{:#?}", result);
    assert_eq!(result, expected);
  }

  fn test_int_radix(n: i64, radix: Radix) {
    let stream = n.to_string();
    let stream = stream.as_bytes();

    let result: Parsed<_, _, HandleAtom<_>> = int_radix(.., radix).parse(stream);
    let expected = Parsed::Success {
      token: n,
      stream: "".as_bytes(),
    };
    println!("{:#?}", result);
    assert_eq!(result, expected);
  }

  fn int_str(stream: &str, radix: Radix) {
    let n = i8::from_str_radix(stream, u8::from(radix) as u32).unwrap();

    let result: Parsed<_, _, HandleAtom<_>> = int_radix(.., radix).parse(stream.as_bytes());
    let expected = Parsed::Success {
      token: n,
      stream: "".as_bytes(),
    };

    assert_eq!(result, expected);
  }

  fn uint_str(stream: &str, radix: Radix) {
    let n = u8::from_str_radix(stream, u8::from(radix) as u32).unwrap();

    let result: Parsed<_, _, HandleAtom<_>> = uint_radix(.., radix).parse(stream.as_bytes());
    let expected = Parsed::Success {
      token: n,
      stream: "".as_bytes(),
    };

    assert_eq!(result, expected);
  }

  #[test]
  fn int_radix_simple() {
    int_str("0", Radix::DEC);
    int_str("42", Radix::DEC);
    int_str("+84", Radix::DEC);
    int_str("127", Radix::DEC);
    int_str("-0", Radix::DEC);
    int_str("-42", Radix::DEC);
    int_str("-84", Radix::DEC);
    int_str("-128", Radix::DEC);

    int_str("0", Radix::HEX);
    int_str("+F", Radix::HEX);
    int_str("7F", Radix::HEX);
    int_str("-0", Radix::HEX);
    int_str("-F", Radix::HEX);
    int_str("-80", Radix::HEX);

    int_str("0", Radix::BIN);
    int_str("+10", Radix::BIN);
    int_str("11", Radix::BIN);
    int_str("-0", Radix::BIN);
    int_str("-10", Radix::BIN);
    int_str("-11", Radix::BIN);

    int_str("0", Radix::OCTAL);
    int_str("+7", Radix::OCTAL);
    int_str("17", Radix::OCTAL);
    int_str("-0", Radix::OCTAL);
    int_str("-7", Radix::OCTAL);
    int_str("-17", Radix::OCTAL);
  }

  #[test]
  fn uint_radix_simple() {
    uint_str("0", Radix::DEC);
    uint_str("42", Radix::DEC);
    uint_str("84", Radix::DEC);
    uint_str("168", Radix::DEC);

    uint_str("0", Radix::HEX);
    uint_str("F", Radix::HEX);
    uint_str("FF", Radix::HEX);

    uint_str("0", Radix::BIN);
    uint_str("10", Radix::BIN);
    uint_str("11", Radix::BIN);

    uint_str("0", Radix::OCTAL);
    uint_str("7", Radix::OCTAL);
    uint_str("17", Radix::OCTAL);
  }

  #[test]
  fn int_radix_overflow() {
    let stream = "128".as_bytes();

    let result: Parsed<i8, _, HandleAtom<_>> = int_radix(.., Radix::DEC).parse(stream);
    assert!(!matches!(result, Parsed::Success { .. }));
  }

  #[test]
  fn uint_radix_overflow() {
    let stream = "256".as_bytes();

    let result: Parsed<u8, _, HandleAtom<_>> = uint_radix(.., Radix::DEC).parse(stream);
    assert!(!matches!(result, Parsed::Success { .. }));
  }

  #[test]
  fn uint_radix_not_a_digit() {
    let stream = "Don't you know about the bird ?".as_bytes();

    let result: Parsed<u8, _, HandleAtom<_>> = uint_radix(1.., Radix::DEC).parse(stream);
    assert!(!matches!(result, Parsed::Success { .. }));
  }

  #[test]
  fn uint_radix_bound() {
    let stream = "42 is the answer".as_bytes();

    let result: Parsed<u8, _, HandleAtom<_>> = uint_radix(..2, Radix::DEC).parse(stream);
    let expected = Parsed::Success {
      token: 42,
      stream: " is the answer".as_bytes(),
    };

    assert_eq!(result, expected);
  }

  #[test]
  //  #[ignore]
  fn uint_radix_random() {
    let mut rng = rand::thread_rng();
    let mut buf = [0; 8];

    for _ in 0..42_000 {
      rng.fill(&mut buf);
      let n = u64::from_ne_bytes(buf);
      test_uint_radix(n, Radix::DEC);
    }
  }

  #[test]
  //  #[ignore]
  fn int_radix_random() {
    let mut rng = rand::thread_rng();
    let mut buf = [0; 8];

    for _ in 0..42_000 {
      rng.fill(&mut buf);
      let n = i64::from_ne_bytes(buf);
      test_int_radix(n, Radix::DEC);
    }
  }
}
