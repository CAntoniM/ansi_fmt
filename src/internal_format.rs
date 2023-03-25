use crate::ansi;
use crate::common;

type Color = common::Color;
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
    ForgroundColor(Option<Color>),
    BackgroundColor(Option<Color>),
}

impl Fromatting {
    pub fn from_ansi(fe: &ansi::FeEscapeSequence) -> Option<Vec<common::Toggle<Fromatting>>> {
        return match fe {
            ansi::FeEscapeSequence::ControlSequence(cls) => match cls {
                ansi::ControlSequence::SelectGraphicalRendition(sgr) => match sgr {
                    ansi::SelectGraphicRendition::Bold => {
                        Some(vec![common::Toggle::Set(Fromatting::Bold)])
                    }
                    ansi::SelectGraphicRendition::Faint => {
                        Some(vec![common::Toggle::Set(Fromatting::Faint)])
                    }
                    ansi::SelectGraphicRendition::Underline => {
                        Some(vec![common::Toggle::Set(Fromatting::Underline(None))])
                    }
                    ansi::SelectGraphicRendition::Italic => {
                        Some(vec![common::Toggle::Set(Fromatting::Italic)])
                    }
                    ansi::SelectGraphicRendition::CrossedOut => {
                        Some(vec![common::Toggle::Set(Fromatting::CrossedOut)])
                    }
                    ansi::SelectGraphicRendition::ForgroundColor(None) => {
                        Some(vec![common::Toggle::UnSet(Fromatting::ForgroundColor(
                            None,
                        ))])
                    }
                    ansi::SelectGraphicRendition::BackgroundColor(None) => {
                        Some(vec![common::Toggle::UnSet(Fromatting::BackgroundColor(
                            None,
                        ))])
                    }
                    ansi::SelectGraphicRendition::ForgroundColor(Some(color)) => {
                        Some(vec![common::Toggle::Set(Fromatting::ForgroundColor(Some(
                            color.clone(),
                        )))])
                    }
                    ansi::SelectGraphicRendition::BackgroundColor(Some(color)) => {
                        Some(vec![common::Toggle::Set(Fromatting::BackgroundColor(
                            Some(color.clone()),
                        ))])
                    }
                    ansi::SelectGraphicRendition::Normal => Some(vec![
                        common::Toggle::UnSet(Fromatting::Bold),
                        common::Toggle::UnSet(Fromatting::Faint),
                        common::Toggle::UnSet(Fromatting::Italic),
                        common::Toggle::UnSet(Fromatting::Underline(None)),
                        common::Toggle::UnSet(Fromatting::CrossedOut),
                        common::Toggle::UnSet(Fromatting::ForgroundColor(None)),
                        common::Toggle::UnSet(Fromatting::BackgroundColor(None)),
                    ]),
                    ansi::SelectGraphicRendition::NormalIntensity => Some(vec![
                        common::Toggle::UnSet(Fromatting::Bold),
                        common::Toggle::UnSet(Fromatting::Faint),
                    ]),
                    ansi::SelectGraphicRendition::NotUnderlined => {
                        Some(vec![common::Toggle::UnSet(Fromatting::Underline(None))])
                    }
                    ansi::SelectGraphicRendition::NotItalic => {
                        Some(vec![common::Toggle::UnSet(Fromatting::Italic)])
                    }
                    ansi::SelectGraphicRendition::NotCrossedOut => {
                        Some(vec![common::Toggle::UnSet(Fromatting::CrossedOut)])
                    }
                    ansi::SelectGraphicRendition::SetUnderlineColor(None) => {
                        Some(vec![common::Toggle::UnSet(Fromatting::Underline(None))])
                    }
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        };
    }
}

type TextElement = common::TextElement<common::Toggle<Fromatting>>;

type Text = common::Text<common::Toggle<Fromatting>>;

impl Text {
    pub fn from_ansi(text: ansi::Text) {
        let mut new_impl = Text { text: Vec::new() };
        for element in text.text.iter() {
            match element {
                ansi::TextElement::Text(string) => {
                    new_impl.text.push(TextElement::Text(string.clone()));
                }
                ansi::TextElement::Marker(marker) => match Fromatting::from_ansi(marker) {
                    Some(fmts) => {
                        for fmt in fmts {
                            new_impl.text.push(TextElement::Marker(fmt))
                        }
                    }
                    None => {}
                },
            }
        }
    }
}
