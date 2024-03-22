## Organization

* core contains all core traits of binator, implement them on std and Rust type.
* context contains structure that will hold the error in your parser, you can ignore them, use a stack of error or even have a full tree of all errors that your parsers generated.
* base contains basic combinator that you start from to make parser, for example, you want the ascii char 'i', you start with `is(b'i')`. Or number like "42", or binary form number (`u16_be`)
* utils contains combinator that you can use to control loop, valid data and more. Like you want as many `i` as possible `is(b'i').fold_bounds(.., || (), Acc::acc)`. When you get used to it this `fold_bounds` do everything you need.
* Stream contains structure that can be used as Stream.

## Example

The very same hex color example from nom but using binator:

```rust
use binator::{
  base::{
    is,
    uint_radix,
    IntRadixParse,
    Radix,
  },
  context::Ignore,
  core::{
    Parse,
    Parsed,
  },
  utils::Utils,
};

#[derive(Debug, PartialEq)]
pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

fn hex_primary<Stream, Context>(stream: Stream) -> Parsed<u8, Stream, Context>
where
  (): IntRadixParse<Stream, Context, u8>,
{
  uint_radix(2, Radix::HEX).parse(stream)
}

fn hex_color<Stream, Context>(stream: Stream) -> Parsed<Color, Stream, Context>
where
  (): IntRadixParse<Stream, Context, u8>,
{
  (is(b'#'), hex_primary, hex_primary, hex_primary)
    .map(|(_, red, green, blue)| Color { red, green, blue })
    .parse(stream)
}

assert_eq!(
  hex_color::<_, Ignore>.parse("#2F14DF".as_bytes()),
  Parsed::Success {
    stream: "".as_bytes(),
    token: Color {
      red: 0x2F,
      green: 0x14,
      blue: 0xDF,
    }
  }
);
```

Bigger example, a little json parser [here](https://github.com/binator/json), or a network packet parser [here](https://github.com/binator/network).

## Influence

- This project has been a lot influenced by [`nom`]. However, it's very different, require nightly and is very experimental while `nom` is way more stable.
- [`combine`] have also influenced this project but way less than `nom`.

## Difference with nom

binator use alias trait and try trait to provide a better experience, but this require nightly.

nom can handle both octet and char, binator only take octet. Don't run yet ! binator make the choice to include an utf8 combinator, this mean where in nom you need two versions of each combinator, one for character, one for octet, binator you just need one for octet, and you must use our utf8 combinator (or you can code yours) when you expect utf8 in your data. We do not want you to validate your data to be valid utf8 and then parse it. Also, for incomplete data is way better. Bonus, in theory this is faster.

Error in binator are way more flexible than in nom, you can create your own error, and there will be added to the pool of error of the big parser you are building. All error are flatten no matter where you create then, this mean your custom error is the same level as binator error, there is no difference between them. This is done with the work of generic that can make hard to work with binator. Nom choice to be more simple on that, limiting the customization of user error.

The core trait of binator is Streaming, the main operation of this trait is split_first, that will simply take one Item from your Stream, so 99% of time it's one octet from your data. While nom have multiple trait you need to implement to be able to use a custom Stream, binator there is only one, and very simple.

## Limitation

Currently, Array are used as "or" branch, if the array is empty (so there is no parser) it's make no sense cause Array parser need to return something so would need to have its own Error "empty array", it shouldn't be possible to use an empty array, but it is because we use const generic to impl Parse it's possible. However, it's VERY hard to write thus code, since compiler can't infer anything from an empty array alone, a user would REALLY need to force it. This will be removed when we can do more with const generic and will NOT be considered a breaking change at any point.

## Performance

While not being the primary goal it's still a goal, for now primary testing show it's similar to nom. So if your goal is peak performance maybe binator is not for you, but if your goal is "fast enough" binator should be ok. Some benchmark test would be welcome, there is already a json parser crate for binator.

## [License]

This project choice the [Zlib license] because it's almost like MIT, but it's more flexible on the inclusion of licenses in binary also it's include the share of modification. It's also constraint on forking, this mean one must not upload copy of this on [`crates.io`] without clearly state it's a fork and not the original.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above (Zlib licence), without any additional terms or conditions. Big contributor will eventually be added to author list.

[![Binator Contributors](https://contributors-img.web.app/image?repo=binator/self)](https://github.com/binator/self/graphs/contributors)

## Grammar

I'm clearly not an English native speaker, so I would accept PR that make documentation more clear, however, I don't want small correction like "US vs UK" version, I don't want PR that just remove space before "!" or "?", because I'm French and I like it that way. I want PR that respect the original author that write the sentence, but if you add new sentence use your own style. In summary, I will accept any PR that add clarity, but not grammar zealot PR.

[License]: license.md
[Zlib license]: https://choosealicense.com/licenses/zlib/
[`crates.io`]: https://crates.io
[`nom`]: https://github.com/Geal/nom
[`combine`]: https://github.com/Marwes/combine
