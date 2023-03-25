/// A simple reprensentation of a 24 bit color code used is most apps
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// An element of a ANSI Complient string containing either a section of text or an escape sequence
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TextElement<T> {
    Text(String),
    Marker(T),
}

pub enum Toggle<T> {
    Set(T),
    UnSet(T),
}

/// This is a really annoying wrapper for the Text class as we can not extend
/// the base text
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Text<T> {
    pub text: Vec<TextElement<T>>,
}

impl Color {
    pub fn red(&self) -> u8 {
        return self.red.clone();
    }

    pub fn green(&self) -> u8 {
        return self.green.clone();
    }

    pub fn blue(&self) -> u8 {
        return self.blue.clone();
    }

    /// This converts the Color given into a bright color varient.
    pub fn make_bright(mut color: Color) -> Color {
        if color.green == 0 && color.blue == 0 && color.red == 0 {
            color.green = 128;
            color.blue = 128;
            color.red = 128;
            return color;
        }
        if color.blue != 0 {
            color.blue = 255
        }
        if color.green != 0 {
            color.green = 255
        }
        if color.red != 0 {
            color.red = 255
        }
        return color;
    }
}

/// Returns a Color that represents black
pub fn black() -> Color {
    Color {
        red: 0,
        green: 0,
        blue: 0,
    }
}

/// Returns a Color that represents red
pub fn red() -> Color {
    Color {
        red: 128,
        green: 0,
        blue: 0,
    }
}

/// Returns a Color that represents green
pub fn green() -> Color {
    Color {
        red: 0,
        green: 128,
        blue: 0,
    }
}

/// Returns a Color that represents yellow
pub fn yellow() -> Color {
    Color {
        red: 128,
        green: 128,
        blue: 0,
    }
}

/// Returns a Color that represents blue
pub fn blue() -> Color {
    Color {
        red: 0,
        green: 0,
        blue: 128,
    }
}

/// Returns a Color that represents magenta
pub fn magenta() -> Color {
    Color {
        red: 128,
        green: 0,
        blue: 128,
    }
}

/// Returns a Color that represents cyan
pub fn cyan() -> Color {
    Color {
        red: 0,
        green: 128,
        blue: 128,
    }
}

/// Returns a Color that represents white
pub fn white() -> Color {
    Color {
        red: 192,
        green: 192,
        blue: 192,
    }
}

pub mod test {

    #[test]
    fn color_back() {
        let color = super::black();
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 0);
    }

    #[test]
    fn color_red() {
        let color = super::red();
        assert_eq!(color.red(), 128);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 0);
    }

    #[test]
    fn color_green() {
        let color = super::green();
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 128);
        assert_eq!(color.blue(), 0);
    }

    #[test]
    fn color_blue() {
        let color = super::blue();
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 128);
    }

    #[test]
    fn color_yellow() {
        let color = super::yellow();
        assert_eq!(color.red(), 128);
        assert_eq!(color.green(), 128);
        assert_eq!(color.blue(), 0);
    }

    #[test]
    fn color_magenta() {
        let color = super::magenta();
        assert_eq!(color.red(), 128);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 128);
    }

    #[test]
    fn color_cyan() {
        let color = super::cyan();
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 128);
        assert_eq!(color.blue(), 128);
    }

    #[test]
    fn color_white() {
        let color = super::white();
        assert_eq!(color.red(), 192);
        assert_eq!(color.green(), 192);
        assert_eq!(color.blue(), 192);
    }

    #[test]
    fn color_make_bright() {
        let tests = [
            (
                super::black(),
                super::Color {
                    red: 128,
                    green: 128,
                    blue: 128,
                },
            ),
            (
                super::blue(),
                super::Color {
                    red: 0,
                    green: 0,
                    blue: 255,
                },
            ),
            (
                super::green(),
                super::Color {
                    red: 0,
                    green: 255,
                    blue: 0,
                },
            ),
            (
                super::red(),
                super::Color {
                    red: 255,
                    green: 0,
                    blue: 0,
                },
            ),
            (
                super::yellow(),
                super::Color {
                    red: 255,
                    green: 255,
                    blue: 0,
                },
            ),
            (
                super::cyan(),
                super::Color {
                    red: 0,
                    green: 255,
                    blue: 255,
                },
            ),
            (
                super::magenta(),
                super::Color {
                    red: 255,
                    green: 0,
                    blue: 255,
                },
            ),
            (
                super::white(),
                super::Color {
                    red: 255,
                    green: 255,
                    blue: 255,
                },
            ),
        ];
        for test in tests {
            let (mut color, expected_result) = test;
            color = super::Color::make_bright(color);
            assert_eq!(color.red(), expected_result.red());
            assert_eq!(color.green(), expected_result.green());
            assert_eq!(color.blue(), expected_result.blue());
        }
    }
}
