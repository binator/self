use paste::paste;

use crate::{
  base::octet,
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

/// Meta trait for number
pub trait NumberParse<Stream, Context> = where
  Stream: Streaming + Eq,
  <Stream as Streaming>::Item: Into<u8>,
  Context: Contexting<UtilsAtom<Stream>>,
  Context: Contexting<CoreAtom<Stream>>;

macro_rules! impl_primitive {
  ($primitive:ident) => {
    paste! {
      /// Parse binary $primitive in big endian
      #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all, ret(Display))
      )]
      pub fn [<$primitive _be>]<Stream, Context>(
        stream: Stream,
      ) -> Parsed<$primitive, Stream, Context>
      where
        (): NumberParse<Stream, Context>,
      {
        octet.fill().map($primitive::from_be_bytes).parse(stream)
      }

      /// Parse binary $primitive in little endian
      #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all, ret(Display))
      )]
      pub fn [<$primitive _le>]<Stream, Context>(
        stream: Stream,
      ) -> Parsed<$primitive, Stream, Context>
      where
        (): NumberParse<Stream, Context>,
      {
        octet.fill().map($primitive::from_le_bytes).parse(stream)
      }

      /// Parse binary $primitive in native endian
      #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all, ret(Display))
      )]
      pub fn [<$primitive _ne>]<Stream, Context>(
        stream: Stream,
      ) -> Parsed<$primitive, Stream, Context>
      where
        (): NumberParse<Stream, Context>,
      {
        octet.fill().map($primitive::from_ne_bytes).parse(stream)
      }
    }
  };
}

macro_rules! impl_primitives {
  ($($primitives:ident,)*) => {
    $(impl_primitive!{$primitives})*
  };
}

impl_primitives!(
  u16, u32, u64, u128, i16, i32, i64, i128, f32, f64, usize, isize,
);
