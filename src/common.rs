/// A simple reprensentation of a 24 bit color code used is most apps
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
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

enum Toggle<T> {
    Set(T),
    UnSet(T),
}

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
    /// Returns a Color that represents black
    pub fn Black() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    /// Returns a Color that represents red
    pub fn Red() -> Color {
        Color {
            red: 128,
            green: 0,
            blue: 0,
        }
    }

    /// Returns a Color that represents green
    pub fn Green() -> Color {
        Color {
            red: 0,
            green: 128,
            blue: 0,
        }
    }

    /// Returns a Color that represents yellow
    pub fn Yellow() -> Color {
        Color {
            red: 128,
            green: 128,
            blue: 0,
        }
    }

    /// Returns a Color that represents blue
    pub fn Blue() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 128,
        }
    }

    /// Returns a Color that represents magenta
    pub fn Magenta() -> Color {
        Color {
            red: 128,
            green: 0,
            blue: 128,
        }
    }

    /// Returns a Color that represents cyan
    pub fn Cyan() -> Color {
        Color {
            red: 0,
            green: 128,
            blue: 128,
        }
    }

    /// Returns a Color that represents white
    pub fn White() -> Color {
        Color {
            red: 192,
            green: 192,
            blue: 192,
        }
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

pub mod test {
    use crate::common::Color;

    #[test]
    fn color_back() {
        let color = Color::Black();
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 0);
    }

    #[test]
    fn color_red() {
        let color = Color::Red();
        assert_eq!(color.red(), 128);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 0);
    }

    #[test]
    fn color_green() {
        let color = Color::Green();
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 128);
        assert_eq!(color.blue(), 0);
    }

    #[test]
    fn color_blue() {
        let color = Color::Blue();
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 128);
    }

    #[test]
    fn color_yellow() {
        let color = Color::Yellow();
        assert_eq!(color.red(), 128);
        assert_eq!(color.green(), 128);
        assert_eq!(color.blue(), 0);
    }

    #[test]
    fn color_magenta() {
        let color = Color::Magenta();
        assert_eq!(color.red(), 128);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 128);
    }

    #[test]
    fn color_cyan() {
        let color = Color::Cyan();
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 128);
        assert_eq!(color.blue(), 128);
    }

    #[test]
    fn color_white() {
        let color = Color::White();
        assert_eq!(color.red(), 192);
        assert_eq!(color.green(), 192);
        assert_eq!(color.blue(), 192);
    }

    #[test]
    fn color_make_bright() {
        let tests = [
            (
                Color::Black(),
                Color {
                    red: 128,
                    green: 128,
                    blue: 128,
                },
            ),
            (
                Color::Blue(),
                Color {
                    red: 0,
                    green: 0,
                    blue: 255,
                },
            ),
            (
                Color::Green(),
                Color {
                    red: 0,
                    green: 255,
                    blue: 0,
                },
            ),
            (
                Color::Red(),
                Color {
                    red: 255,
                    green: 0,
                    blue: 0,
                },
            ),
            (
                Color::Yellow(),
                Color {
                    red: 255,
                    green: 255,
                    blue: 0,
                },
            ),
            (
                Color::Cyan(),
                Color {
                    red: 0,
                    green: 255,
                    blue: 255,
                },
            ),
            (
                Color::Magenta(),
                Color {
                    red: 255,
                    green: 0,
                    blue: 255,
                },
            ),
            (
                Color::White(),
                Color {
                    red: 255,
                    green: 255,
                    blue: 255,
                },
            ),
        ];
        for test in tests {
            let (mut color, expected_result) = test;
            color = Color::make_bright(color);
            assert_eq!(color.red(), expected_result.red());
            assert_eq!(color.green(), expected_result.green());
            assert_eq!(color.blue(), expected_result.blue());
        }
    }
}
