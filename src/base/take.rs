use crate::{
  Contexting,
  CoreAtom,
  Parse,
  Parsed,
  Split,
  Streaming,
};

/// Return n number of items from stream in a Span
pub fn take<Stream, Context>(n: usize) -> impl Parse<Stream, Context, Token = Stream::Span>
where
  Stream: Streaming,
  Context: Contexting<CoreAtom<Stream>>,
{
  Take { n }
}

#[derive(Clone)]
struct Take {
  n: usize,
}

impl<Stream, Context> Parse<Stream, Context> for Take
where
  Stream: Streaming,
  Context: Contexting<CoreAtom<Stream>>,
{
  type Token = Stream::Span;

  #[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", name = "take", skip_all, ret(Display))
  )]
  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    match stream.split_at(self.n) {
      Split::Success { item, stream } => Parsed::Success {
        token: item,
        stream,
      },
      Split::NotEnoughItem(stream) => {
        Parsed::Failure(Context::new(CoreAtom::EndOfStream { stream }))
      }
      Split::Error(error) => Parsed::Error(Context::new(CoreAtom::Error { error })),
    }
  }
}
