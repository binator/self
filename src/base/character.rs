use crate::core::{
  any,
  Contexting,
  CoreAtom,
  Parsed,
  Parse,
  Streaming,
};
use crate::utils::Utils;

/// Parser that will consume one item from stream and convert it to char.
/// 
/// This parser will NOT check for utf8 this mean that it's must be used with
/// care. If you want to handle utf8 and not only ascii use utf8 parser.
// TODO: remove ?
#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "trace", skip_all, ret(Display))
)]
pub fn character<Stream, Context>(stream: Stream) -> Parsed<char, Stream, Context>
where
  Stream: Streaming,
  Context: Contexting<CoreAtom<Stream>>,
  Stream::Item: Into<char>,
{
  any.map(Into::into).parse(stream)
}
