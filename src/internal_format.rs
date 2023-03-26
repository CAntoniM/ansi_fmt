use crate::ansi;
use crate::common;

type Color = common::Color;
/// This represents the styling of text that we support as part of our output
/// The idea is that all writers must be able to output these particular styles
/// with out worrying about the other support by ANSI
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
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
    pub fn from_ansi(text: ansi::Text)  -> Text{
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
        return new_impl;
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    #[test]
    pub fn formatting_from_ansi() {
        let test_cases = vec![
            (crate::ansi::FeEscapeSequence::SingleShiftTwo, None),
            (crate::ansi::FeEscapeSequence::SingleShiftThree, None),
            (crate::ansi::FeEscapeSequence::DeviceControlString, None),
            (crate::ansi::FeEscapeSequence::OperatingSystemCommand, None),
            (crate::ansi::FeEscapeSequence::StringTerminator, None),
            (crate::ansi::FeEscapeSequence::StartOfString, None),
            (crate::ansi::FeEscapeSequence::PrivacyMessage, None),
            (
                crate::ansi::FeEscapeSequence::ApplicationProgramCommand,
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::SelectGraphicalRendition(
                        crate::ansi::SelectGraphicRendition::Italic,
                    ),
                ),
                Some(vec![crate::common::Toggle::Set(super::Fromatting::Italic)]),
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::CursorDown(3),
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::CursorForward(3),
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::CursorBack(3),
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::CursorNextLine(3),
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::CursorPreviousLine(3),
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::CursorHorizontalAbsolute(3),
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::CursorPosition(3, 4),
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::EraseInDisplay(3),
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::EraseInLine(4),
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::ScrollUp(4),
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::ScrollDown(4),
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::HorizonalVerticalPosition(4, 4),
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::AUXPortOn,
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::AUXPortOff,
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::DeviceStatusReport,
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::SaveCursorPosistion,
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::RestoreCursorPosistion,
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::VT220Cursor,
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::HideCursor,
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::EnableReportingFocus,
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::DisableReportingFocus,
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::EnableAltScreenBuf,
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::DisableAltScreenBuf,
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::BracketPasteMode,
                ),
                None,
            ),
            (
                crate::ansi::FeEscapeSequence::ControlSequence(
                    crate::ansi::ControlSequence::NoBracketPasteMode,
                ),
                None,
            ),
        ];
        for test_case in test_cases {
            let (input, expected_output) = test_case;
            assert_eq!(super::Fromatting::from_ansi(&input), expected_output)
        }
    }

    #[test]
    pub fn text_from_ansi() {
        let test_cases = vec![(
            crate::ansi::Text {
                text: vec![
                    crate::ansi::TextElement::Text("This".to_string()),
                    crate::ansi::TextElement::Marker(crate::ansi::FeEscapeSequence::SingleShiftTwo),
                    crate::ansi::TextElement::Marker(
                        crate::ansi::FeEscapeSequence::ControlSequence(
                            crate::ansi::ControlSequence::SelectGraphicalRendition(
                                crate::ansi::SelectGraphicRendition::ForgroundColor(Some(
                                    crate::common::red(),
                                )),
                            ),
                        ),
                    ),
                    crate::ansi::TextElement::Text("is a".to_string()),
                    crate::ansi::TextElement::Marker(
                        crate::ansi::FeEscapeSequence::ControlSequence(
                            crate::ansi::ControlSequence::SelectGraphicalRendition(
                                crate::ansi::SelectGraphicRendition::Bold,
                            ),
                        ),
                    ),
                    crate::ansi::TextElement::Text("Test".to_string()),
                    crate::ansi::TextElement::Marker(
                        crate::ansi::FeEscapeSequence::ControlSequence(
                            crate::ansi::ControlSequence::SelectGraphicalRendition(
                                crate::ansi::SelectGraphicRendition::Normal,
                            ),
                        ),
                    ),
                ],
            },
            super::Text {
                text: vec![
                    super::TextElement::Text("This".to_string()),
                    super::TextElement::Marker(crate::common::Toggle::Set(
                        super::Fromatting::ForgroundColor(Some(crate::common::red())),
                    )),
                    super::TextElement::Text("is a".to_string()),
                    super::TextElement::Marker(crate::common::Toggle::Set(super::Fromatting::Bold)),
                    super::TextElement::Text("Test".to_string()),
                    super::TextElement::Marker(crate::common::Toggle::UnSet(super::Fromatting::Bold)),
                    super::TextElement::Marker(crate::common::Toggle::UnSet(super::Fromatting::Faint)),
                    super::TextElement::Marker(crate::common::Toggle::UnSet(super::Fromatting::Italic)),
                    super::TextElement::Marker(crate::common::Toggle::UnSet(super::Fromatting::Underline(None))),
                    super::TextElement::Marker(crate::common::Toggle::UnSet(super::Fromatting::CrossedOut)),
                    super::TextElement::Marker(crate::common::Toggle::UnSet(super::Fromatting::ForgroundColor(None))),
                    super::TextElement::Marker(crate::common::Toggle::UnSet(super::Fromatting::BackgroundColor(None))),
                ],
            },
        )];
        for test_case in test_cases {
            let (input,expected_output) = test_case;
            assert_eq!(super::Text::from_ansi(input),expected_output)
        }
    }
}
