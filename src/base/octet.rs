use crate::{
  base::any,
  utils::Utils,
  Contexting,
  CoreAtom,
  Parse,
  Parsed,
  Streaming,
};

/// Will read an item from the Stream and convert it to an octet
#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "trace", skip_all, ret(Display))
)]
pub fn octet<Stream, Context>(stream: Stream) -> Parsed<u8, Stream, Context>
where
  Stream: Streaming,
  Context: Contexting<CoreAtom<Stream>>,
  Stream::Item: Into<u8>,
{
  any.map(Into::into).parse(stream)
}
