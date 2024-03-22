use core::{
  fmt::Debug,
  ops::BitOr,
};

use crate::core::{
  Parsed,
  Streaming,
  Success,
};

/// Parse is a trait that all parsers should implement.
/// There is a blanked implementation for type that implement FnMut that match
/// signature of parse(). This mean you can quickly use a function to implement
/// a Parser.
pub trait Parse<Stream, Context> {
  /// Token is what the parser produced.
  /// For example, a parser that read an u32 would have as Token u32.
  type Token; //: Debug;

  /// The main method of binator, any parser will be called by this method.
  /// `parse()` will take a stream as parameter and eat data from it to produce
  /// a Token. The result is what the parser `Parsed` is an enum of possible
  /// outcome.
  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context>;
}

impl<Token, Stream, Context, F> Parse<Stream, Context> for F
where
  F: FnMut(Stream) -> Parsed<Token, Stream, Context>,
{
  type Token = Token;

  fn parse(&mut self, stream: Stream) -> Parsed<Token, Stream, Context> {
    self(stream)
  }
}

// Array: Behavior chained "or"

// const fn non_zero(n: usize) -> usize {
//   if n == 0 {
//     panic!("Parse empty array don't make sense")
//   } else {
//     n
//   }
// }

/// Array can be used to try several parser until one succeed.
/// The parser are tried from start to end of the array.
/// This is limited to parser of the same type.
impl<Stream, Context, Parser, const N: usize> Parse<Stream, Context> for [Parser; N]
where
  Stream: Streaming,
  Parser: Parse<Stream, Context>,
  Context: BitOr<Output = Context>,
  //  [(); non_zero(N)]:,
{
  type Token = Parser::Token;

  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    let mut iter = self.iter_mut();

    if let Some(first) = iter.next() {
      match first.parse(stream.clone()) {
        success @ Parsed::Success { .. } => success,
        Parsed::Failure(context) => {
          let mut acc = context;

          for parser in iter {
            match parser.parse(stream.clone()) {
              success @ Parsed::Success { .. } => {
                return success;
              }
              Parsed::Failure(context) => {
                acc = acc.bitor(context);
              }
              Parsed::Error(context) => {
                return Parsed::Error(acc.bitor(context));
              }
            }
          }

          Parsed::Failure(acc)
        }
        Parsed::Error(context) => Parsed::Error(context),
      }
    } else {
      panic!("Parse empty array don't make sense")
    }
  }
}

// Tuple: Behavior chained "and"
include!(concat!(env!("OUT_DIR"), "/parse_tuple.rs"));

impl<Stream, Context> Parse<Stream, Context> for () {
  type Token = ();

  fn parse(&mut self, stream: Stream) -> Parsed<(), Stream, Context> {
    Parsed::new_success((), stream)
  }
}

#[cfg(test)]
mod tests {
  use core::ops::BitOr;

  use crate::core::{
    Parse,
    Parsed,
    Streaming,
  };

  // Should not compile
  #[allow(dead_code)]
  fn array_parse_zero<Stream, Context>(stream: Stream) -> Parsed<(), Stream, Context>
  where
    Stream: Streaming,
    <Stream as Streaming>::Item: Into<char>,
    Context: BitOr<Output = Context>,
  {
    let mut foo: [(); 0] = [];
    Parse::<Stream, Context>::parse(&mut foo, stream)
  }
}

#[cfg(feature = "either")]
impl<Stream, Context, L, R> Parse<Stream, Context> for either::Either<L, R>
where
  L: Parse<Stream, Context>,
  R: Parse<Stream, Context, Token = L::Token>,
{
  type Token = L::Token;

  fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
    match self {
      either::Either::Left(l) => l.parse(stream),
      either::Either::Right(r) => r.parse(stream),
    }
  }
}
