use crate::{
  base::{
    is,
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

/// Enum that hold Sign value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
  /// When sign is positive
  Pos,
  /// When sign is negative
  Neg,
}

/// Sign   ::= [+-]
#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "trace", skip_all, ret(Display))
)]
pub fn sign<Stream, Context>(stream: Stream) -> Parsed<Sign, Stream, Context>
where
  Stream: Streaming,
  <Stream as Streaming>::Item: Into<u8>,
  Context: Contexting<UtilsAtom<Stream>>,
  Context: Contexting<BaseAtom<u8>>,
  Context: Contexting<CoreAtom<Stream>>,
{
  is(b'-')
    .map(|_| Sign::Neg)
    .or(is(b'+').map(|_| Sign::Pos))
    .parse(stream)
}
