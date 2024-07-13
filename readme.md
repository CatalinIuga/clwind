# clwind - TailwindCSS-ish API for you CLI text

ANSII color codes are a pain to remember, and just like plain CSS, it can take more time to write than it should. This is a simple utility that allows you to add colors and styles to your CLI text with a TailwindCSS-like syntax.

## Installation

Get the module from []()

```bash
cargo add clwind
```

## Usage

```rs
use clwind::{clw, Color::Hex};

fn main() {
    let ansii_formated = clw("Hello, World!")
        .bg_bright_red()
        .text(Hex(0xf2f2f2))
        .font_bold()
        .font_underline();

    println!("{}", ansii_formated);
}
```

## Features

### Colors

- Variants: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`; and the `bright_` prefixes for each of them.
- Hex: `Hex(0xf2f2f2)`
- RGB: `RGB(255, 255, 255)`
- 256: `Color256(255)`

### Styles

- bold
- dim
- italic
- underline
- blink
- reverse
- hidden
- strikethrough

## License

MIT
