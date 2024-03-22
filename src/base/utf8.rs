use crate::{
  base::{
    octet,
    BaseAtom,
  },
  core::{
    Contexting,
    CoreAtom,
    Parse,
    Parsed,
    Streaming,
    Success,
  },
  utils::Utils,
};

fn raw<Stream, Context>(stream: Stream) -> Parsed<u32, Stream, Context>
where
  Stream: Streaming,
  Context: Contexting<CoreAtom<Stream>>,
  Context: Contexting<BaseAtom<u8>>,
  Stream::Item: Into<u8>,
{
  let Success { token: a, stream } = octet.parse(stream)?;
  if a & 0x80 == 0 {
    Parsed::Success {
      token: a as u32,
      stream,
    }
  } else if a & 0xE0 == 0xC0 {
    let Success { token: b, stream } = octet.parse(stream)?;

    Parsed::Success {
      token: (a as u32 & 0x1F) << 6 | (b as u32 & 0x3F),
      stream,
    }
  } else if a & 0xF0 == 0xE0 {
    let Success { token: b, stream } = octet.parse(stream)?;
    let Success { token: c, stream } = octet.parse(stream)?;

    Parsed::Success {
      token: (a as u32 & 0x0F) << 12 | (b as u32 & 0x3F) << 6 | (c as u32 & 0x3F),
      stream,
    }
  } else if a & 0xF8 == 0xF0 {
    let Success { token: b, stream } = octet.parse(stream)?;
    let Success { token: c, stream } = octet.parse(stream)?;
    let Success { token: d, stream } = octet.parse(stream)?;

    Parsed::Success {
      token: (a as u32 & 0x07) << 18
        | (b as u32 & 0x3F) << 12
        | (c as u32 & 0x3F) << 6
        | (d as u32 & 0x3F),
      stream,
    }
  } else {
    Parsed::Failure(Contexting::new(BaseAtom::Utf8 {}))
  }
}

/// Parser that will read stream and return valid utf8 char
/// If you are expecting utf8 you MUST use this Parser, and not character
/// Parser.
#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "trace", skip_all, ret(Display))
)]
pub fn utf8<Stream, Context>(stream: Stream) -> Parsed<char, Stream, Context>
where
  Stream: Streaming,
  Context: Contexting<CoreAtom<Stream>>,
  Context: Contexting<BaseAtom<u8>>,
  Stream::Item: Into<u8>,
{
  raw
    .try_map(|raw| char::from_u32(raw).ok_or_else(|| Contexting::new(BaseAtom::Utf8 {})))
    .parse(stream)
}

#[cfg(test)]
mod tests {
  use crate::{
    context::Ignore,
    core::Parsed,
  };

  #[test]
  fn utf8() {
    println!("{}", "❤".len());
    assert_eq!(
      super::utf8::<_, Ignore>("❤".as_bytes()),
      Parsed::Success {
        token: '❤',
        stream: "".as_bytes(),
      }
    );
  }
}
