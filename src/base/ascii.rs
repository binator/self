//! ASCII Core Rules (RFC5234 B.1.)

use stdcore::fmt::{
  self,
  Display,
  Formatter,
};

use crate::{
  base::*,
  core::*,
  utils::*,
};

/// Meta trait for ascii combinator
pub trait AsciiParse<Stream, Context> = where
  Stream: Streaming,
  <Stream as Streaming>::Item: Into<u8>,
  Context: Contexting<CoreAtom<Stream>>,
  Context: Contexting<BaseAtom<u8>>,
  Context: Contexting<UtilsAtom<Stream>>;

macro_rules! base_wrapper {
  ($doc:meta, $camel:ident, $lower:ident, $pat:pat, $context:ident) => {
    #[doc=stringify!($camel)]
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct $camel(u8);

    impl $camel {
      /// Return a new
      #[doc=stringify!($camel)]
      /// if value of c is valid
      pub const fn new(c: u8) -> Option<Self> {
        match c {
          $pat => Some(Self(c)),
          _ => None,
        }
      }
    }

    impl From<$camel> for char {
      fn from($lower: $camel) -> Self {
        $lower.0 as char
      }
    }

    impl From<$camel> for u8 {
      fn from($lower: $camel) -> Self {
        $lower.0
      }
    }

    #[$doc]
    pub fn $lower<Stream, Context>(stream: Stream) -> Parsed<$camel, Stream, Context>
    where
      (): AsciiParse<Stream, Context>,
    {
      octet
        .try_map(|c| {
          $camel::new(c).ok_or_else(|| {
            Context::new(BaseAtom::Ascii {
              found: c,
              expected: stringify!($pat),
            })
          })
        })
        .parse(stream)
    }
  };
}

base_wrapper!(doc = "ALPHA = A - Z / a - z", Alpha, alpha, b'A'..=b'Z' | b'a'..=b'z', AlphaAtom);
base_wrapper!(doc = "ALPHANUM = A - Z / a - z / 0 - 9", AlphaNum, alphanum, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9', AlphaNumAtom);
base_wrapper!(doc = "BIT = 0 / 1", Bit, bit, b'0' | b'1', BitAtom);
base_wrapper!(
  doc = "CHAR = 0x01 - 0x7F",
  Char,
  char,
  b'\x01'..=b'\x7F',
  CharAtom
);
// fixme no need wrapper
base_wrapper!(doc = "CR = \\r", Cr, cr, b'\r', CrAtom);
base_wrapper!(
  doc = "CTL = 0x00 - 0x1F / 0x7F",
  Ctl,
  ctl,
  b'\x00'..=b'\x1F' | b'\x7F',
  CtlAtom
);
// fixme no need wrapper
base_wrapper!(doc = "DQUOTE = \"", DQuote, dquote, b'"', DQuoteAtom);
base_wrapper!(
  doc = "HEXDIG = DIGIT / A - F / a - f",
  HexDig,
  hexdig,
  b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f',
  HexDigAtom);
// fixme no need wrapper
base_wrapper!(doc = "HTAB = \\t", HTab, htab, b'\t', HTabAtom);
// fixme no need wrapper
base_wrapper!(doc = "LF = \\n", Lf, lf, b'\n', LfAtom);
base_wrapper!(doc = "WSP = SP / HTAB", Wsp, wsp, b' ' | b'\t', WspAtom);
// fixme no need wrapper
base_wrapper!(doc = "SP = ' '", Sp, sp, b'"', SpAtom);
base_wrapper!(doc = "VCHAR = ! - ~", VChar, vchar, b'!'..=b'~', VCharAtom);

base_wrapper!(doc = "DIGIT = 0-9", Digit, digit, b'0'..=b'9', DigitAtom);

/// LWsp
pub struct LWsp<Span>(Span);

/// Use of this linear-white-space rule permits lines containing only white
/// space that are no longer legal in mail headers and have caused
/// interoperability problems in other contexts.
///
/// Do not use when defining mail headers and use with caution in other
/// contexts.
///
/// LWSP = *(WSP / CRLF WSP)
// code as equivalent LWSP = *([CRLF] WSP)
pub fn lwsp<Stream, Context>(
  stream: Stream,
) -> Parsed<LWsp<<Stream as Streaming>::Span>, Stream, Context>
where
  (): AsciiParse<Stream, Context>,
{
  crlf
    .opt()
    .and(wsp)
    .drop()
    .fold_bounds(.., || (), Acc::acc)
    .span()
    .map(|success| LWsp(success.stream))
    .parse(stream)
}

/// CrLf
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CrLf;

impl Display for CrLf {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "\r\n")
  }
}

/// Internet standard newline
///
/// CRLF = CR LF
///
/// Note: this variant will strictly expect "\r\n".
/// Use [crlf_relaxed](fn.crlf_relaxed.html) to accept "\r\n" as well as only
/// "\n".
pub fn crlf<Stream, Context>(stream: Stream) -> Parsed<CrLf, Stream, Context>
where
  (): AsciiParse<Stream, Context>,
{
  (cr, lf).map(|_| CrLf).parse(stream)
}

/// CrLfRelaxed
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CrLfRelaxed {
  /// true if cr was present
  pub have_cr: bool,
}

impl Display for CrLfRelaxed {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    if self.have_cr {
      writeln!(f, "\r")
    } else {
      writeln!(f)
    }
  }
}

/// Newline, with and without "\r".
pub fn crlf_relaxed<Stream, Context>(stream: Stream) -> Parsed<CrLfRelaxed, Stream, Context>
where
  (): AsciiParse<Stream, Context>,
{
  (cr.opt(), lf)
    .map(|(cr, _)| CrLfRelaxed {
      have_cr: cr.is_some(),
    })
    .parse(stream)
}

// pub fn no_case<Stream, Context>(c: u8) -> impl Parse<Stream, <Stream
// as UtilsStream>::Item, Context> where
// where
//   (): AsciiParse<Stream, Context>,
// {
//   is(c.to_ascii_lowercase()).or(is(c.to_ascii_uppercase()))
// }

#[cfg(test)]
mod tests {
  use super::*;
  use crate::context::Ignore;

  #[test]
  fn test_alpha() {
    assert_eq!(
      alpha::<_, Ignore>("a".as_bytes()),
      Parsed::Success {
        token: Alpha::new(b'a').unwrap(),
        stream: "".as_bytes(),
      }
    );
    assert_eq!(
      alpha::<_, Ignore>("z".as_bytes()),
      Parsed::Success {
        token: Alpha::new(b'z').unwrap(),
        stream: "".as_bytes(),
      }
    );

    assert_eq!(
      alpha::<_, Ignore>("A".as_bytes()),
      Parsed::Success {
        token: Alpha::new(b'A').unwrap(),
        stream: "".as_bytes(),
      }
    );
    assert_eq!(
      alpha::<_, Ignore>("Z".as_bytes()),
      Parsed::Success {
        token: Alpha::new(b'Z').unwrap(),
        stream: "".as_bytes(),
      }
    );

    assert!(!matches!(
      alpha::<_, Ignore>("".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      alpha::<_, Ignore>("`".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      alpha::<_, Ignore>("{".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      alpha::<_, Ignore>("@".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      alpha::<_, Ignore>("[".as_bytes()),
      Parsed::Success { .. }
    ));
  }

  #[test]
  fn test_bit() {
    assert_eq!(
      bit::<_, Ignore>("0".as_bytes()),
      Parsed::Success {
        token: Bit::new(b'0').unwrap(),
        stream: "".as_bytes(),
      }
    );
    assert_eq!(
      bit::<_, Ignore>("1".as_bytes()),
      Parsed::Success {
        token: Bit::new(b'1').unwrap(),
        stream: "".as_bytes(),
      }
    );

    assert!(!matches!(
      bit::<_, Ignore>("".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      bit::<_, Ignore>("/".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      bit::<_, Ignore>("2".as_bytes()),
      Parsed::Success { .. }
    ));
  }

  #[test]
  fn test_char() {
    assert_eq!(
      char::<_, Ignore>("\x01".as_bytes()),
      Parsed::Success {
        token: Char::new(b'\x01').unwrap(),
        stream: "".as_bytes(),
      }
    );
    assert_eq!(
      char::<_, Ignore>("\x7f".as_bytes()),
      Parsed::Success {
        token: Char::new(b'\x7f').unwrap(),
        stream: "".as_bytes(),
      }
    );

    assert!(!matches!(
      char::<_, Ignore>("".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      char::<_, Ignore>("\x00".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      char::<_, Ignore>("\u{80}".as_bytes()),
      Parsed::Success { .. }
    ));
  }

  #[test]
  fn test_cr() {
    assert_eq!(
      cr::<_, Ignore>("\r".as_bytes()),
      Parsed::Success {
        token: Cr::new(b'\r').unwrap(),
        stream: "".as_bytes(),
      }
    );

    assert!(!matches!(
      cr::<_, Ignore>("".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      cr::<_, Ignore>("\x0c".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      cr::<_, Ignore>("\x0e".as_bytes()),
      Parsed::Success { .. }
    ));
  }

  #[test]
  fn test_crlf() {
    assert_eq!(
      crlf::<_, Ignore>("\r\n".as_bytes()),
      Parsed::Success {
        token: CrLf,
        stream: "".as_bytes(),
      }
    );

    assert!(!matches!(
      crlf::<_, Ignore>("".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      crlf::<_, Ignore>("\x0c".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      crlf::<_, Ignore>("\r".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      crlf::<_, Ignore>("\x0e".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      crlf::<_, Ignore>("\x09".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      crlf::<_, Ignore>("\n".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      crlf::<_, Ignore>("\x0b".as_bytes()),
      Parsed::Success { .. }
    ));
  }

  #[test]
  fn test_crlf_relaxed() {
    assert_eq!(
      crlf_relaxed::<_, Ignore>("\n".as_bytes()),
      Parsed::Success {
        token: CrLfRelaxed { have_cr: false },
        stream: "".as_bytes(),
      }
    );
    assert_eq!(
      crlf_relaxed::<_, Ignore>("\r\n".as_bytes()),
      Parsed::Success {
        token: CrLfRelaxed { have_cr: true },
        stream: "".as_bytes(),
      }
    );

    assert!(!matches!(
      crlf_relaxed::<_, Ignore>("".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      crlf_relaxed::<_, Ignore>("\x0c".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      crlf_relaxed::<_, Ignore>("\r".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      crlf_relaxed::<_, Ignore>("\x0e".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      crlf_relaxed::<_, Ignore>("\x09".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      crlf_relaxed::<_, Ignore>("\x0b".as_bytes()),
      Parsed::Success { .. }
    ));
  }

  #[test]
  fn test_ctl() {
    assert_eq!(
      ctl::<_, Ignore>("\x00".as_bytes()),
      Parsed::Success {
        token: Ctl::new(b'\x00').unwrap(),
        stream: "".as_bytes(),
      }
    );
    assert_eq!(
      ctl::<_, Ignore>("\x1f".as_bytes()),
      Parsed::Success {
        token: Ctl::new(b'\x1f').unwrap(),
        stream: "".as_bytes(),
      }
    );
    assert_eq!(
      ctl::<_, Ignore>("\x7f".as_bytes()),
      Parsed::Success {
        token: Ctl::new(b'\x7f').unwrap(),
        stream: "".as_bytes(),
      }
    );

    assert!(!matches!(
      ctl::<_, Ignore>("".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      ctl::<_, Ignore>("\x20".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      ctl::<_, Ignore>("\u{80}".as_bytes()),
      Parsed::Success { .. }
    ));
  }

  #[test]
  fn test_digit() {
    for i in 0..10u8 {
      assert_eq!(
        digit::<_, Ignore>(i.to_string().as_bytes()),
        Parsed::Success {
          token: Digit::new(i + b'0').unwrap(),
          stream: "".as_bytes(),
        }
      );
    }

    assert!(!matches!(
      digit::<_, Ignore>("".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      digit::<_, Ignore>("A".as_bytes()),
      Parsed::Success { .. }
    ));
  }

  #[test]
  fn test_hexdig() {
    assert_eq!(
      hexdig::<_, Ignore>("0".as_bytes()),
      Parsed::Success {
        token: HexDig::new(b'0').unwrap(),
        stream: "".as_bytes(),
      }
    );
    assert_eq!(
      hexdig::<_, Ignore>("9".as_bytes()),
      Parsed::Success {
        token: HexDig::new(b'9').unwrap(),
        stream: "".as_bytes(),
      }
    );
    assert_eq!(
      hexdig::<_, Ignore>("a".as_bytes()),
      Parsed::Success {
        token: HexDig::new(b'a').unwrap(),
        stream: "".as_bytes(),
      }
    );
    assert_eq!(
      hexdig::<_, Ignore>("f".as_bytes()),
      Parsed::Success {
        token: HexDig::new(b'f').unwrap(),
        stream: "".as_bytes(),
      }
    );
    assert_eq!(
      hexdig::<_, Ignore>("A".as_bytes()),
      Parsed::Success {
        token: HexDig::new(b'A').unwrap(),
        stream: "".as_bytes(),
      }
    );
    assert_eq!(
      hexdig::<_, Ignore>("F".as_bytes()),
      Parsed::Success {
        token: HexDig::new(b'F').unwrap(),
        stream: "".as_bytes(),
      }
    );

    assert!(!matches!(
      hexdig::<_, Ignore>("".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      hexdig::<_, Ignore>("/".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      hexdig::<_, Ignore>(":".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      hexdig::<_, Ignore>("`".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      hexdig::<_, Ignore>("g".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      hexdig::<_, Ignore>("@".as_bytes()),
      Parsed::Success { .. }
    ));
    assert!(!matches!(
      hexdig::<_, Ignore>("G".as_bytes()),
      Parsed::Success { .. }
    ));
  }
}
