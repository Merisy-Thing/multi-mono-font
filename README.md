# Multi Mono Font

A Rust crate for mixed typesetting of different sized monospaced fonts within the same text line for embedded-graphics.

## Features

- Render multiple font sizes in a single line of text
- Based on embedded-graphics MonoFonts
- Optimized memory usage with u8 types
- Support for RLE font compression
- Static text alignment and baseline options

## Usage

```rust
const MULTI_STYLE: MultiMonoTextStyle<Rgb565> = MultiMonoTextStyle::new(
    &[&UPPER_FONT, &HZ_FONT, &LOWER_FONT],
    MultiMonoLineHeight::Max,
    Rgb565::WHITE,
);

let _next = Text::new("测HElLo试OK\n", Point::new(0, 32), MULTI_STYLE)
    .draw(&mut disp)
    .unwrap();
```

See [examples/mono.rs](examples/mono.rs) for more details.

## License

MIT
