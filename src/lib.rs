use std::fmt;

/// Represents a color for terminal output.
#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    /// RGB color with red, green, and blue components.
    Rgb(u8, u8, u8),
    /// 256-color palette index.
    Color256(u8),
    // Hexadecimal color code.
    Hex(u32),
}

impl Color {
    fn to_ansi_code(self) -> String {
        match self {
            Color::Black => "30",
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::White => "37",
            Color::BrightBlack => "90",
            Color::BrightRed => "91",
            Color::BrightGreen => "92",
            Color::BrightYellow => "93",
            Color::BrightBlue => "94",
            Color::BrightMagenta => "95",
            Color::BrightCyan => "96",
            Color::BrightWhite => "97",
            Color::Rgb(r, g, b) => return format!("38;2;{};{};{}", r, g, b),
            Color::Color256(c) => return format!("38;5;{}", c),
            Color::Hex(h) => {
                let r = (h >> 16) as u8;
                let g = ((h >> 8) & 0xFF) as u8;
                let b = (h & 0xFF) as u8;
                println!("{} {} {}", r, g, b);
                return format!("38;2;{};{};{}", r, g, b);
            }
        }
        .to_string()
    }

    fn to_bg_ansi_code(self) -> String {
        match self {
            Color::Rgb(r, g, b) => format!("48;2;{};{};{}", r, g, b),
            Color::Color256(c) => format!("48;5;{}", c),
            Color::Hex(h) => {
                let r = (h >> 16) as u8;
                let g = ((h >> 8) & 0xFF) as u8;
                let b = (h & 0xFF) as u8;
                format!("48;2;{};{};{}", r, g, b)
            }
            _ => (self.to_ansi_code().parse::<u8>().unwrap_or(30) + 10).to_string(),
        }
    }
}

/// Represents a text style for terminal output.
#[derive(Clone, Copy)]
pub enum Style {
    Bold = 1,
    Dim = 2,
    Italic = 3,
    Underline = 4,
    Blink = 5,
    Reverse = 7,
    Hidden = 8,
    Strikethrough = 9,
}

impl Style {
    fn to_ansi_code(self) -> String {
        match self {
            Style::Bold => "1",
            Style::Dim => "2",
            Style::Italic => "3",
            Style::Underline => "4",
            Style::Blink => "5",
            Style::Reverse => "7",
            Style::Hidden => "8",
            Style::Strikethrough => "9",
        }
        .to_string()
    }
}

/// Represents a colored and styled text for terminal output.
pub struct CLW {
    value: String,
    bg: Option<Color>,
    text: Option<Color>,
    font: Vec<Style>,
}

impl CLW {
    /// ## Creates a new `CLW` instance with the given text.
    ///
    /// ### Arguments
    ///
    /// * `value` - The text to be styled
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let styled_text = clw("Hello, world!");
    /// ```
    fn new<S: Into<String>>(value: S) -> Self {
        CLW {
            value: value.into(),
            text: None,
            bg: None,
            font: Vec::new(),
        }
    }

    /// ## Sets the text color.
    ///
    /// ### Arguments
    ///
    /// * `color` - The color to set for the text
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::{clw, Color};
    ///
    /// let red_text = clw("Red text").text(Color::Red);
    /// let rgb_text = clw("RGB text").text(Color::Rgb(100, 150, 200));
    /// let color256_text = clw("256-color text").text(Color::Color256(118));
    /// ```
    pub fn text(mut self, color: Color) -> Self {
        self.text = Some(color);
        self
    }

    /// ## Sets the background color.
    ///
    /// ### Arguments
    ///
    /// * `color` - The color to set for the background
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::{clw, Color};
    ///
    /// let white_bg = clw("White background").bg(Color::White);
    /// let rgb_bg = clw("RGB background").bg(Color::Rgb(100, 150, 200));
    /// let color256_bg = clw("256-color background").bg(Color::Color256(118));
    /// ```
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// ## Adds a style to the text.
    ///
    /// ### Arguments
    ///
    /// * `style` - The style to add to the text
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::{clw, Style};
    ///
    /// let bold_text = clw("Bold text").font(Style::Bold);
    /// let italic_text = clw("Italic text").font(Style::Italic);
    /// let underline_text = clw("Underlined text").font(Style::Underline);
    /// ```
    pub fn font(mut self, style: Style) -> Self {
        self.font.push(style);
        self
    }

    /// ## Sets the text color to black.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let black_text = clw("Black text").text_black();
    ///
    /// println!("{}", black_text);
    /// ```
    pub fn text_black(self) -> Self {
        self.text(Color::Black)
    }

    /// ## Sets the text color to red.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let red_text = clw("Red text").text_red();
    ///
    /// println!("{}", red_text);
    /// ```
    pub fn text_red(self) -> Self {
        self.text(Color::Red)
    }

    /// ## Sets the text color to green.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let green_text = clw("Green text").text_green();
    ///
    /// println!("{}", green_text);
    ///
    /// ```
    pub fn text_green(self) -> Self {
        self.text(Color::Green)
    }

    /// ## Sets the text color to yellow.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let yellow_text = clw("Yellow text").text_yellow();
    ///
    /// println!("{}", yellow_text);
    /// ```
    pub fn text_yellow(self) -> Self {
        self.text(Color::Yellow)
    }

    /// ## Sets the text color to blue.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let blue_text = clw("Blue text").text_blue();
    ///
    /// println!("{}", blue_text);
    /// ```
    pub fn text_blue(self) -> Self {
        self.text(Color::Blue)
    }

    /// ## Sets the text color to magenta.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let magenta_text = clw("Magenta text").text_magenta();
    ///
    /// println!("{}", magenta_text);
    /// ```
    pub fn text_magenta(self) -> Self {
        self.text(Color::Magenta)
    }

    /// ## Sets the text color to cyan.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let cyan_text = clw("Cyan text").text_cyan();
    ///
    /// println!("{}", cyan_text);
    /// ```
    pub fn text_cyan(self) -> Self {
        self.text(Color::Cyan)
    }

    /// ## Sets the text color to white.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let white_text = clw("White text").text_white();
    ///
    /// println!("{}", white_text);
    /// ```
    pub fn text_white(self) -> Self {
        self.text(Color::White)
    }

    /// ## Sets the text color to bright black.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_black_text = clw("Bright black text").text_bright_black();
    ///
    /// println!("{}", bright_black_text);
    /// ```
    pub fn text_bright_black(self) -> Self {
        self.text(Color::BrightBlack)
    }

    /// ## Sets the text color to bright red.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_red_text = clw("Bright red text").text_bright_red();
    ///
    /// println!("{}", bright_red_text);
    /// ```
    pub fn text_bright_red(self) -> Self {
        self.text(Color::BrightRed)
    }

    /// ## Sets the text color to bright green.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_green_text = clw("Bright green text").text_bright_green();
    ///
    /// println!("{}", bright_green_text);
    /// ```
    pub fn text_bright_green(self) -> Self {
        self.text(Color::BrightGreen)
    }

    /// ## Sets the text color to bright yellow.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_yellow_text = clw("Bright yellow text").text_bright_yellow();
    ///
    /// println!("{}", bright_yellow_text);
    /// ```
    pub fn text_bright_yellow(self) -> Self {
        self.text(Color::BrightYellow)
    }

    /// ## Sets the text color to bright blue.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_blue_text = clw("Bright blue text").text_bright_blue();
    ///
    /// println!("{}", bright_blue_text);
    /// ```
    pub fn text_bright_blue(self) -> Self {
        self.text(Color::BrightBlue)
    }

    /// ## Sets the text color to bright magenta.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_magenta_text = clw("Bright magenta text").text_bright_magenta();
    ///
    /// println!("{}", bright_magenta_text);
    /// ```
    pub fn text_bright_magenta(self) -> Self {
        self.text(Color::BrightMagenta)
    }

    /// ## Sets the text color to bright cyan.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_cyan_text = clw("Bright cyan text").text_bright_cyan();
    ///
    /// println!("{}", bright_cyan_text);
    /// ```
    pub fn text_bright_cyan(self) -> Self {
        self.text(Color::BrightCyan)
    }

    /// ## Sets the text color to bright white.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_white_text = clw("Bright white text").text_bright_white();
    ///
    /// println!("{}", bright_white_text);
    /// ```
    pub fn text_bright_white(self) -> Self {
        self.text(Color::BrightWhite)
    }

    /// ## Sets the background color to black.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let black_bg = clw("Black background").bg_black();
    ///
    /// println!("{}", black_bg);
    /// ```
    pub fn bg_black(self) -> Self {
        self.bg(Color::Black)
    }

    /// ## Sets the background color to red.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let red_bg = clw("Red background").bg_red();
    ///
    /// println!("{}", red_bg);
    /// ```
    pub fn bg_red(self) -> Self {
        self.bg(Color::Red)
    }

    /// ## Sets the background color to green.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let green_bg = clw("Green background").bg_green();
    ///
    /// println!("{}", green_bg);
    /// ```
    pub fn bg_green(self) -> Self {
        self.bg(Color::Green)
    }

    /// ## Sets the background color to yellow.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let yellow_bg = clw("Yellow background").bg_yellow();
    ///
    /// println!("{}", yellow_bg);
    /// ```
    pub fn bg_yellow(self) -> Self {
        self.bg(Color::Yellow)
    }

    /// ## Sets the background color to blue.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let blue_bg = clw("Blue background").bg_blue();
    ///
    /// println!("{}", blue_bg);
    /// ```
    pub fn bg_blue(self) -> Self {
        self.bg(Color::Blue)
    }

    /// ## Sets the background color to magenta.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let magenta_bg = clw("Magenta background").bg_magenta();
    ///
    /// println!("{}", magenta_bg);
    /// ```
    pub fn bg_magenta(self) -> Self {
        self.bg(Color::Magenta)
    }

    /// ## Sets the background color to cyan.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let cyan_bg = clw("Cyan background").bg_cyan();
    ///
    /// println!("{}", cyan_bg);
    /// ```
    pub fn bg_cyan(self) -> Self {
        self.bg(Color::Cyan)
    }

    /// ## Sets the background color to white.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let white_bg = clw("White background").bg_white();
    ///
    /// println!("{}", white_bg);
    /// ```
    pub fn bg_white(self) -> Self {
        self.bg(Color::White)
    }

    /// ## Sets the background color to bright black.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_black_bg = clw("Bright black background").bg_bright_black();
    ///
    /// println!("{}", bright_black_bg);
    /// ```
    pub fn bg_bright_black(self) -> Self {
        self.bg(Color::BrightBlack)
    }

    /// ## Sets the background color to bright red.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_red_bg = clw("Bright red background").bg_bright_red();
    ///
    /// println!("{}", bright_red_bg);
    /// ```
    pub fn bg_bright_red(self) -> Self {
        self.bg(Color::BrightRed)
    }

    /// ## Sets the background color to bright green.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_green_bg = clw("Bright green background").bg_bright_green();
    ///
    /// println!("{}", bright_green_bg);
    /// ```
    pub fn bg_bright_green(self) -> Self {
        self.bg(Color::BrightGreen)
    }

    /// ## Sets the background color to bright yellow.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_yellow_bg = clw("Bright yellow background").bg_bright_yellow();
    ///
    /// println!("{}", bright_yellow_bg);
    /// ```
    pub fn bg_bright_yellow(self) -> Self {
        self.bg(Color::BrightYellow)
    }

    /// ## Sets the background color to bright blue.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_blue_bg = clw("Bright blue background").bg_bright_blue();
    ///
    /// println!("{}", bright_blue_bg);
    /// ```
    pub fn bg_bright_blue(self) -> Self {
        self.bg(Color::BrightBlue)
    }

    /// ## Sets the background color to bright magenta.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_magenta_bg = clw("Bright magenta background").bg_bright_magenta();
    ///
    /// println!("{}", bright_magenta_bg);
    /// ```
    pub fn bg_bright_magenta(self) -> Self {
        self.bg(Color::BrightMagenta)
    }

    /// ## Sets the background color to bright cyan.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_cyan_bg = clw("Bright cyan background").bg_bright_cyan();
    ///
    /// println!("{}", bright_cyan_bg);
    /// ```
    pub fn bg_bright_cyan(self) -> Self {
        self.bg(Color::BrightCyan)
    }

    /// ## Sets the background color to bright white.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bright_white_bg = clw("Bright white background").bg_bright_white();
    ///
    /// println!("{}", bright_white_bg);
    /// ```
    pub fn bg_bright_white(self) -> Self {
        self.bg(Color::BrightWhite)
    }

    /// ## Sets the font style to bold.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let bold_text = clw("Bold text").font_bold();
    ///
    /// println!("{}", bold_text);
    /// ```
    pub fn font_bold(self) -> Self {
        self.font(Style::Bold)
    }

    /// ## Sets the font style to dim.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let dim_text = clw("Dim text").font_dim();
    ///
    /// println!("{}", dim_text);
    /// ```
    pub fn font_dim(self) -> Self {
        self.font(Style::Dim)
    }

    /// ## Sets the font style to italic.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let italic_text = clw("Italic text").font_italic();
    ///
    /// println!("{}", italic_text);
    /// ```
    pub fn font_italic(self) -> Self {
        self.font(Style::Italic)
    }

    /// ## Sets the font style to underline.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let underline_text = clw("Underline text").font_underline();
    ///
    /// println!("{}", underline_text);
    /// ```
    pub fn font_underline(self) -> Self {
        self.font(Style::Underline)
    }

    /// ## Sets the font style to blink.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let blink_text = clw("Blinking text").font_blink();
    ///
    /// println!("{}", blink_text);
    /// ```
    pub fn font_blink(self) -> Self {
        self.font(Style::Blink)
    }

    /// ## Sets the font style to reverse.
    ///
    /// This will swap the text and background colors.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let reverse_text = clw("Reversed text").font_reverse();
    ///
    /// println!("{}", reverse_text);
    /// ```
    pub fn font_reverse(self) -> Self {
        self.font(Style::Reverse)
    }

    /// ## Sets the font style to hidden.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let hidden_text = clw("Hidden text").font_hidden();
    ///
    /// println!("{}", hidden_text);
    /// ```
    pub fn font_hidden(self) -> Self {
        self.font(Style::Hidden)
    }

    /// ## Sets the font style to strikethrough.
    ///
    /// ### Examples
    ///
    /// ```
    /// use clwind::clw;
    ///
    /// let strikethrough_text = clw("Strikethrough text").font_strikethrough();
    ///
    /// println!("{}", strikethrough_text);
    /// ```
    pub fn font_strikethrough(self) -> Self {
        self.font(Style::Strikethrough)
    }

    pub fn print(&self) {
        print!("{}", self);
    }

    pub fn println(&self) {
        println!("{}", self);
    }

    pub fn eprint(&self) {
        eprint!("{}", self);
    }

    pub fn eprintln(&self) {
        eprintln!("{}", self);
    }
}

impl fmt::Display for CLW {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut codes = Vec::new();
        if let Some(color) = self.text {
            codes.push(color.to_ansi_code());
        }

        if let Some(color) = self.bg {
            codes.push(color.to_bg_ansi_code());
        }

        for style in &self.font {
            codes.push(style.to_ansi_code());
        }

        match codes.len() {
            0 => write!(f, "{}", self.value),
            _ => write!(f, "\x1b[{}m{}\x1b[0m", codes.join(";"), self.value),
        }
    }
}

/// #### Creates a new `CLW` instance with the given text.
///
/// This is a convenience function for `CLW::new()`.
///
/// ##### Arguments
///
/// * `str` - The text to be styled
///
/// ##### Examples
///
/// ```
/// use clwind::clw;
///
/// let styled_text = clw("Styled")
///     .bg_bright_black()
///     .text_cyan()
///     .font_bold()
///     .font_blink();
///
/// println!("{}", styled_text);
/// ```
pub fn clw(str: &str) -> CLW {
    CLW::new(str)
}
