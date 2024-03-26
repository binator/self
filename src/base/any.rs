use crate::{
  Contexting,
  CoreAtom,
  Parsed,
  Split,
  Streaming,
};

/// This will return any item in the stream, that equivalent to .next() from
/// iterator. This can be used in many combinator that the root of all others
/// base combinator. any will take care of end of stream and stream error.
#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "trace", skip_all, ret(Display))
)]
pub fn any<Stream, Context>(stream: Stream) -> Parsed<Stream::Item, Stream, Context>
where
  Stream: Streaming,
  Context: Contexting<CoreAtom<Stream>>,
{
  match stream.split_first() {
    Split::Success { item, stream } => Parsed::Success {
      token: item,
      stream,
    },
    Split::NotEnoughItem(stream) => Parsed::Failure(Context::new(CoreAtom::EndOfStream { stream })),
    Split::Error(error) => Parsed::Error(Context::new(CoreAtom::Error { error })),
  }
}
