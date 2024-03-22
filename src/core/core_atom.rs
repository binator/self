use stdcore::fmt::{
  Debug,
  Display,
  Formatter,
};

use crate::core::*;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Core context used to implement context for basic type like u8
pub enum CoreAtom<Stream, Error = <Stream as Streaming>::Error> {
  /// Used when end of stream is reached.
  EndOfStream {
    /// The stream that returned end of stream.
    stream: Stream,
  },
  /// Used when stream return an Error.
  Error {
    /// the error returned by the stream.
    error: Error,
  },
}

impl<Stream: Streaming> Display for CoreAtom<Stream>
where
  Stream: Debug,
  Stream::Error: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::EndOfStream { .. } => write!(f, "End of stream"),
      Self::Error { error } => write!(f, "The stream have encounter an error {:?}", error,),
    }
  }
}
