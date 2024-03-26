use core::{
  fmt::Debug,
  ops::{
    Range,
    RangeFrom,
    RangeFull,
    RangeInclusive,
    RangeTo,
    RangeToInclusive,
  },
};

use crate::{
  utils::UtilsAtom,
  Contexting,
  Parse,
  Parsed,
  Streaming,
};

/// Implementation of [crate::utils::Utils::fold_bounds]
#[derive(Clone)]
pub struct FoldBounds<Parser, Bounds, Init, F> {
  parser: Parser,
  bounds: Bounds,
  init: Init,
  f: F,
}

/// Function style version of [crate::utils::Utils::fold_bounds]
pub fn fold_bounds<Bounds, Stream, Context, Parser, Acc, Init, F>(
  parser: Parser, bounds: Bounds, init: Init, fold: F,
) -> FoldBounds<Parser, Bounds, Init, F>
where
  Stream: Streaming,
  Context: Contexting<UtilsAtom<Stream>>,
  Parser: Parse<Stream, Context>,
  Init: FnMut() -> Acc,
  F: FnMut(Acc, Parser::Token) -> Acc,
  Bounds: FoldBoundsParse,
  Acc: Debug,
{
  FoldBounds {
    bounds,
    parser,
    init,
    f: fold,
  }
}

/// This trait must be implemented by Bounds you are using.
/// As user you should not need to care about this except for few cases.
pub trait FoldBoundsParse {
  /// This method allow to implement parse for every type that implement
  /// FoldBoundsParse.
  fn fold_bounds<Stream, Context, Parser, Acc, Init, F>(
    &self, parser: &mut Parser, init: &mut Init, f: &mut F, stream: Stream,
  ) -> Parsed<Acc, Stream, Context>
  where
    Stream: Streaming,
    Context: Contexting<UtilsAtom<Stream>>,
    Parser: Parse<Stream, Context>,
    Init: FnMut() -> Acc,
    F: FnMut(Acc, Parser::Token) -> Acc;
}

impl<Bounds, Stream, Context, Parser, Acc, Init, F> Parse<Stream, Context>
  for FoldBounds<Parser, Bounds, Init, F>
where
  Stream: Streaming,
  Context: Contexting<UtilsAtom<Stream>>,
  Parser: Parse<Stream, Context>,
  Init: FnMut() -> Acc,
  F: FnMut(Acc, Parser::Token) -> Acc,
  Bounds: FoldBoundsParse,
  Acc: Debug,
{
  type Token = Acc;

  // #[cfg_attr(
  //   feature = "tracing",
  //   tracing::instrument(level = "trace", skip_all, ret(Display))
  // )]
  fn parse(&mut self, stream: Stream) -> Parsed<Acc, Stream, Context> {
    self
      .bounds
      .fold_bounds(&mut self.parser, &mut self.init, &mut self.f, stream)
  }
}

macro_rules! allow_failure {
  ($stream:expr, $parser:expr, $acc:expr, $fold:expr) => {{
    match $parser.parse($stream.clone()) {
      Parsed::Success { token, stream } => {
        $acc = $fold($acc, token);
        $stream = stream;
      }
      Parsed::Failure(_context) => {
        return Parsed::Success {
          token: $acc,
          stream: $stream,
        };
      }
      Parsed::Error(context) => {
        return Parsed::Error(context);
      }
    };
  }};
}

macro_rules! deny_failure {
  ($stream:expr, $parser:expr, $acc:expr, $fold:expr, $min:expr, $i:expr) => {{
    match $parser.parse($stream) {
      Parsed::Success { token, stream } => {
        $acc = $fold($acc, token);
        $stream = stream;
      }
      Parsed::Failure(context) => {
        return Parsed::Failure(context.add(UtilsAtom::MinNotReach { min: $min, i: $i }));
      }
      Parsed::Error(context) => {
        return Parsed::Error(context);
      }
    };
  }};
}

impl FoldBoundsParse for RangeFull {
  fn fold_bounds<Stream, Context, Parser, Acc, Init, F>(
    &self, parser: &mut Parser, init: &mut Init, f: &mut F, mut stream: Stream,
  ) -> Parsed<Acc, Stream, Context>
  where
    Stream: Streaming,
    Context: Contexting<UtilsAtom<Stream>>,
    Parser: Parse<Stream, Context>,
    Init: FnMut() -> Acc,
    F: FnMut(Acc, Parser::Token) -> Acc,
  {
    let mut acc = init();
    loop {
      allow_failure!(stream, parser, acc, f)
    }
  }
}

impl FoldBoundsParse for RangeFrom<usize> {
  fn fold_bounds<Stream, Context, Parser, Acc, Init, F>(
    &self, parser: &mut Parser, init: &mut Init, f: &mut F, mut stream: Stream,
  ) -> Parsed<Acc, Stream, Context>
  where
    Stream: Streaming,
    Context: Contexting<UtilsAtom<Stream>>,
    Parser: Parse<Stream, Context>,
    Init: FnMut() -> Acc,
    F: FnMut(Acc, Parser::Token) -> Acc,
  {
    let mut acc = init();

    for i in 0..self.start {
      deny_failure!(stream, parser, acc, f, self.start, i)
    }

    loop {
      allow_failure!(stream, parser, acc, f)
    }
  }
}

impl FoldBoundsParse for Range<usize> {
  fn fold_bounds<Stream, Context, Parser, Acc, Init, F>(
    &self, parser: &mut Parser, init: &mut Init, f: &mut F, mut stream: Stream,
  ) -> Parsed<Acc, Stream, Context>
  where
    Stream: Streaming,
    Context: Contexting<UtilsAtom<Stream>>,
    Parser: Parse<Stream, Context>,
    Init: FnMut() -> Acc,
    F: FnMut(Acc, Parser::Token) -> Acc,
  {
    let mut acc = init();

    for i in 0..self.start {
      deny_failure!(stream, parser, acc, f, self.start, i)
    }

    for _ in self.start..self.end {
      allow_failure!(stream, parser, acc, f)
    }

    Parsed::Success { token: acc, stream }
  }
}

impl FoldBoundsParse for RangeInclusive<usize> {
  fn fold_bounds<Stream, Context, Parser, Acc, Init, F>(
    &self, parser: &mut Parser, init: &mut Init, f: &mut F, mut stream: Stream,
  ) -> Parsed<Acc, Stream, Context>
  where
    Stream: Streaming,
    Context: Contexting<UtilsAtom<Stream>>,
    Parser: Parse<Stream, Context>,
    Init: FnMut() -> Acc,
    F: FnMut(Acc, Parser::Token) -> Acc,
  {
    let mut acc = init();

    for i in 0..*self.start() {
      deny_failure!(stream, parser, acc, f, *self.start(), i)
    }

    for _ in *self.start()..=*self.end() {
      allow_failure!(stream, parser, acc, f)
    }

    Parsed::Success { token: acc, stream }
  }
}

impl FoldBoundsParse for RangeTo<usize> {
  fn fold_bounds<Stream, Context, Parser, Acc, Init, F>(
    &self, parser: &mut Parser, init: &mut Init, f: &mut F, mut stream: Stream,
  ) -> Parsed<Acc, Stream, Context>
  where
    Stream: Streaming,
    Context: Contexting<UtilsAtom<Stream>>,
    Parser: Parse<Stream, Context>,
    Init: FnMut() -> Acc,
    F: FnMut(Acc, Parser::Token) -> Acc,
  {
    let mut acc = init();

    for _ in 0..self.end {
      allow_failure!(stream, parser, acc, f)
    }

    Parsed::Success { token: acc, stream }
  }
}

impl FoldBoundsParse for RangeToInclusive<usize> {
  fn fold_bounds<Stream, Context, Parser, Acc, Init, F>(
    &self, parser: &mut Parser, init: &mut Init, f: &mut F, mut stream: Stream,
  ) -> Parsed<Acc, Stream, Context>
  where
    Stream: Streaming,
    Context: Contexting<UtilsAtom<Stream>>,
    Parser: Parse<Stream, Context>,
    Init: FnMut() -> Acc,
    F: FnMut(Acc, Parser::Token) -> Acc,
  {
    let mut acc = init();

    for _ in 0..=self.end {
      allow_failure!(stream, parser, acc, f)
    }

    Parsed::Success { token: acc, stream }
  }
}

macro_rules! impl_primitive {
  ($primitive:ident) => {
    impl FoldBoundsParse for $primitive {
      fn fold_bounds<Stream, Context, Parser, Acc, Init, F>(
        &self, parser: &mut Parser, init: &mut Init, f: &mut F, mut stream: Stream,
      ) -> Parsed<Acc, Stream, Context>
      where
        Stream: Clone,
        Stream: Eq,
        Context: Contexting<UtilsAtom<Stream>>,
        Parser: Parse<Stream, Context>,
        Init: FnMut() -> Acc,
        F: FnMut(Acc, Parser::Token) -> Acc,
      {
        let mut acc = init();

        let min = usize::from(*self);
        for i in 0..min {
          deny_failure!(stream, parser, acc, f, min, i)
        }

        Parsed::Success { token: acc, stream }
      }
    }
  };
}

macro_rules! impl_primitives {
  ($($primitives:ident,)*) => {
    $(impl_primitive!{$primitives})*
  };
}

// the error when not using a usize when calling fold is very hard to understand
// u8 and u16 could be implemented too BUT this could be error prone because of
// overflow
impl_primitives!(usize,);

#[cfg(test)]
mod tests {
  use core::convert::Infallible;

  use derive_more::{
    Display,
    From,
  };

  use crate::{
    base::{
      is,
      BaseAtom,
    },
    context::{
      Keep,
      Last,
    },
    utils::{
      Utils,
      UtilsAtom,
    },
    Contexting,
    CoreAtom,
    Parse,
    Parsed,
    Streaming,
  };

  #[derive(Display, Debug, Clone, From, PartialEq)]
  enum FromAtom<Stream: Streaming> {
    Fold(UtilsAtom<Stream>),
    Is(BaseAtom<u8>),
    Any(CoreAtom<Stream, Infallible>),
    Stream,
  }

  // impl<Stream> PartialEq for FromAtom<Stream> {
  //   fn eq(&self, other: &Self) -> bool {
  //     discriminant(self) == discriminant(other)
  //   }
  // }

  type HandleAtom<Stream> = Keep<Last, FromAtom<Stream>>;

  #[test]
  fn fold_bounds_full() {
    let stream = b"[".as_ref();

    let result: Parsed<_, _, HandleAtom<_>> = is(b'[')
      .or(is(b']'))
      .fold_bounds(.., || (), |_, _| ())
      .parse(stream);
    let expected = Parsed::Success {
      token: (),
      stream: &stream[1..],
    };
    assert_eq!(result, expected);
  }

  #[test]
  fn fold_bounds() {
    let stream = b"[".as_ref();
    let result: Parsed<_, _, HandleAtom<_>> = is(b'[')
      .or(is(b']'))
      .fold_bounds(1..5, || (), |_, _| ())
      .parse(stream);
    let expected = Parsed::Success {
      token: (),
      stream: &stream[1..],
    };
    assert_eq!(result, expected);
  }

  #[test]
  fn fold_bounds_failure() {
    let stream = b"abcd".as_ref();
    let result: Parsed<_, _, HandleAtom<_>> = is(b'[')
      .or(is(b']'))
      .fold_bounds(1..5, || (), |_, _| ())
      .parse(stream);
    let context = Keep::new(UtilsAtom::MinNotReach { i: 0, min: 1 });
    assert_eq!(result, Parsed::Failure(context));
  }
}
