use core::fmt::{
  Display,
  Formatter,
};

/// This represent a stand alone Success from Parsed result.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Success<Token, Stream> {
  /// The rest of the stream
  pub stream: Stream,
  /// The token produced by the parser
  pub token: Token,
}

impl<Token, Stream> Success<Token, Stream> {
  /// Allow to quickly access token to map it.
  pub fn map_token<MappedToken, Map>(self, map: Map) -> Success<MappedToken, Stream>
  where
    Map: FnOnce(Token) -> MappedToken,
  {
    Success {
      token: map(self.token),
      stream: self.stream,
    }
  }

  /// Consume self and return token
  pub fn into_token(self) -> Token {
    self.token
  }

  /// Allow to quickly stream token to map it.
  pub fn map_stream<MappedStream, Map>(self, map: Map) -> Success<Token, MappedStream>
  where
    Map: FnOnce(Stream) -> MappedStream,
  {
    Success {
      token: self.token,
      stream: map(self.stream),
    }
  }

  /// Consume self and return stream
  pub fn into_stream(self) -> Stream {
    self.stream
  }
}

impl<Token, Stream> Display for Success<Token, Stream>
where
  Token: Display,
  Stream: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "Success: token: {} stream: {}", self.token, self.stream)
  }
}
