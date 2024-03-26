use crate::{
  base::any,
  Contexting,
  CoreAtom,
  Parse,
  Parsed,
  Streaming,
};

/// Context from end_of_stream parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndOfStreamAtom<Stream: Streaming, Item = <Stream as Streaming>::Item> {
  /// Item found instead of end of stream
  pub item: Item,
  /// rest of the stream without item
  pub stream: Stream,
}

/// Return Success if the stream return end of stream.
#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "trace", skip_all, ret(Display))
)]
pub fn end_of_stream<Stream, Context>(stream: Stream) -> Parsed<(), Stream, Context>
where
  Stream: Streaming,
  Context: Contexting<CoreAtom<Stream>>,
  Context: Contexting<EndOfStreamAtom<Stream>>,
{
  match any.parse(stream.clone()) {
    Parsed::Success { token, stream } => Parsed::Failure(Context::new(EndOfStreamAtom {
      item: token,
      stream,
    })),
    Parsed::Failure(_context) => Parsed::Success { token: (), stream },
    Parsed::Error(context) => Parsed::Error(context),
  }
}
