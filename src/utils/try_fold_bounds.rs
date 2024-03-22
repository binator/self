use core::{
  fmt::Debug,
  ops::{
    FromResidual,
    Range,
    RangeFrom,
    RangeFull,
    RangeInclusive,
    RangeTo,
    RangeToInclusive,
    Try,
  },
};

use crate::{
  core::{
    Contexting,
    Parse,
    Parsed,
    Streaming,
  },
  utils::UtilsAtom,
};

/// Implementation of [crate::utils::Utils::try_fold_bounds]
#[derive(Clone)]
pub struct TryFoldBounds<Parser, Bounds, Init, F> {
  parser: Parser,
  bounds: Bounds,
  init: Init,
  f: F,
}

/// Function style version of [crate::utils::Utils::try_fold_bounds]
pub fn try_fold_bounds<Bounds, Stream, Context, Parser, Acc, Init, Ret, F>(
  parser: Parser, bounds: Bounds, init: Init, try_fold: F,
) -> TryFoldBounds<Parser, Bounds, Init, F>
where
  Stream: Streaming,
  Context: Contexting<UtilsAtom<Stream>>,
  Parser: Parse<Stream, Context>,
  Init: Fn() -> Ret,
  F: Fn(Acc, Parser::Token) -> Ret,
  Ret: Try<Output = Acc>,
  Parsed<Acc, Stream, Context>: FromResidual<Ret::Residual>,
  Bounds: TryFoldBoundsParse,
  Acc: Debug,
{
  TryFoldBounds {
    bounds,
    parser,
    init,
    f: try_fold,
  }
}

/// This trait must be implemented by Bounds you are using.
/// As user you should not need to care about this except for few cases.
pub trait TryFoldBoundsParse {
  /// This method allow to implement parse for every type that implement
  /// Try FoldBoundsParse.
  fn try_fold_bounds<Stream, Context, Parser, Acc, Init, Ret, F>(
    &self, parser: &mut Parser, init: &mut Init, f: &mut F, stream: Stream,
  ) -> Parsed<Acc, Stream, Context>
  where
    Stream: Streaming,
    Context: Contexting<UtilsAtom<Stream>>,
    Parser: Parse<Stream, Context>,
    Init: Fn() -> Ret,
    F: Fn(Acc, Parser::Token) -> Ret,
    Ret: Try<Output = Acc>,
    Parsed<Acc, Stream, Context>: FromResidual<Ret::Residual>,
    Acc: Debug;
}

impl<Bounds, Stream, Context, Parser, Acc, Init, Ret, F> Parse<Stream, Context>
  for TryFoldBounds<Parser, Bounds, Init, F>
where
  Stream: Streaming,
  Context: Contexting<UtilsAtom<Stream>>,
  Parser: Parse<Stream, Context>,
  Init: Fn() -> Ret,
  F: Fn(Acc, Parser::Token) -> Ret,
  Ret: Try<Output = Acc>,
  Parsed<Acc, Stream, Context>: FromResidual<Ret::Residual>,
  Bounds: TryFoldBoundsParse,
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
      .try_fold_bounds(&mut self.parser, &mut self.init, &mut self.f, stream)
  }
}

macro_rules! allow_failure {
  ($stream:expr, $parser:expr, $acc:expr, $try_fold:expr) => {{
    match $parser.parse($stream.clone()) {
      Parsed::Success { token, stream } => {
        $acc = $try_fold($acc, token)?;
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
  ($stream:expr, $parser:expr, $acc:expr, $try_fold:expr, $min:expr, $i:expr) => {{
    match $parser.parse($stream) {
      Parsed::Success { token, stream } => {
        $acc = $try_fold($acc, token)?;
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

impl TryFoldBoundsParse for RangeFull {
  fn try_fold_bounds<Stream, Context, Parser, Acc, Init, Ret, F>(
    &self, parser: &mut Parser, init: &mut Init, f: &mut F, mut stream: Stream,
  ) -> Parsed<Acc, Stream, Context>
  where
    Stream: Streaming,
    Context: Contexting<UtilsAtom<Stream>>,
    Parser: Parse<Stream, Context>,
    Init: Fn() -> Ret,
    F: Fn(Acc, Parser::Token) -> Ret,
    Ret: Try<Output = Acc>,
    Parsed<Acc, Stream, Context>: FromResidual<Ret::Residual>,
    Acc: Debug,
  {
    let mut acc = init()?;
    loop {
      allow_failure!(stream, parser, acc, f)
    }
  }
}

impl TryFoldBoundsParse for RangeFrom<usize> {
  fn try_fold_bounds<Stream, Context, Parser, Acc, Init, Ret, F>(
    &self, parser: &mut Parser, init: &mut Init, f: &mut F, mut stream: Stream,
  ) -> Parsed<Acc, Stream, Context>
  where
    Stream: Streaming,
    Context: Contexting<UtilsAtom<Stream>>,
    Parser: Parse<Stream, Context>,
    Init: Fn() -> Ret,
    F: Fn(Acc, Parser::Token) -> Ret,
    Ret: Try<Output = Acc>,
    Parsed<Acc, Stream, Context>: FromResidual<Ret::Residual>,
    Acc: Debug,
  {
    let mut acc = init()?;

    for i in 0..self.start {
      deny_failure!(stream, parser, acc, f, self.start, i)
    }

    loop {
      allow_failure!(stream, parser, acc, f)
    }
  }
}

impl TryFoldBoundsParse for Range<usize> {
  fn try_fold_bounds<Stream, Context, Parser, Acc, Init, Ret, F>(
    &self, parser: &mut Parser, init: &mut Init, f: &mut F, mut stream: Stream,
  ) -> Parsed<Acc, Stream, Context>
  where
    Stream: Streaming,
    Context: Contexting<UtilsAtom<Stream>>,
    Parser: Parse<Stream, Context>,
    Init: Fn() -> Ret,
    F: Fn(Acc, Parser::Token) -> Ret,
    Ret: Try<Output = Acc>,
    Parsed<Acc, Stream, Context>: FromResidual<Ret::Residual>,
    Acc: Debug,
  {
    let mut acc = init()?;

    for i in 0..self.start {
      deny_failure!(stream, parser, acc, f, self.start, i)
    }

    for _ in self.start..self.end {
      allow_failure!(stream, parser, acc, f)
    }

    Parsed::Success { token: acc, stream }
  }
}

impl TryFoldBoundsParse for RangeInclusive<usize> {
  fn try_fold_bounds<Stream, Context, Parser, Acc, Init, Ret, F>(
    &self, parser: &mut Parser, init: &mut Init, f: &mut F, mut stream: Stream,
  ) -> Parsed<Acc, Stream, Context>
  where
    Stream: Streaming,
    Context: Contexting<UtilsAtom<Stream>>,
    Parser: Parse<Stream, Context>,
    Init: Fn() -> Ret,
    F: Fn(Acc, Parser::Token) -> Ret,
    Ret: Try<Output = Acc>,
    Parsed<Acc, Stream, Context>: FromResidual<Ret::Residual>,
    Acc: Debug,
  {
    let mut acc = init()?;

    for i in 0..*self.start() {
      deny_failure!(stream, parser, acc, f, *self.start(), i)
    }

    for _ in *self.start()..=*self.end() {
      allow_failure!(stream, parser, acc, f)
    }

    Parsed::Success { token: acc, stream }
  }
}

impl TryFoldBoundsParse for RangeTo<usize> {
  fn try_fold_bounds<Stream, Context, Parser, Acc, Init, Ret, F>(
    &self, parser: &mut Parser, init: &mut Init, f: &mut F, mut stream: Stream,
  ) -> Parsed<Acc, Stream, Context>
  where
    Stream: Streaming,
    Context: Contexting<UtilsAtom<Stream>>,
    Parser: Parse<Stream, Context>,
    Init: Fn() -> Ret,
    F: Fn(Acc, Parser::Token) -> Ret,
    Ret: Try<Output = Acc>,
    Parsed<Acc, Stream, Context>: FromResidual<Ret::Residual>,
    Acc: Debug,
  {
    let mut acc = init()?;

    for _ in 0..self.end {
      allow_failure!(stream, parser, acc, f)
    }

    Parsed::Success { token: acc, stream }
  }
}

impl TryFoldBoundsParse for RangeToInclusive<usize> {
  fn try_fold_bounds<Stream, Context, Parser, Acc, Init, Ret, F>(
    &self, parser: &mut Parser, init: &mut Init, f: &mut F, mut stream: Stream,
  ) -> Parsed<Acc, Stream, Context>
  where
    Stream: Streaming,
    Context: Contexting<UtilsAtom<Stream>>,
    Parser: Parse<Stream, Context>,
    Init: Fn() -> Ret,
    F: Fn(Acc, Parser::Token) -> Ret,
    Ret: Try<Output = Acc>,
    Parsed<Acc, Stream, Context>: FromResidual<Ret::Residual>,
    Acc: Debug,
  {
    let mut acc = init()?;

    for _ in 0..=self.end {
      allow_failure!(stream, parser, acc, f)
    }

    Parsed::Success { token: acc, stream }
  }
}

macro_rules! impl_primitive {
  ($primitive:ident) => {
    impl TryFoldBoundsParse for $primitive {
      fn try_fold_bounds<Stream, Context, Parser, Acc, Init, Ret, F>(
        &self, parser: &mut Parser, init: &mut Init, f: &mut F, mut stream: Stream,
      ) -> Parsed<Acc, Stream, Context>
      where
        Stream: Streaming,
        Context: Contexting<UtilsAtom<Stream>>,
        Parser: Parse<Stream, Context>,
        Init: Fn() -> Ret,
        F: Fn(Acc, Parser::Token) -> Ret,
        Ret: Try<Output = Acc>,
        Parsed<Acc, Stream, Context>: FromResidual<Ret::Residual>,
        Acc: Debug,
      {
        let mut acc = init()?;

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

// the error when not using a usize when calling try_fold is very hard to
// understand u8 and u16 could be implemented too BUT this could be error prone
// because of overflow
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

  #[derive(Display, Debug, Clone, From, PartialEq)]
  enum FromAtom<Stream: Streaming> {
    TryFold(UtilsAtom<Stream>),
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
  fn try_fold_bounds_full() {
    let stream = b"[".as_ref();

    let result: Parsed<_, _, HandleAtom<_>> = is(b'[')
      .or(is(b']'))
      .try_fold_bounds(.., || Ok(()), |_, _| Ok(()))
      .parse(stream);
    let expected = Parsed::Success {
      token: (),
      stream: &stream[1..],
    };
    assert_eq!(result, expected);
  }

  #[test]
  fn try_fold_bounds() {
    let stream = b"[".as_ref();
    let result: Parsed<_, _, HandleAtom<_>> = is(b'[')
      .or(is(b']'))
      .try_fold_bounds(1..5, || Ok(()), |_, _| Ok(()))
      .parse(stream);
    let expected = Parsed::Success {
      token: (),
      stream: &stream[1..],
    };
    assert_eq!(result, expected);
  }

  #[test]
  fn try_fold_bounds_failure() {
    let stream = b"abcd".as_ref();
    let result: Parsed<_, _, HandleAtom<_>> = is(b'[')
      .or(is(b']'))
      .try_fold_bounds(1..5, || Ok(()), |_, _| Ok(()))
      .parse(stream);
    let context = Keep::new(UtilsAtom::MinNotReach { i: 0, min: 1 });
    assert_eq!(result, Parsed::Failure(context));
  }
}
