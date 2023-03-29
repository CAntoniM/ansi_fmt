use crate::common;
use crate::input_fmt::ansi;

type Color = common::Color;
/// This represents the styling of text that we support as part of our output
/// The idea is that all writers must be able to output these particular styles
/// with out worrying about the other support by ANSI
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum Fromatting {
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
                ansi::ControlSequence::SelectGraphicalRendition(sgrs) => {
                    let mut results: Vec<common::Toggle<Fromatting>> = Vec::new();
                    for sgr in sgrs {
                        match sgr {
                            ansi::SelectGraphicRendition::Bold => {
                                results.push(common::Toggle::Set(Fromatting::Bold))
                            }
                            ansi::SelectGraphicRendition::Faint => {
                                results.push(common::Toggle::Set(Fromatting::Faint))
                            }
                            ansi::SelectGraphicRendition::Underline => {
                                results.push(common::Toggle::Set(Fromatting::Underline(None)))
                            }
                            ansi::SelectGraphicRendition::Italic => {
                                results.push(common::Toggle::Set(Fromatting::Italic))
                            }
                            ansi::SelectGraphicRendition::CrossedOut => {
                                results.push(common::Toggle::Set(Fromatting::CrossedOut))
                            }
                            ansi::SelectGraphicRendition::ForgroundColor(None) => results
                                .push(common::Toggle::UnSet(Fromatting::ForgroundColor(None))),
                            ansi::SelectGraphicRendition::BackgroundColor(None) => results
                                .push(common::Toggle::UnSet(Fromatting::BackgroundColor(None))),
                            ansi::SelectGraphicRendition::ForgroundColor(Some(color)) => results
                                .push(common::Toggle::Set(Fromatting::ForgroundColor(Some(
                                    color.clone(),
                                )))),
                            ansi::SelectGraphicRendition::BackgroundColor(Some(color)) => results
                                .push(common::Toggle::Set(Fromatting::BackgroundColor(Some(
                                    color.clone(),
                                )))),
                            ansi::SelectGraphicRendition::Normal => {
                                results.push(common::Toggle::UnSet(Fromatting::Bold));
                                results.push(common::Toggle::UnSet(Fromatting::Faint));
                                results.push(common::Toggle::UnSet(Fromatting::Italic));
                                results.push(common::Toggle::UnSet(Fromatting::Underline(None)));
                                results.push(common::Toggle::UnSet(Fromatting::CrossedOut));
                                results
                                    .push(common::Toggle::UnSet(Fromatting::ForgroundColor(None)));
                                results
                                    .push(common::Toggle::UnSet(Fromatting::BackgroundColor(None)));
                            }
                            ansi::SelectGraphicRendition::NormalIntensity => {
                                results.push(common::Toggle::UnSet(Fromatting::Bold));
                                results.push(common::Toggle::UnSet(Fromatting::Faint));
                            }
                            ansi::SelectGraphicRendition::NotUnderlined => {
                                results.push(common::Toggle::UnSet(Fromatting::Underline(None)))
                            }
                            ansi::SelectGraphicRendition::NotItalic => {
                                results.push(common::Toggle::UnSet(Fromatting::Italic))
                            }
                            ansi::SelectGraphicRendition::NotCrossedOut => {
                                results.push(common::Toggle::UnSet(Fromatting::CrossedOut))
                            }
                            ansi::SelectGraphicRendition::SetUnderlineColor(None) => {
                                results.push(common::Toggle::UnSet(Fromatting::Underline(None)))
                            }
                            _ => {}
                        };
                    }
                    return Some(results);
                }
                _ => None,
            },
            _ => None,
        };
    }
}

pub type TextElement = common::TextElement<common::Toggle<Fromatting>>;

pub type Text = common::Text<common::Toggle<Fromatting>>;

impl Text {
    pub fn from_ansi(text: ansi::Text) -> Text {
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
    use crate::{common, input_fmt::ansi};
    use std::vec;

    #[test]
    pub fn formatting_from_ansi() {
        let test_cases = vec![
            (ansi::FeEscapeSequence::SingleShiftTwo, None),
            (ansi::FeEscapeSequence::SingleShiftThree, None),
            (ansi::FeEscapeSequence::DeviceControlString, None),
            (ansi::FeEscapeSequence::OperatingSystemCommand, None),
            (ansi::FeEscapeSequence::StringTerminator, None),
            (ansi::FeEscapeSequence::StartOfString, None),
            (ansi::FeEscapeSequence::PrivacyMessage, None),
            (ansi::FeEscapeSequence::ApplicationProgramCommand, None),
            (
                ansi::FeEscapeSequence::ControlSequence(
                    ansi::ControlSequence::SelectGraphicalRendition(vec![
                        ansi::SelectGraphicRendition::Italic,
                    ]),
                ),
                Some(vec![crate::common::Toggle::Set(super::Fromatting::Italic)]),
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::CursorDown(3)),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::CursorForward(3)),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::CursorBack(3)),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::CursorNextLine(3)),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::CursorPreviousLine(
                    3,
                )),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(
                    ansi::ControlSequence::CursorHorizontalAbsolute(3),
                ),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::CursorPosition(
                    3, 4,
                )),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::EraseInDisplay(3)),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::EraseInLine(4)),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::ScrollUp(4)),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::ScrollDown(4)),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(
                    ansi::ControlSequence::HorizonalVerticalPosition(4, 4),
                ),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::AUXPortOn),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::AUXPortOff),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::DeviceStatusReport),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::SaveCursorPosistion),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(
                    ansi::ControlSequence::RestoreCursorPosistion,
                ),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::VT220Cursor),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::HideCursor),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(
                    ansi::ControlSequence::EnableReportingFocus,
                ),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(
                    ansi::ControlSequence::DisableReportingFocus,
                ),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::EnableAltScreenBuf),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::DisableAltScreenBuf),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::BracketPasteMode),
                None,
            ),
            (
                ansi::FeEscapeSequence::ControlSequence(ansi::ControlSequence::NoBracketPasteMode),
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
            ansi::Text {
                text: vec![
                    ansi::TextElement::Text("This".to_string()),
                    ansi::TextElement::Marker(ansi::FeEscapeSequence::SingleShiftTwo),
                    ansi::TextElement::Marker(ansi::FeEscapeSequence::ControlSequence(
                        ansi::ControlSequence::SelectGraphicalRendition(vec![
                            ansi::SelectGraphicRendition::ForgroundColor(Some(common::red())),
                        ]),
                    )),
                    ansi::TextElement::Text("is a".to_string()),
                    ansi::TextElement::Marker(ansi::FeEscapeSequence::ControlSequence(
                        ansi::ControlSequence::SelectGraphicalRendition(vec![
                            ansi::SelectGraphicRendition::Bold,
                        ]),
                    )),
                    ansi::TextElement::Text("Test".to_string()),
                    ansi::TextElement::Marker(ansi::FeEscapeSequence::ControlSequence(
                        ansi::ControlSequence::SelectGraphicalRendition(vec![
                            ansi::SelectGraphicRendition::Normal,
                        ]),
                    )),
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
                    super::TextElement::Marker(crate::common::Toggle::UnSet(
                        super::Fromatting::Bold,
                    )),
                    super::TextElement::Marker(crate::common::Toggle::UnSet(
                        super::Fromatting::Faint,
                    )),
                    super::TextElement::Marker(crate::common::Toggle::UnSet(
                        super::Fromatting::Italic,
                    )),
                    super::TextElement::Marker(crate::common::Toggle::UnSet(
                        super::Fromatting::Underline(None),
                    )),
                    super::TextElement::Marker(crate::common::Toggle::UnSet(
                        super::Fromatting::CrossedOut,
                    )),
                    super::TextElement::Marker(crate::common::Toggle::UnSet(
                        super::Fromatting::ForgroundColor(None),
                    )),
                    super::TextElement::Marker(crate::common::Toggle::UnSet(
                        super::Fromatting::BackgroundColor(None),
                    )),
                ],
            },
        )];
        for test_case in test_cases {
            let (input, expected_output) = test_case;
            assert_eq!(super::Text::from_ansi(input), expected_output)
        }
    }
}
