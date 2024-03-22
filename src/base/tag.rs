use crate::{
  base::{
    take,
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

/// Take a &'static str and return a Parser that will
/// compare it with Stream, this requiere the Stream Span
/// to implement `AsRef<[u8]>`
pub fn tag<Stream, Context>(tag: &'static str) -> impl Parse<Stream, Context, Token = &'static str>
where
  Stream: Streaming,
  Stream::Span: AsRef<[u8]>,
  Context: Contexting<CoreAtom<Stream>>,
  Context: Contexting<BaseAtom<u8>>,
  Context: Contexting<UtilsAtom<Stream>>,
{
  Tag { tag }
}

struct Tag {
  tag: &'static str,
}

impl<Stream, Context> Parse<Stream, Context> for Tag
where
  Stream: Streaming,
  Stream::Span: AsRef<[u8]>,
  Context: Contexting<CoreAtom<Stream>>,
  Context: Contexting<BaseAtom<u8>>,
  Context: Contexting<UtilsAtom<Stream>>,
{
  type Token = &'static str;

  #[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", name = "tag", skip_all, ret(Display))
  )]
  fn parse(&mut self, stream: Stream) -> Parsed<&'static str, Stream, Context> {
    take(self.tag.len())
      .add_atom(|| BaseAtom::Tag { tag: self.tag })
      .try_map(|token: Stream::Span| {
        if token.as_ref() == self.tag.as_bytes() {
          Ok(self.tag)
        } else {
          Err(Context::new(BaseAtom::Tag { tag: self.tag }))
        }
      })
      .parse(stream)
  }
}

/// Take a &'static str and return a Parser that will
/// compare it without the ascii case with Stream, this requiere the Stream Span
/// to implement `AsRef<[u8]>`
pub fn tag_no_case<Stream, Context>(
  tag: &'static str,
) -> impl Parse<Stream, Context, Token = &'static str>
where
  Stream: Streaming,
  Stream::Span: AsRef<[u8]>,
  Context: Contexting<CoreAtom<Stream>>,
  Context: Contexting<BaseAtom<u8>>,
  Context: Contexting<UtilsAtom<Stream>>,
{
  TagNoCase { tag }
}

struct TagNoCase {
  tag: &'static str,
}

impl<Stream, Context> Parse<Stream, Context> for TagNoCase
where
  Stream: Streaming,
  Stream::Span: AsRef<[u8]>,
  Context: Contexting<CoreAtom<Stream>>,
  Context: Contexting<BaseAtom<u8>>,
  Context: Contexting<UtilsAtom<Stream>>,
{
  type Token = &'static str;

  #[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", name = "tag_no_case", skip_all, ret(Display))
  )]
  fn parse(&mut self, stream: Stream) -> Parsed<&'static str, Stream, Context> {
    take(self.tag.len())
      .add_atom(|| BaseAtom::Tag { tag: self.tag })
      .try_map(|token: Stream::Span| {
        if token.as_ref().eq_ignore_ascii_case(self.tag.as_bytes()) {
          Ok(self.tag)
        } else {
          Err(Context::new(BaseAtom::Tag { tag: self.tag }))
        }
      })
      .parse(stream)
  }
}
