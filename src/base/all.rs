use crate::core::{
  Contexting,
  CoreAtom,
  Parsed,
  Streaming,
};

/// Parser that will consume all data from stream.
#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "trace", skip_all, ret(Display))
)]
pub fn all<Stream, Context>(stream: Stream) -> Parsed<Stream::Span, Stream, Context>
where
  Stream: Clone + Streaming,
  Context: Contexting<CoreAtom<Stream>>,
{
  match stream.all() {
    Ok(success) => success.into(),
    Err(error) => Parsed::Error(Context::new(CoreAtom::Error { error })),
  }
}
