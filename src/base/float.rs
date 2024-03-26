use core::{
  fmt::{
    self,
    Debug,
    Display,
    Formatter,
  },
  str::FromStr,
};

use crate::{
  base::{
    ascii::AsciiParse,
    is,
    sign,
    tag_no_case,
    to_digit,
    BaseAtom,
  },
  utils::{
    Acc,
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

/// Information about float failure
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FloatAtom {
  /// Number parser couldn't recognize a number
  Number,
  /// Number inf parser couldn't recognize inf
  Inf,
  /// Number nan parser couldn't recognize NaN
  NaN,
  /// Should not happen, contact dev if this is returned
  /// Could be remove one day
  Bug,
}

impl Display for FloatAtom {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      FloatAtom::Number => write!(f, "Float: Number"),
      FloatAtom::Inf => write!(f, "Float: Inf"),
      FloatAtom::NaN => write!(f, "Float: NaN"),
      FloatAtom::Bug => write!(f, "Float: Bug"),
    }
  }
}

/// Meta trait for float
pub trait FloatParse<Stream, Context> = AsciiParse<Stream, Context>
where
  Stream: Streaming,
  <Stream as Streaming>::Item: Into<u8>,
  <Stream as Streaming>::Span: AsRef<[u8]>,
  Context: Contexting<FloatAtom>,
  Context: Contexting<UtilsAtom<Stream>>,
  Context: Contexting<BaseAtom<u8>>,
  Context: Contexting<CoreAtom<Stream>>;

/// Float  ::= Sign? ( 'inf' | 'NaN' | Number )
#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "trace", skip_all, ret(Display))
)]
pub fn float<Token, Stream, Context>(stream: Stream) -> Parsed<Token, Stream, Context>
where
  (): FloatParse<Stream, Context>,
  Token: FromStr + Debug,
{
  let Success {
    token: Success { stream: float, .. },
    stream,
  } = sign
    .opt()
    .and(
      number
        .or(tag_no_case("nan").drop().add_atom(|| FloatAtom::NaN))
        .or(tag_no_case("inf").drop().add_atom(|| FloatAtom::Inf)),
    )
    .span()
    .parse(stream)?;

  let float = unsafe { core::str::from_utf8_unchecked(float.as_ref()) };

  if let Ok(float) = Token::from_str(float) {
    Parsed::Success {
      token: float,
      stream,
    }
  } else {
    Parsed::Failure(Context::new(FloatAtom::Bug))
  }
}

// Number ::= ( Digit+ | Digit+ '.' Digit* | Digit* '.' Digit+ ) Exp?
// Number ::= ( Digit+ ( '.' Digit* )? | '.' Digit+ ) Exp?
#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "trace", skip_all, ret(Display))
)]
fn number<Stream, Context>(stream: Stream) -> Parsed<(), Stream, Context>
where
  Stream: FloatParse<Stream, Context>,
{
  to_digit
    .drop()
    .fold_bounds(1.., || (), Acc::acc)
    .and(
      is(b'.')
        .and(to_digit.drop().fold_bounds(.., || (), Acc::acc))
        .opt(),
    )
    .drop()
    .or(
      is(b'.')
        .and(to_digit.drop().fold_bounds(1.., || (), Acc::acc))
        .drop(),
    )
    .and(exp.opt())
    .drop()
    .parse(stream)
    .map_context(|context| context.add(FloatAtom::Number))
}

// Exp    ::= [eE] Sign? Digit+
#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "trace", skip_all, ret(Display))
)]
fn exp<Stream, Context>(stream: Stream) -> Parsed<(), Stream, Context>
where
  Stream: FloatParse<Stream, Context>,
{
  let Success { token: _, stream } = is(b'e').or(is(b'E')).parse(stream)?;

  let Success { token: _, stream } = sign.opt().parse(stream)?;

  let Success { token: _, stream } = to_digit
    .drop()
    .fold_bounds(1.., || (), Acc::acc)
    .opt()
    .parse(stream)?;

  Parsed::Success { token: (), stream }
}

#[cfg(test)]
mod tests {
  use core::{
    convert::Infallible,
    mem::discriminant,
  };
  use std::str::FromStr;

  use derive_more::{
    Display,
    From,
  };
  use rand::Rng;
  use test_log::test;

  use super::{
    float,
    FloatAtom,
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
    Float(FloatAtom),
    Core(CoreAtom<Stream, Infallible>),
    Utils(UtilsAtom<Stream>),
    Base(BaseAtom<u8>),
  }

  impl<Stream: Streaming> PartialEq for Context<Stream> {
    fn eq(&self, other: &Self) -> bool {
      discriminant(self) == discriminant(other)
    }
  }

  type HandleAtom<Stream> = Tree<Context<Stream>>;

  fn test_float(f: f64) {
    let stream = f.to_string();
    let stream = stream.as_bytes();

    let result: Parsed<_, _, HandleAtom<_>> = float.parse(stream);
    let expected = Parsed::Success {
      token: f,
      stream: "".as_bytes(),
    };
    println!("{:#?}", result);
    assert_eq!(result, expected);
  }

  fn test_str(stream: &str) {
    let f = f64::from_str(stream).unwrap();

    let result: Parsed<_, _, HandleAtom<_>> = float.parse(stream.as_bytes());
    let expected = Parsed::Success {
      token: f,
      stream: "".as_bytes(),
    };

    assert_eq!(result, expected);
  }

  #[test]
  fn float_simple() {
    test_str("42.42");
    test_str("42.");
    test_str(".42");
    test_str("0000000000042.");
    test_str(".4200000000000");
  }

  #[test]
  fn float_nan() {
    let stream = f64::NAN.to_string();
    let stream = stream.as_bytes();

    let result: Parsed<f64, _, HandleAtom<_>> = float.parse(stream);
    assert!(result.unwrap().token.is_nan());

    let result: Parsed<f64, _, HandleAtom<_>> = float.parse("nAn".as_bytes());
    assert!(result.unwrap().token.is_nan());

    let stream = "Na";
    let result: Parsed<f64, _, HandleAtom<_>> = float.parse(stream.as_bytes());

    println!("{}", result.as_ref().unwrap_context());

    assert!(!matches!(result, Parsed::Success { .. }));
  }

  #[test]
  fn float_infite() {
    let stream = f64::INFINITY.to_string();
    let stream = stream.as_bytes();

    let result: Parsed<f64, _, HandleAtom<_>> = float.parse(stream);
    assert!(result.unwrap().token.is_infinite());

    let result: Parsed<f64, _, HandleAtom<_>> = float.parse("INF".as_bytes());
    assert!(result.unwrap().token.is_infinite());
  }

  #[test]
  #[ignore]
  fn float_random() {
    let mut rng = rand::thread_rng();
    let mut buf = [0; 8];

    for _ in 0..42_000 {
      rng.fill(&mut buf);
      let f = f64::from_ne_bytes(buf);
      if f.is_finite() {
        test_float(f);
      }
    }
  }
}
