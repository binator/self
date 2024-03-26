use core::{
  convert::Infallible,
  fmt::{
    self,
    Debug,
    Display,
    Formatter,
  },
  ops::{
    ControlFlow,
    FromResidual,
    Try,
  },
};

use crate::{
  Contexting,
  ParsedAux,
  ProvideElement,
  Split,
  Streaming,
  Success,
};

/// Parsed represent the result of a `parse()`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Parsed<Token, Stream, Context> {
  /// When parser successfully parse the stream
  Success {
    /// token produced by the parser.
    token: Token,
    /// the stream used by the parser, should have less item than before.
    stream: Stream,
  },
  /// When parser fail to parse the stream.
  /// This is not fatal and is a normal behavior for a parser.
  Failure(Context),
  // this could be added as "fast fail" path
  // Cut(Context),
  /// When parser encouter an error, this should be fatal. Could be a
  /// programming error or something wrong will the stream.
  Error(Context),
}

impl<Token, Stream, Context> Parsed<Token, Stream, Context> {
  /// Shortcut for `Parsed::Success { token, stream }`
  pub const fn new_success(token: Token, stream: Stream) -> Self {
    Self::Success { token, stream }
  }

  /// Shortcut for `Parsed::Failure(context_handle)`
  pub const fn new_failure(content: Context) -> Self {
    Self::Failure(content)
  }

  /// Shortcut for `Parsed::Error(context_handle)`
  pub const fn new_error(content: Context) -> Self {
    Self::Error(content)
  }

  /// Borrow `Parsed` to make temporary Parsed of reference
  pub const fn as_ref(&self) -> Parsed<&Token, &Stream, &Context> {
    match self {
      Self::Success { token, stream } => Parsed::Success { token, stream },
      Self::Failure(context) => Parsed::Failure(context),
      Self::Error(context) => Parsed::Error(context),
    }
  }

  /// Allow to quickly access success to map it.
  pub fn map_success<MappedToken, Map>(self, map: Map) -> Parsed<MappedToken, Stream, Context>
  where
    Map: FnOnce(Success<Token, Stream>) -> Success<MappedToken, Stream>,
  {
    match self {
      Parsed::Success { token, stream } => map(Success { token, stream }).into(),
      Parsed::Failure(context) => Parsed::Failure(context),
      Parsed::Error(context) => Parsed::Error(context),
    }
  }

  /// Allow to quickly access token to map it.
  pub fn map_token<MappedToken, Map>(self, map: Map) -> Parsed<MappedToken, Stream, Context>
  where
    Map: FnOnce(Token) -> MappedToken,
  {
    match self {
      Parsed::Success { token, stream } => Parsed::Success {
        token: map(token),
        stream,
      },
      Parsed::Failure(context) => Parsed::Failure(context),
      Parsed::Error(context) => Parsed::Error(context),
    }
  }

  /// Allow to quickly access stream to map it.
  pub fn map_stream<Map>(self, map: Map) -> Parsed<Token, Stream, Context>
  where
    Map: FnOnce(Stream) -> Stream,
  {
    match self {
      Parsed::Success { token, stream } => Parsed::Success {
        token,
        stream: map(stream),
      },
      Parsed::Failure(context) => Parsed::Failure(context),
      Parsed::Error(context) => Parsed::Error(context),
    }
  }

  /// Allow to quickly access context to map it.
  pub fn map_context<MappedAtom, Map>(self, map: Map) -> Parsed<Token, Stream, MappedAtom>
  where
    Map: FnOnce(Context) -> MappedAtom,
  {
    match self {
      Parsed::Success { token, stream } => Parsed::Success { token, stream },
      Parsed::Failure(context) => Parsed::Failure(map(context)),
      Parsed::Error(context) => Parsed::Error(map(context)),
    }
  }

  /// Shortcut to add a Atom to the Context
  pub fn add_context<C, Map>(self, map: Map) -> Parsed<Token, Stream, Context>
  where
    Map: FnOnce() -> C,
    Context: Contexting<C>,
  {
    match self {
      Parsed::Success { token, stream } => Parsed::Success { token, stream },
      Parsed::Failure(content) => Parsed::Failure(content.add(map())),
      Parsed::Error(content) => Parsed::Error(content.add(map())),
    }
  }

  /// Return Success if Parsed is Success otherwise panic.
  /// Use only for testing purpose.
  pub fn unwrap(self) -> Success<Token, Stream>
  where
    Context: Debug,
  {
    match self {
      Parsed::Success { token, stream } => Success { token, stream },
      Parsed::Failure(context) => panic!("Call unwrap on Parsed::Failure: {:?}", context),
      Parsed::Error(context) => panic!("Call unwrap on Parsed::Error: {:?}", context),
    }
  }

  /// Return Context if Parsed is Failure or Error otherwise panic.
  /// Use only for testing purpose.
  pub fn unwrap_context(self) -> Context
  where
    Token: Debug,
    Stream: Debug,
  {
    match self {
      Parsed::Success { token, stream } => {
        panic!("Call unwrap on Parsed::Success: {:?} {:?}", token, stream)
      }
      Parsed::Failure(context) => context,
      Parsed::Error(context) => context,
    }
  }

  /// Return true if Parsed is Success.
  pub const fn is_success(&self) -> bool {
    match self {
      Parsed::Success { .. } => true,
      _ => false,
    }
  }
}

impl<Token, Stream, Context> From<Success<Token, Stream>> for Parsed<Token, Stream, Context> {
  fn from(Success { token, stream }: Success<Token, Stream>) -> Self {
    Parsed::Success { token, stream }
  }
}

impl<Token, Stream, Context> FromResidual for Parsed<Token, Stream, Context> {
  fn from_residual(residual: Parsed<Infallible, Infallible, Context>) -> Self {
    match residual {
      Parsed::Success { .. } => unreachable!(),
      Parsed::Failure(context) => Parsed::Failure(context),
      Parsed::Error(context) => Parsed::Error(context),
    }
  }
}

impl<Token, Stream, Context> FromResidual<ParsedAux<Infallible, Context>>
  for Parsed<Token, Stream, Context>
{
  fn from_residual(residual: ParsedAux<Infallible, Context>) -> Self {
    match residual {
      ParsedAux::Success(_success) => unreachable!(),
      ParsedAux::Failure(context) => Parsed::Failure(context),
      ParsedAux::Error(context) => Parsed::Error(context),
    }
  }
}

impl<Token, Stream, Context> FromResidual<Result<Infallible, Context>>
  for Parsed<Token, Stream, Context>
{
  fn from_residual(residual: Result<Infallible, Context>) -> Self {
    match residual {
      Ok(_success) => unreachable!(),
      Err(context) => Parsed::Failure(context),
    }
  }
}

// impl<Token, Stream, Context> FromResidual<Infallible>
//   for Parsed<Token, Stream, Context>
// {
//   fn from_residual(_: Infallible) -> Self {
//     unreachable!()
//   }
// }

impl<Token, Stream, Context> Try for Parsed<Token, Stream, Context> {
  type Output = Success<Token, Stream>;
  type Residual = Parsed<Infallible, Infallible, Context>;

  fn from_output(output: Self::Output) -> Self {
    output.into()
  }

  fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
    match self {
      Parsed::Success { token, stream } => ControlFlow::Continue(Success { token, stream }),
      Parsed::Failure(context) => ControlFlow::Break(Parsed::Failure(context)),
      Parsed::Error(context) => ControlFlow::Break(Parsed::Error(context)),
    }
  }
}

use owo_colors::OwoColorize;

impl<Token, Stream, Context> Display for Parsed<Token, Stream, Context>
where
  Token: Debug,
  Stream: Streaming,
  Context: ProvideElement,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Parsed::Success { token, stream } => match stream.clone().split_at(8) {
        Split::Success {
          item: stream,
          stream: _,
        } => {
          write!(
            f,
            "{}: {:02X?}, stream: {:02X?} ..",
            "Success".green(),
            token,
            stream
          )
        }
        Split::NotEnoughItem(stream) => {
          write!(
            f,
            "{}: {:02X?}, stream: {:02X?}",
            "Success".green(),
            token,
            stream
          )
        }
        Split::Error(error) => {
          write!(
            f,
            "{}: {:02X?}, stream: {:?}",
            "Success".green(),
            token,
            error
          )
        }
      },
      Parsed::Failure(context) => {
        write!(f, "{}: {}", "Failure".yellow(), context.last())
      }
      Parsed::Error(context) => {
        write!(f, "{}: {}", "Error".red(), context.last())
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    Parsed,
    Success,
  };

  fn multiply_by_42(parsed: Parsed<u8, (), ()>) -> Parsed<u8, (), ()> {
    let Success { token, .. } = parsed?;
    Parsed::Success {
      token: token * 42,
      stream: (),
    }
  }

  #[test]
  fn parsed() {
    assert_eq!(
      multiply_by_42(Parsed::new_success(1, ())),
      Parsed::new_success(42, ())
    );
    assert_eq!(multiply_by_42(Parsed::Failure(())), Parsed::Failure(()));
    assert_eq!(multiply_by_42(Parsed::Error(())), Parsed::Error(()));
  }
}
