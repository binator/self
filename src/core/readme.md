Contains all require traits of binator, implement them on std and Rust type.

Binator define trait that structure your parser. For something to be considered as a Parser by binator it must implement [Parse] trait. This trait is used every time you use a Parser. This trait only have one method [Parse::parse], it takes a Stream as parameter. A Stream can be anything that implement [Streaming], for example binator implement it for `&'a [u8]`. Most of the time a Parser will use indirectly [Streaming::split_first] to get a [Streaming::Item] from the Stream. When a Parser is done with the input it will return [Parsed]. It's an enumeration that implement [core::ops::Try] so you can use `?` on a Parser, this enumeration is used to represent the result of a Parser. A Parser can return [Parsed::Success], [Parsed::Failure] or [Parsed::Error]. Success contains a Token, that what the Parser produced from the Stream, and a Stream that contains the input not used by the Parser. Failure means the parser didn't recognize the input, it's not a fatal error at all, it's perfectly normal for a combinator parser to return Failure. And then Error is a fatal Error, like an Error produced by the Stream or by a Parser. Both Failure and Error contains a Context. Context is something that implement [Contexting], it's the way binator accumulate Failure, Context is like a container of Failure. If a Parser need to return a context, it can use [Contexting::new] that require an Atom. Atom can be anything a Parser want, for example, core define [crate::base::FloatAtom]. [Contexting] require that the Context implement [core::ops::Add] and [core::ops::BitOr] this mean if you already called another Parser that return a Context you can add you own Atom and build a more precise Context for the final user. Most combinator of binator do this for you already. With all of this you know mostly all about how binator works.

## Terminology

### Stream

A structure that will produce `Item` when asked

### Parser 

Something that will check that `Item` produced by `Stream` are correct


### Context

A structure that will manage `Failure` and `Error` generate by `Parser`

### Token

Represent what a `Parser` return when `Success`

### Atom

A structure that contain information about the `Failure` or `Error` from a `Parser`

### Element

Something, generally an enumeration, that will contain all different kind of `Atom`

### Parsed

Enumeration that indicate result of a `Parser`

### Parse

A trait that all `Parser` implement, used to use a `Parser`

### Failure

Indicate a `Parser` didn't validate the input

### Success

Indicate a `Parser` validate the input

### Error

Indicate a `Parser` encounter an irrecoverable error.

### Streaming

A trait that `Stream` implement to make their job

### Item

`Item` produced by a `Stream`, generally just an `u8`

### Span

A delimited part of the `Stream`

### Contexting

A trait that all `Context` will implement, used to accumulate failure of `Parser`
