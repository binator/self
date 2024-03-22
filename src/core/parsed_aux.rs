use core::{
  convert::Infallible,
  ops::{
    ControlFlow,
    FromResidual,
    Try,
  },
};

/// This is like Parsed but Succeed doesn't contain stream
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParsedAux<Token, Context> {
  /// Success without Stream
  Success(Token),
  /// Failure
  Failure(Context),
  /// Error
  Error(Context),
}

impl<Token, Context> FromResidual for ParsedAux<Token, Context> {
  fn from_residual(residual: ParsedAux<Infallible, Context>) -> Self {
    match residual {
      ParsedAux::Success(_success) => unreachable!(),
      ParsedAux::Failure(context) => ParsedAux::Failure(context),
      ParsedAux::Error(context) => ParsedAux::Error(context),
    }
  }
}

impl<Token, Context> FromResidual<Result<Infallible, Context>> for ParsedAux<Token, Context> {
  fn from_residual(residual: Result<Infallible, Context>) -> Self {
    match residual {
      Ok(_success) => unreachable!(),
      Err(context) => ParsedAux::Failure(context),
    }
  }
}

impl<Token, Context> Try for ParsedAux<Token, Context> {
  type Output = Token;
  type Residual = ParsedAux<Infallible, Context>;

  fn from_output(output: Self::Output) -> Self {
    Self::Success(output)
  }

  fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
    match self {
      Self::Success(token) => ControlFlow::Continue(token),
      Self::Failure(context) => ControlFlow::Break(ParsedAux::Failure(context)),
      Self::Error(context) => ControlFlow::Break(ParsedAux::Error(context)),
    }
  }
}

// pub struct NoFail<T>(pub T);

// impl<T> FromResidual for NoFail<T> {
//     fn from_residual(_: <Self as Try>::Residual) -> Self {
//       unreachable!()
//     }
// }

// impl<T> Try for NoFail<T> {
//     type Output = T;

//     type Residual = Infallible;

//     fn from_output(output: Self::Output) -> Self {
//         Self(output)
//     }

//     fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
//       ControlFlow::Continue(self.0)
//     }
// }
