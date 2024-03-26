#![doc = include_str!("readme.md")]

//! Utils combinator
//!
//! [Utils] trait contain everything you want to know

use core::{
  fmt::{
    Debug,
    Display,
  },
  ops::{
    BitOr,
    FromResidual,
    Try,
  },
};

use crate::{
  Contexting,
  Parse,
  Parsed,
  Streaming,
};

mod span;
pub use span::*;
mod opt;
pub use opt::*;
mod and;
pub use and::*;
mod and_then;
pub use and_then::*;
mod and_drop;
pub use and_drop::*;
mod drop_and;
pub use drop_and::*;
mod or;
pub use or::*;
mod not;
pub use not::*;
mod peek;
pub use peek::*;
mod map;
pub use map::*;
mod to;
pub use to::*;
mod try_map;
pub use try_map::*;
mod drop;
pub use drop::*;
// mod drop_last;
// pub use drop_last::*;
// mod drop_first;
// pub use drop_first::*;

mod filter;
pub use filter::*;
mod filter_map;
pub use filter_map::*;

mod fold_bounds;
pub use fold_bounds::*;
mod try_fold_bounds;
pub use try_fold_bounds::*;
mod try_fold_iter;
pub use try_fold_iter::*;
mod fold_until;
pub use fold_until::*;
mod try_fold_until;
pub use try_fold_until::*;
mod fill;
pub use fill::*;

mod enumerate;
pub use enumerate::*;
mod limit;
pub use limit::*;

mod add_atom;
pub use add_atom::*;

mod acc;
pub use acc::*;
mod try_acc;
pub use try_acc::*;
mod extend;
pub use extend::*;
mod try_extend;
pub use try_extend::*;
mod push;
pub use push::*;
mod try_push;
pub use try_push::*;

/// Atom for most utils combinator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UtilsAtom<Stream> {
  /// When combinator like fold didn't reach the minimun number of Token asked
  MinNotReach {
    /// The number of Token found
    i: usize,
    /// The number of Token requested
    min: usize,
  },
  /// When combinator like fold_until fail before until is reached
  UntilNotReach,
  //  IterEndNotReach,
  /// When max combinator reached the max allowed.
  // Stand alone ?
  Max(usize),
  /// When filter combinator return failure if filter refuse the Token
  Filter,
  /// When Span combinator call diff from stream but it's return Error.
  /// If you encounter this, it's either mean the two stream are not the same or
  /// you rewind the stream to a previous point of original stream
  // missing success token
  Diff {
    /// The original steam
    stream: Stream,
    /// The stream returned by the success parser
    stream_success: Stream,
  },
}

impl<Stream> Display for UtilsAtom<Stream> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      UtilsAtom::MinNotReach { i, min } => write!(f, "MinNotReach: {} < {}", i, min),
      UtilsAtom::UntilNotReach => write!(f, "UntilNotReach"),
      //      UtilsAtom::IterEndNotReach => write!(f, "IterEndNotReach"),
      UtilsAtom::Max(n) => write!(f, "Max {}", n),
      UtilsAtom::Filter { .. } => write!(f, "Filter"),
      UtilsAtom::Diff { .. } => write!(f, "Diff"),
    }
  }
}

/// Extend Parse trait with combinator
pub trait Utils<Stream, Context>: Sized + Parse<Stream, Context>
where
  Stream: Streaming,
{
  /// and_then will call the underline parser, if successful it will call the
  /// function in parameter and give it the produced Token. The function must
  /// return a new parser that will be called to producted the Token returned
  /// by and_then parser.
  fn and_then<OtherParser, F>(self, f: F) -> AndThen<Self, F>
  where
    OtherParser: Parse<Stream, Context>,
    F: Fn(Self::Token) -> OtherParser,
  {
    and_then(self, f)
  }

  /// and combinator will call the underline parser, and if successful the
  /// parser given in parameter. If the second parser is also successful, and
  /// combinator will return a tuple that contain the two Token producted.
  fn and<OtherParser, OtherToken>(self, other: OtherParser) -> And<Self, OtherParser>
  where
    OtherParser: Parse<Stream, Context, Token = OtherToken>,
  {
    and(self, other)
  }

  /// Same than and combinator but it will drop the second Token instead,
  /// returning only the first Token from the inner parser.
  fn and_drop<OtherParser, OtherToken>(self, other: OtherParser) -> AndDrop<Self, OtherParser>
  where
    OtherParser: Parse<Stream, Context, Token = OtherToken>,
  {
    and_drop(self, other)
  }

  /// Same than and combinator but it will drop the underline Token instead,
  /// returning only the second Token from the parser in parameter.
  fn drop_and<OtherParser, OtherToken>(self, other: OtherParser) -> DropAnd<Self, OtherParser>
  where
    OtherParser: Parse<Stream, Context, Token = OtherToken>,
  {
    drop_and(self, other)
  }

  /// Call the underline parser but drop the Token if sucessful. This can be
  /// considered as a shortcut of the toilet closure: `.map(|_| ())`.
  fn drop(self) -> Drop<Self> {
    drop(self)
  }

  /// Will call the underline parser N times to fill an array of size N and
  /// return [Token; N] if successfull
  fn fill<const N: usize>(self) -> Fill<Self, N>
  where
    Context: Contexting<UtilsAtom<Stream>>,
  {
    fill(self)
  }

  // consider removing too specific, how to replace ?
  /// This combinator take a Iterator, it will call the inner parser
  /// as many time the iterator lenght. The Item produced and the Token
  /// produced will be given to the function F along with the accumulator
  /// produced by Init function. The Init and F function can use return any
  /// type that implement Try. The Token produced is the last accumulator value.
  fn try_fold_iter<IntoIter, Init, Acc, Ret, F>(
    self, iter: IntoIter, init: Init, f: F,
  ) -> TryFoldIter<Self, IntoIter, Init, F>
  where
    Context: Contexting<UtilsAtom<Stream>>,
    IntoIter: IntoIterator + Clone,
    Init: Fn() -> Ret,
    F: Fn(Acc, Self::Token, IntoIter::Item) -> Ret,
    Ret: Try<Output = Acc>,
    Parsed<Acc, Stream, Context>: FromResidual<Ret::Residual>,
  {
    try_fold_iter(self, iter, init, f)
  }

  /// This Combinator will call the inner parser as long as the until
  /// parser is not successful, each Token produced will be feed to F
  /// function along with the accumulator.
  ///
  /// The Token produced by fold_until is a tuple of the last
  /// value of the accumulator and the Token from until parser.
  fn fold_until<TokenUntil, Acc, Until, Init, F>(
    self, until: Until, init: Init, f: F,
  ) -> FoldUntil<Self, Until, Init, F>
  where
    Context: Contexting<UtilsAtom<Stream>>,
    Until: Parse<Stream, Context, Token = TokenUntil>,
    Init: FnMut() -> Acc,
    F: FnMut(Acc, Self::Token) -> Acc,
  {
    fold_until(self, until, init, f)
  }

  /// The same then fold_until but can be used with type that implement Try
  fn try_fold_until<TokenUntil, Acc, Parser, Until, Init, Ret, F>(
    self, until: Until, init: Init, f: F,
  ) -> TryFoldUntil<Self, Until, Init, F>
  where
    Context: Contexting<UtilsAtom<Stream>>,
    Until: Parse<Stream, Context, Token = TokenUntil>,
    Init: Fn() -> Ret,
    F: Fn(Acc, Self::Token) -> Ret,
    Ret: Try<Output = Acc>,
    Parsed<(Acc, Until::Token), Stream, Context>: FromResidual<Ret::Residual>,
  {
    try_fold_until(self, until, init, f)
  }

  /// Main fold combinator, the bahavior depend on the Bounds argument.
  /// This combinator is implemented for Range and usize. The number of
  /// iteration depend of the type and the value used for the Bounds argument.
  ///
  /// | Type                      | Value | Min | Max |
  /// |:--------------------------|:------|:----|:----|
  /// | `Range<usize>`            | 2..4  | 2   | 4   |
  /// | `RangeInclusive<usize>`   | 2..=4 | 2   | 5   |
  /// | `RangeFrom<usize>`        | 4..   | 4   | ∞   |
  /// | `RangeTo<usize>`          | ..4   | 0   | 4   |
  /// | `RangeToInclusive<usize>` | ..=4  | 0   | 5   |
  /// | `RangeFull`               | ..    | 0   | ∞   |
  /// | `usize`                   | 4     | 4   | 4   |
  ///
  /// If the minimun value is not respected, it will return an Failure. Then
  /// until the inner parser return a Failure or the maximun value is reach it
  /// will continue iterate. Then it will return a Success with the last value
  /// of the Accumulator This offer a great number of possibility for your
  /// parser with only one combinator.
  fn fold_bounds<Bounds, Acc, Init, F>(
    self, bounds: Bounds, init: Init, f: F,
  ) -> FoldBounds<Self, Bounds, Init, F>
  where
    Context: Contexting<UtilsAtom<Stream>>,
    Init: FnMut() -> Acc,
    F: FnMut(Acc, Self::Token) -> Acc,
    Bounds: FoldBoundsParse,
    Acc: Debug,
  {
    fold_bounds(self, bounds, init, f)
  }

  /// Same than fold_bounds but F and Acc can return type that implement Try
  fn try_fold_bounds<Bounds, Acc, Init, Ret, F>(
    self, bounds: Bounds, init: Init, f: F,
  ) -> TryFoldBounds<Self, Bounds, Init, F>
  where
    Context: Contexting<UtilsAtom<Stream>>,
    Init: Fn() -> Ret,
    F: Fn(Acc, Self::Token) -> Ret,
    Ret: Try<Output = Acc>,
    Parsed<Acc, Stream, Context>: FromResidual<Ret::Residual>,
    Bounds: TryFoldBoundsParse,
    Acc: Debug,
  {
    try_fold_bounds(self, bounds, init, f)
  }

  /// if the underline parser is not successful it will add call
  /// F and add the Atom provided to the Context
  fn add_atom<F, Atom>(self, f: F) -> AddAtom<Self, F>
  where
    F: Fn() -> Atom,
    Context: Contexting<Atom>,
  {
    add_atom(self, f)
  }

  /// If the underline parser is successful it will call F
  /// with the Token produced and change return a new Success with
  /// the Token returned by F.
  fn map<F, OtherToken>(self, f: F) -> Map<Self, F>
  where
    F: Fn(Self::Token) -> OtherToken,
  {
    map(self, f)
  }

  /// Only allow Success path if F return true
  fn filter<F>(self, f: F) -> Filter<Self, F>
  where
    F: Fn(&Self::Token) -> bool,
    Context: Contexting<UtilsAtom<Stream>>,
  {
    filter(self, f)
  }

  /// Merge of map and filter combinator, only return Success if
  /// F return Some.
  fn filter_map<F, OtherToken>(self, f: F) -> FilterMap<Self, F>
  where
    F: Fn(Self::Token) -> Option<OtherToken>,
    OtherToken: Clone,
    Context: Contexting<UtilsAtom<Stream>>,
  {
    filter_map(self, f)
  }

  /// If underline parse is successful it will drop the Token and replace it
  /// with the token in argument. This is mostly a inconditional .map()
  /// usefull to avoid the closure. (Can be used in Slice parser where map is
  /// not)
  fn to<OtherToken>(self, t: OtherToken) -> To<Self, OtherToken>
  where
    OtherToken: Clone,
  {
    to(self, t)
  }

  /// Evil Combinator, it will reverse Success in Failure and
  /// Failure in Success but will not touch Error. This Combinator
  /// should probably never be used.
  fn not(self) -> Not<Self>
  where
    Stream: Clone,
    Context: Contexting<NotAtom<Self::Token, Stream>>,
  {
    not(self)
  }

  /// Make a parser optional allowing failure. Return Some(Token)
  /// in case of Success of underline parser and None in case of Failure.
  /// This parser can't fail.
  fn opt(self) -> Optional<Self>
  where
    Stream: Clone,
  {
    opt(self)
  }

  /// Very much like Iterator .enumerate(). It will add a counter to every
  /// Token produced. Return a tuple `(counter, Token)`.
  fn enumerate(self) -> Enumerate<Self> {
    enumerate(self)
  }

  // evil ? should we remove ?
  /// Very much like Iterator .take(), it will only allow n call to inner parser
  /// before returning Failure. This parser use a state be aware that it must be
  /// recreate to be reset.
  fn limit(self, n: usize) -> Limit<Self>
  where
    Context: Contexting<UtilsAtom<Stream>>,
  {
    limit(self, n)
  }

  /// Allow branching, or will call the inner parser and if not sucessful
  /// it will call the second parser. or can be chained many time allowing
  /// multiple branch.
  fn or<OtherParser>(self, b: OtherParser) -> Or<Self, OtherParser>
  where
    Stream: Clone,
    OtherParser: Parse<Stream, Context>,
    Context: BitOr,
  {
    or(self, b)
  }

  /// peek allow to not consume the Stream but return the Token it would have
  /// produced. It should be used very often since you would parse more than
  /// once the same input.
  fn peek(self) -> Peek<Self>
  where
    Stream: Clone,
  {
    peek(self)
  }

  /// span allow to save the Stream consumed by the underline parser in a form
  /// of a Span. This is very usefull for error or to avoid fully tokenize an
  /// input. Be aware that Span can contains lifetime since it's linked to
  /// Stream implementation. For example, an Stream of slice u8 will contains
  /// a lifetime.
  fn span(self) -> Span<Self>
  where
    Context: Contexting<UtilsAtom<Stream>>,
  {
    span(self)
  }

  /// Same than .filter_map() but expect an Atom in case of Failure.
  fn try_map<OtherToken, F, Ret>(self, f: F) -> TryMap<Self, F>
  where
    F: Fn(Self::Token) -> Ret,
    Ret: Try<Output = OtherToken>,
    Parsed<OtherToken, Stream, Context>: FromResidual<Ret::Residual>,
  {
    try_map(self, f)
  }
}

impl<T, Stream, Context> Utils<Stream, Context> for T
where
  Stream: Streaming,
  Self: Parse<Stream, Context>,
{
}
