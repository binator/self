use crate::{
  base::{
    ascii::digit,
    BaseAtom,
  },
  core::{
    Contexting,
    CoreAtom,
    Parse,
    Parsed,
    Streaming,
  },
  utils::{
    Utils,
    UtilsAtom,
  },
};

/// Parse character digit and return it in integer format
#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "trace", skip_all, ret(Display))
)]
pub fn to_digit<Stream, Context>(stream: Stream) -> Parsed<u8, Stream, Context>
where
  Stream: Streaming,
  Stream::Item: Into<u8>,
  Context: Contexting<CoreAtom<Stream>>,
  Context: Contexting<BaseAtom<u8>>,
  Context: Contexting<UtilsAtom<Stream>>,
{
  digit.map(|d| u8::from(d) - b'0').parse(stream)
}
