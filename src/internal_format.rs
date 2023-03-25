
type Color = crate::common::Color;

/// This represents the styling of text that we support as part of our output
/// The idea is that all writers must be able to output these particular styles
/// with out worrying about the other support by ANSI
#[derive(PartialEq, Eq, PartialOrd)]
enum Fromatting {
    Bold,
    Faint,
    Italic,
    Underline(Option<Color>),
    CrossedOut,
    ForgroundColor(Color),
    BackgroundColor(Color),
}

type TextElement = crate::common::TextElement<Fromatting>;
type Text = crate::common::Text<Fromatting>;
