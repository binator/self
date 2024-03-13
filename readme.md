## Example

The very same hex color example from nom but using binator:

```rust
use binator::{
  base::is,
  context::Ignore,
  core::{
    Parse,
    Parsed,
  },
  number::{
    uint_radix,
    IntRadixParse,
    Radix,
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

## Influence

- This project has been a lot influenced by [`nom`]. However, it's very different, require nightly and is very experimental while `nom` is way more stable.
- [`combine`] have also influenced this project but way less than `nom`.

## Limitation

Currently, Array are used as "or" branch, if the array is empty (so there is no parser) it's make no sense cause Array parser need to return something so would need to have its own Error "empty array", it shouldn't be possible to use an empty array, but it is because we use const generic to impl Parse it's possible. However, it's VERY hard to write thus code, since compiler can't infer anything from an empty array alone, a user would REALLY need to force it. This will be removed when we can do more with const generic and will NOT be considered a breaking change at any point.

## Performance

While not being the primary goal it's still a goal, for now primary testing show it's similar to nom. So if your goal is peak performance maybe binator is not for you, but if your goal is "fast enough" binator should be ok. Some benchmark test would be welcome, there is already a json parser crate for binator.

## [License]

This project choice the [Zlib license] because it's almost like MIT, but it's more flexible on the inclusion of licenses in binary also it's include the share of modification. It's also constraint on forking, this mean one must not upload copy of this on [`crates.io`] without clearly state it's a fork and not the original.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above (Zlib licence), without any additional terms or conditions. Big contributor will eventually be added to author list.

[![Binator Contributors](https://contributors-img.web.app/image?repo=Stargateur/binator)](https://github.com/Stargateur/binator/graphs/contributors)

## Grammar

I'm clearly not an English native speaker, so I would accept PR that make documentation more clear, however, I don't want small correction like "US vs UK" version, I don't want PR that just remove space before "!" or "?", because I'm French and I like it that way. I want PR that respect the original author that write the sentence, but if you add new sentence use your own style. In summary, I will accept any PR that add clarity, but not grammar zealot PR.

[License]: license.md
[Zlib license]: https://choosealicense.com/licenses/zlib/
[`crates.io`]: https://crates.io
[`nom`]: https://github.com/Geal/nom
[`combine`]: https://github.com/Marwes/combine
