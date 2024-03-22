use std::{
  env,
  fs::OpenOptions,
  io::{
    self,
    BufWriter,
    Write,
  },
  num::NonZeroUsize,
  path::{
    Path,
    PathBuf,
  },
};

use quote::{
  format_ident,
  quote,
};
use syn::Index;

fn tuple<W: Write>(mut w: W, i: NonZeroUsize) -> Result<(), io::Error> {
  let i = i.get();
  let parsers = (0usize..i).map(|i| format_ident!("P{}", i));
  let tuple = parsers.clone();
  let generics = parsers.clone();
  let generics_where = parsers;
  let tuple = quote! {
    #(#tuple,)*
  };
  let generics = quote! {
    #(#generics),*
  };
  let tokens = (0usize..i).map(|i| format_ident!("O{}", i));
  let generics_where = quote! {
    #(#generics_where: Parse<Stream, Context, Token = #tokens>),*
  };

  let tokens2 = (0usize..i).map(|i| format_ident!("O{}", i));
  let tokens3 = (0usize..i).map(|i| format_ident!("O{}", i));
  let tokens4 = (0usize..i).map(|i| format_ident!("token_{}", i));

  let matches = (0usize..i).map(Index::from).map(|i| {
    let token = format_ident!("token_{}", i);
    quote! {
      let Success { token: #token, stream } = self.#i.parse(stream)?;
    }
  });

  let codegen = quote! {
    impl<#(#tokens2: Debug,)* #generics, Stream, Context> Parse<Stream, Context> for (#tuple)
    where
      Stream: Streaming,
      #generics_where
    {
      type Token = (#(#tokens3,)*);

      fn parse(&mut self, stream: Stream) -> Parsed<Self::Token, Stream, Context> {
        #(#matches)*

        Parsed::new_success((#(#tokens4,)*), stream)
      }
    }
  };

  write!(w, "{}", rustfmt_wrapper::rustfmt(codegen).unwrap())
}

fn tuples(path: &Path) -> Result<(), io::Error> {
  let dest_path = Path::new(path).join("parse_tuple.rs");
  let file = OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open(&dest_path)?;
  let mut buf = BufWriter::new(file);

  for i in 1..12 {
    tuple(&mut buf, i.try_into().unwrap())?;
  }

  Ok(())
}

fn main() -> Result<(), io::Error> {
  println!("cargo:rerun-if-changed=build.rs");

  let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
  tuples(&out_dir)?;

  Ok(())
}
