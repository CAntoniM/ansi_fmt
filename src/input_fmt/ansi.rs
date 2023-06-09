use crate::common;
use std::str::Chars;
/// This is an alias for the ASCII Escape character
static ESC: char = 0x1B as char;

pub type Color = common::Color;

impl Color {
    pub fn from_index(index: u8) -> Option<Color> {
        return match index {
            0 => Some(common::black()),
            1 => Some(common::red()),
            2 => Some(common::green()),
            3 => Some(common::yellow()),
            4 => Some(common::blue()),
            5 => Some(common::magenta()),
            6 => Some(common::cyan()),
            7 => Some(common::white()),
            _ => None,
        };
    }

    /// Converts the args used as part of SelectGraphicsRendition into a 24bit Color variable
    ///
    /// The function expects that the user is passing in arguments in the on of the following structure
    ///
    /// 1. a 2 followed by a red green or blue
    /// 2. a 5 followed by a 8 bit color code
    ///
    /// if arguments are not provided it will assume a value or 0 for these arguments it will only
    /// return 0 if the color mode provided as the first argument is not present or recognised
    pub fn from_args(args: &mut Vec<u8>) -> Option<Color> {
        return match args.pop() {
            Some(arg) => match arg {
                2 => Some(Color {
                    red: args.pop().unwrap_or(0),
                    green: args.pop().unwrap_or(0),
                    blue: args.pop().unwrap_or(0),
                }),
                5 => match args.pop() {
                    Some(color) => Some(Color {
                        red: (color >> 5) * 32,
                        green: ((color & 28) >> 2) * 32,
                        blue: (color & 3) * 32,
                    }),
                    None => Some(common::black()),
                },
                _ => None,
            },
            None => None,
        };
    }
}

/// this is a representation of the SelectGraphicsRendidtion sequences that are commonly used in terminals.
/// we should aim to be as accurate to the most terminals as possible and cover as many values as possible
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum SelectGraphicRendition {
    Normal,
    Bold,
    Faint,
    Italic,
    Underline,
    SlowBlink,
    RapidBlink,
    Invert,
    Conceal,
    CrossedOut,
    Font(u8),
    DoublyUnderlined,
    NormalIntensity,
    NotItalic,
    NotUnderlined,
    NotBlinking,
    ProportionalSpacing,
    NotReveresed,
    Reveal,
    NotCrossedOut,
    ForgroundColor(Option<Color>),
    BackgroundColor(Option<Color>),
    DisableProportionalSpacing,
    Framed,
    Encircled,
    Overlined,
    NeitherFramedNorEncircled,
    NotOverlined,
    SetUnderlineColor(Option<Color>),
    IdeogramUnderline,
    IdeogramDoubleUnderline,
    IdeogramOverline,
    IdeogramDoubleOverline,
    IdeogramStressMarking,
    NoIdeogram,
    Superscript,
    Subscript,
    NethirSuperOrSubScript,
}

impl SelectGraphicRendition {
    /// This will parse the args of the SelectGraphics rendition into the concreate values used internally
    /// This is expected to that either a number to represent a particular graphics change or non will be
    /// proivded at which point it will revert to the default settings
    pub fn from(args: &mut Vec<u8>) -> Vec<SelectGraphicRendition> {
        args.reverse();
        let mut graphics: Vec<SelectGraphicRendition> = Vec::new();
        while let Some(arg) = args.pop() {
            let sgr_opt = match arg {
                0 => Some(SelectGraphicRendition::Normal),
                1 => Some(SelectGraphicRendition::Bold),
                2 => Some(SelectGraphicRendition::Faint),
                3 => Some(SelectGraphicRendition::Italic),
                4 => Some(SelectGraphicRendition::Underline),
                5 => Some(SelectGraphicRendition::SlowBlink),
                6 => Some(SelectGraphicRendition::RapidBlink),
                7 => Some(SelectGraphicRendition::Invert),
                8 => Some(SelectGraphicRendition::Conceal),
                9 => Some(SelectGraphicRendition::CrossedOut),
                10..=20 => Some(SelectGraphicRendition::Font(arg - 10)),
                21 => Some(SelectGraphicRendition::DoublyUnderlined),
                22 => Some(SelectGraphicRendition::NormalIntensity),
                23 => Some(SelectGraphicRendition::NotItalic),
                24 => Some(SelectGraphicRendition::NotUnderlined),
                25 => Some(SelectGraphicRendition::NotBlinking),
                26 => Some(SelectGraphicRendition::ProportionalSpacing),
                27 => Some(SelectGraphicRendition::NotReveresed),
                28 => Some(SelectGraphicRendition::Reveal),
                29 => Some(SelectGraphicRendition::NotCrossedOut),
                30..=37 => Some(SelectGraphicRendition::ForgroundColor(Color::from_index(
                    arg - 30,
                ))),
                38 => Some(SelectGraphicRendition::ForgroundColor(Color::from_args(
                    args,
                ))),
                39 => Some(SelectGraphicRendition::ForgroundColor(None)),
                40..=47 => Some(SelectGraphicRendition::BackgroundColor(Color::from_index(
                    arg - 40,
                ))),
                48 => Some(SelectGraphicRendition::BackgroundColor(Color::from_args(
                    args,
                ))),
                49 => Some(SelectGraphicRendition::BackgroundColor(None)),
                50 => Some(SelectGraphicRendition::DisableProportionalSpacing),
                51 => Some(SelectGraphicRendition::Framed),
                52 => Some(SelectGraphicRendition::Encircled),
                53 => Some(SelectGraphicRendition::Overlined),
                54 => Some(SelectGraphicRendition::NeitherFramedNorEncircled),
                55 => Some(SelectGraphicRendition::NotOverlined),
                58 => Some(SelectGraphicRendition::SetUnderlineColor(Color::from_args(
                    args,
                ))),
                59 => Some(SelectGraphicRendition::SetUnderlineColor(None)),
                60 => Some(SelectGraphicRendition::IdeogramUnderline),
                61 => Some(SelectGraphicRendition::IdeogramDoubleUnderline),
                62 => Some(SelectGraphicRendition::IdeogramOverline),
                63 => Some(SelectGraphicRendition::IdeogramDoubleOverline),
                64 => Some(SelectGraphicRendition::IdeogramStressMarking),
                65 => Some(SelectGraphicRendition::NoIdeogram),
                73 => Some(SelectGraphicRendition::Superscript),
                74 => Some(SelectGraphicRendition::Subscript),
                75 => Some(SelectGraphicRendition::NethirSuperOrSubScript),
                90..=97 => match Color::from_index(arg - 90) {
                    Some(c) => Some(SelectGraphicRendition::ForgroundColor(Some(
                        Color::make_bright(c),
                    ))),
                    None => Some(SelectGraphicRendition::ForgroundColor(None)),
                },
                100..=107 => match Color::from_index(arg - 100) {
                    Some(c) => Some(SelectGraphicRendition::BackgroundColor(Some(
                        Color::make_bright(c),
                    ))),
                    None => Some(SelectGraphicRendition::BackgroundColor(None)),
                },
                _ => None,
            };
            if let Some(sgr) = sgr_opt {
                graphics.push(sgr)
            }
        }
        return graphics;
    }
}

/// This is the list of valid control sequences that are valid as part of the FeEscape Sequence
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ControlSequence {
    CursorUp(u8),
    CursorDown(u8),
    CursorForward(u8),
    CursorBack(u8),
    CursorNextLine(u8),
    CursorPreviousLine(u8),
    CursorHorizontalAbsolute(u8),
    CursorPosition(u8, u8),
    EraseInDisplay(u8),
    EraseInLine(u8),
    ScrollUp(u8),
    ScrollDown(u8),
    HorizonalVerticalPosition(u8, u8),
    SelectGraphicalRendition(Vec<SelectGraphicRendition>),
    AUXPortOn,
    AUXPortOff,
    DeviceStatusReport,
    SaveCursorPosistion,
    RestoreCursorPosistion,
    VT220Cursor,
    HideCursor,
    EnableReportingFocus,
    DisableReportingFocus,
    EnableAltScreenBuf,
    DisableAltScreenBuf,
    BracketPasteMode,
    NoBracketPasteMode,
}

impl ControlSequence {
    /// This will conver the text representation of the arguments given in the ebnf described below and return them as a series of int arguments.
    ///
    /// ```ebnf
    /// digit = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0"
    /// int = { digit }
    /// args = [int],{[";"],[int]}
    /// ```
    ///
    pub fn get_args(text: &mut String) -> Vec<u8> {
        let mut args: Vec<u8> = Vec::new();
        if let Some(c) = text.pop() {
            if c != ';' {
                text.push(c);
            }
        }
        for arg_str in text.split(';') {
            args.push(arg_str.parse::<u8>().unwrap_or(0))
        }
        return args;
    }
    /// This will parse the text in the form described by the ebnf below into its internal ControlSequence representation if it is possible.
    ///
    /// ```ebf
    /// digit = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0"
    /// int = { digit }
    /// args = [int],{[";"],[int]}
    /// command_identifier = 'm'| 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'J' | 'K' | 'S' | 'T' | 'f' | 'i' | 'n' | 's' | 'u'
    /// control_sequence = args, command_identifier
    /// ```
    pub fn from(chars: &mut Chars) -> Option<ControlSequence> {
        let mut text_buffer = String::new();
        while let Some(c) = chars.next() {
            match c {
                'm' => {
                    let mut args = ControlSequence::get_args(&mut text_buffer);
                    return Some(ControlSequence::SelectGraphicalRendition(
                        SelectGraphicRendition::from(&mut args),
                    ));
                }
                'A' => {
                    let args = ControlSequence::get_args(&mut text_buffer);
                    return Some(ControlSequence::CursorUp(
                        args.get(0).unwrap_or(&(1 as u8)).clone(),
                    ));
                }
                'B' => {
                    let args = ControlSequence::get_args(&mut text_buffer);
                    return Some(ControlSequence::CursorDown(
                        args.get(0).unwrap_or(&(1 as u8)).clone(),
                    ));
                }
                'C' => {
                    let args = ControlSequence::get_args(&mut text_buffer);
                    return Some(ControlSequence::CursorForward(
                        args.get(0).unwrap_or(&(1 as u8)).clone(),
                    ));
                }
                'D' => {
                    let args = ControlSequence::get_args(&mut text_buffer);
                    return Some(ControlSequence::CursorBack(
                        args.get(0).unwrap_or(&(1 as u8)).clone(),
                    ));
                }
                'E' => {
                    let args = ControlSequence::get_args(&mut text_buffer);
                    return Some(ControlSequence::CursorNextLine(
                        args.get(0).unwrap_or(&(1 as u8)).clone(),
                    ));
                }
                'F' => {
                    let args = ControlSequence::get_args(&mut text_buffer);
                    return Some(ControlSequence::CursorPreviousLine(
                        args.get(0).unwrap_or(&(1 as u8)).clone(),
                    ));
                }
                'G' => {
                    let args = ControlSequence::get_args(&mut text_buffer);
                    return Some(ControlSequence::CursorHorizontalAbsolute(
                        args.get(0).unwrap_or(&(1 as u8)).clone(),
                    ));
                }
                'H' => {
                    let args = ControlSequence::get_args(&mut text_buffer);
                    return Some(ControlSequence::CursorPosition(
                        args.get(0).unwrap_or(&(1 as u8)).clone(),
                        args.get(1).unwrap_or(&(1 as u8)).clone(),
                    ));
                }
                'J' => {
                    let args = ControlSequence::get_args(&mut text_buffer);
                    return Some(ControlSequence::EraseInDisplay(
                        args.get(0).unwrap_or(&(1 as u8)).clone(),
                    ));
                }
                'K' => {
                    let args = ControlSequence::get_args(&mut text_buffer);
                    return Some(ControlSequence::EraseInLine(
                        args.get(0).unwrap_or(&(1 as u8)).clone(),
                    ));
                }
                'S' => {
                    let args = ControlSequence::get_args(&mut text_buffer);
                    return Some(ControlSequence::ScrollUp(
                        args.get(0).unwrap_or(&(1 as u8)).clone(),
                    ));
                }
                'T' => {
                    let args = ControlSequence::get_args(&mut text_buffer);
                    return Some(ControlSequence::ScrollDown(
                        args.get(0).unwrap_or(&(1 as u8)).clone(),
                    ));
                }
                'f' => {
                    let args = ControlSequence::get_args(&mut text_buffer);
                    return Some(ControlSequence::HorizonalVerticalPosition(
                        args.get(0).unwrap_or(&(1 as u8)).clone(),
                        args.get(1).unwrap_or(&(1 as u8)).clone(),
                    ));
                }
                'i' => {
                    let args = ControlSequence::get_args(&mut text_buffer);
                    return match args.get(0) {
                        Some(arg) => match arg {
                            5 => Some(ControlSequence::AUXPortOn),
                            4 => Some(ControlSequence::AUXPortOff),
                            _ => None,
                        },
                        None => None,
                    };
                }
                'h' => {
                    return match text_buffer.as_str() {
                        "?25" => Some(ControlSequence::VT220Cursor),
                        "?1004" => Some(ControlSequence::EnableReportingFocus),
                        "?1049" => Some(ControlSequence::EnableAltScreenBuf),
                        "?2004" => Some(ControlSequence::BracketPasteMode),
                        _ => None,
                    }
                }
                'l' => {
                    return match text_buffer.as_str() {
                        "?25" => Some(ControlSequence::HideCursor),
                        "?1004" => Some(ControlSequence::DisableReportingFocus),
                        "?1049" => Some(ControlSequence::DisableAltScreenBuf),
                        "?2004" => Some(ControlSequence::NoBracketPasteMode),
                        _ => None,
                    }
                }
                'n' => return Some(ControlSequence::DeviceStatusReport),
                's' => return Some(ControlSequence::SaveCursorPosistion),
                'u' => return Some(ControlSequence::RestoreCursorPosistion),
                _ => {
                    text_buffer.push(c);
                }
            }
        }
        return None;
    }
}

/// This is the internal reprenstation of ANSI FeEscapeSequences
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FeEscapeSequence {
    SingleShiftTwo,
    SingleShiftThree,
    DeviceControlString,
    ControlSequence(ControlSequence),
    OperatingSystemCommand,
    StringTerminator,
    StartOfString,
    PrivacyMessage,
    ApplicationProgramCommand,
}

impl FeEscapeSequence {
    /// Takes in a string following the below ebnf and returns out our internal FeEscapeSequence
    ///
    /// ```ebf
    /// digit = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0"
    /// int = { digit }
    /// args = [int],{[";"],[int]}
    /// command_identifier = 'm'| 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'J' | 'K' | 'S' | 'T' | 'f' | 'i' | 'n' | 's' | 'u'
    /// control_sequence = args, command_identifier
    /// fe_escape_sequence = fe_identifier "N" | "O" | "P" | "[",control_sequence | "X" | "^" | "_" | "\\"
    /// ```
    pub fn from(chars: &mut Chars) -> Option<FeEscapeSequence> {
        return match chars.next() {
            Some(c) => match c {
                'N' | 'n' => Some(FeEscapeSequence::SingleShiftTwo),
                'O' | 'o' => Some(FeEscapeSequence::SingleShiftThree),
                'P' | 'p' => Some(FeEscapeSequence::DeviceControlString),
                '[' => match ControlSequence::from(chars) {
                    Some(controlsequence) => {
                        Some(FeEscapeSequence::ControlSequence(controlsequence))
                    }
                    None => None,
                },
                ']' => Some(FeEscapeSequence::OperatingSystemCommand),
                'X' | 'x' => Some(FeEscapeSequence::StartOfString),
                '^' => Some(FeEscapeSequence::PrivacyMessage),
                '_' => Some(FeEscapeSequence::ApplicationProgramCommand),
                '\\' => Some(FeEscapeSequence::StringTerminator),
                _ => None,
            },
            None => None,
        };
    }

    /// This is varient of from that returns a copy of the string given with the escape sequence removed if there was one at the start as well as the escape sequence found.
    pub fn extract_from(string: &str) -> (String, Option<FeEscapeSequence>) {
        let mut chars = string.chars();
        let esc_seq = FeEscapeSequence::from(&mut chars);
        return (chars.as_str().to_string(), esc_seq);
    }
}

pub type TextElement = crate::common::TextElement<FeEscapeSequence>;

pub type Text = crate::common::Text<FeEscapeSequence>;

impl Text {
    // returns a new fully allocated ansi text struct
    pub fn new() -> Text {
        Text { text: Vec::new() }
    }

    // This will is a wrapper function allowing you to instant a ansi::Text and read in a string into it
    pub fn from(text: String) -> Text {
        let mut ansi_text = Text::new();
        ansi_text.read(text);
        return ansi_text;
    }

    /// This allows us to read in a complient ANSI String into our internal representation it does this by parsing out the ansi escape sequences that follow the ebnf given below
    ///
    /// ```ebnf
    /// digit = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0"
    /// int = { digit }
    /// args = [int],{[";"],[int]}
    /// command_identifier = 'm'| 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'J' | 'K' | 'S' | 'T' | 'f' | 'i' | 'n' | 's' | 'u'
    /// control_sequence = args, command_identifier
    /// fe_escape_sequence = fe_identifier "N" | "O" | "P" | "[",control_sequence | "X" | "^" | "_" | "\\"
    /// esc = "\e"
    /// letter = "A" | "B" | "C" | "D" | "E" | "F" | "G"
    ///        | "H" | "I" | "J" | "K" | "L" | "M" | "N"
    ///        | "O" | "P" | "Q" | "R" | "S" | "T" | "U"
    ///        | "V" | "W" | "X" | "Y" | "Z" | "a" | "b"
    ///        | "c" | "d" | "e" | "f" | "g" | "h" | "i"
    ///        | "j" | "k" | "l" | "m" | "n" | "o" | "p"
    ///        | "q" | "r" | "s" | "t" | "u" | "v" | "w"
    ///        | "x" | "y" | "z" ;
    ///
    /// digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
    ///
    /// symbol = "[" | "]" | "{" | "}" | "(" | ")" | "<" | ">"
    ///         | "'" | '"' | "=" | "|" | "." | "," | ";" | "-"
    ///         | "+" | "*" | "?" | "\n" | "\t" | "\r" | "\f" | "\b" ;
    ///
    /// character = letter | digit | symbol | "_" | " " ;
    /// string = {character}
    /// esc_sequence = esc,fe_escape_sequence
    /// text = {string | esc_sequence}
    /// ```
    pub fn read(&mut self, text: String) {
        let mut sequences = text.split(ESC);
        match sequences.next() {
            Some(text) => {
                let s = text.to_string();
                if !s.is_empty() {
                    self.text.push(TextElement::Text(s));
                }
            }
            None => {}
        };
        for sequence in sequences {
            if sequence.len() <= 0 {
                continue;
            }
            let (text, opt_fe_sequence) = FeEscapeSequence::extract_from(sequence);
            if let Some(fe_sequence) = opt_fe_sequence {
                self.text.push(TextElement::Marker(fe_sequence));
            }
            if !text.is_empty() {
                self.text.push(TextElement::Text(text))
            }
        }
    }

    /// This clears the buffer of that is held internally is the same as allocating a new struct however it allocation than the creating a new vector.
    pub fn flush(&mut self) {
        self.text.clear()
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, vec};

    use crate::{common, input_fmt::ansi::TextElement};

    use super::{Color, ControlSequence, FeEscapeSequence, SelectGraphicRendition};

    #[test]
    fn color_from_index() {
        let tests = [
            (0, Some(common::black())),
            (1, Some(common::red())),
            (2, Some(common::green())),
            (3, Some(common::yellow())),
            (4, Some(common::blue())),
            (5, Some(common::magenta())),
            (6, Some(common::cyan())),
            (7, Some(common::white())),
            (8, None),
        ];
        for test in tests {
            let (index, expected_result) = test;
            assert_eq!(Color::from_index(index), expected_result);
        }
    }

    #[test]
    fn color_from_args() {
        let base_24_bit = vec![2 as u8];
        for r in 0..=255 as u8 {
            for g in 0..=255 as u8 {
                for b in 0..=255 as u8 {
                    let mut args = base_24_bit.clone();
                    args.push(r);
                    args.push(g);
                    args.push(b);
                    args.reverse();
                    assert_eq!(
                        Color::from_args(&mut args),
                        Some(Color {
                            red: r,
                            green: g,
                            blue: b
                        })
                    )
                }
            }
        }
        let base_8_bit = vec![5 as u8];
        for c in 0..=255 as u8 {
            let mut args = base_8_bit.clone();
            args.push(c);
            args.reverse();
            assert_eq!(
                Color::from_args(&mut args),
                Some(Color {
                    red: (c >> 5) * 32,
                    green: ((c & 28) >> 2) * 32,
                    blue: (c & 3) * 32,
                })
            )
        }
    }

    #[test]
    fn sgr_from() {
        let results = HashMap::from([
            (0 as u8, vec![SelectGraphicRendition::Normal]),
            (1 as u8, vec![SelectGraphicRendition::Bold]),
            (2 as u8, vec![SelectGraphicRendition::Faint]),
            (3 as u8, vec![SelectGraphicRendition::Italic]),
            (4 as u8, vec![SelectGraphicRendition::Underline]),
            (5 as u8, vec![SelectGraphicRendition::SlowBlink]),
            (6 as u8, vec![SelectGraphicRendition::RapidBlink]),
            (7 as u8, vec![SelectGraphicRendition::Invert]),
            (8 as u8, vec![SelectGraphicRendition::Conceal]),
            (9 as u8, vec![SelectGraphicRendition::CrossedOut]),
            (10 as u8, vec![SelectGraphicRendition::Font(0)]),
            (11 as u8, vec![SelectGraphicRendition::Font(1)]),
            (12 as u8, vec![SelectGraphicRendition::Font(2)]),
            (13 as u8, vec![SelectGraphicRendition::Font(3)]),
            (14 as u8, vec![SelectGraphicRendition::Font(4)]),
            (15 as u8, vec![SelectGraphicRendition::Font(5)]),
            (16 as u8, vec![SelectGraphicRendition::Font(6)]),
            (17 as u8, vec![SelectGraphicRendition::Font(7)]),
            (18 as u8, vec![SelectGraphicRendition::Font(8)]),
            (19 as u8, vec![SelectGraphicRendition::Font(9)]),
            (20 as u8, vec![SelectGraphicRendition::Font(10)]),
            (21 as u8, vec![SelectGraphicRendition::DoublyUnderlined]),
            (22 as u8, vec![SelectGraphicRendition::NormalIntensity]),
            (23 as u8, vec![SelectGraphicRendition::NotItalic]),
            (24 as u8, vec![SelectGraphicRendition::NotUnderlined]),
            (25 as u8, vec![SelectGraphicRendition::NotBlinking]),
            (26 as u8, vec![SelectGraphicRendition::ProportionalSpacing]),
            (27 as u8, vec![SelectGraphicRendition::NotReveresed]),
            (28 as u8, vec![SelectGraphicRendition::Reveal]),
            (29 as u8, vec![SelectGraphicRendition::NotCrossedOut]),
            (
                30 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Color::from_index(0))],
            ),
            (
                31 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Color::from_index(1))],
            ),
            (
                32 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Color::from_index(2))],
            ),
            (
                33 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Color::from_index(3))],
            ),
            (
                34 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Color::from_index(4))],
            ),
            (
                35 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Color::from_index(5))],
            ),
            (
                36 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Color::from_index(6))],
            ),
            (
                37 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Color::from_index(7))],
            ),
            (
                38 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Color::from_args(
                    &mut Vec::new(),
                ))],
            ),
            (39 as u8, vec![SelectGraphicRendition::ForgroundColor(None)]),
            (
                40 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Color::from_index(
                    0,
                ))],
            ),
            (
                41 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Color::from_index(
                    1,
                ))],
            ),
            (
                42 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Color::from_index(
                    2,
                ))],
            ),
            (
                43 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Color::from_index(
                    3,
                ))],
            ),
            (
                44 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Color::from_index(
                    4,
                ))],
            ),
            (
                45 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Color::from_index(
                    5,
                ))],
            ),
            (
                46 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Color::from_index(
                    6,
                ))],
            ),
            (
                47 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Color::from_index(
                    7,
                ))],
            ),
            (
                48 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Color::from_args(
                    &mut Vec::new(),
                ))],
            ),
            (
                49 as u8,
                vec![SelectGraphicRendition::BackgroundColor(None)],
            ),
            (
                50 as u8,
                vec![SelectGraphicRendition::DisableProportionalSpacing],
            ),
            (51 as u8, vec![SelectGraphicRendition::Framed]),
            (52 as u8, vec![SelectGraphicRendition::Encircled]),
            (53 as u8, vec![SelectGraphicRendition::Overlined]),
            (
                54 as u8,
                vec![SelectGraphicRendition::NeitherFramedNorEncircled],
            ),
            (55 as u8, vec![SelectGraphicRendition::NotOverlined]),
            (
                58 as u8,
                vec![SelectGraphicRendition::SetUnderlineColor(Color::from_args(
                    &mut Vec::new(),
                ))],
            ),
            (
                59 as u8,
                vec![SelectGraphicRendition::SetUnderlineColor(None)],
            ),
            (60 as u8, vec![SelectGraphicRendition::IdeogramUnderline]),
            (
                61 as u8,
                vec![SelectGraphicRendition::IdeogramDoubleUnderline],
            ),
            (62 as u8, vec![SelectGraphicRendition::IdeogramOverline]),
            (
                63 as u8,
                vec![SelectGraphicRendition::IdeogramDoubleOverline],
            ),
            (
                64 as u8,
                vec![SelectGraphicRendition::IdeogramStressMarking],
            ),
            (65 as u8, vec![SelectGraphicRendition::NoIdeogram]),
            (73 as u8, vec![SelectGraphicRendition::Superscript]),
            (74 as u8, vec![SelectGraphicRendition::Subscript]),
            (
                75 as u8,
                vec![SelectGraphicRendition::NethirSuperOrSubScript],
            ),
            (
                90 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Some(
                    Color::make_bright(common::black()),
                ))],
            ),
            (
                91 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Some(
                    Color::make_bright(common::red()),
                ))],
            ),
            (
                92 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Some(
                    Color::make_bright(common::green()),
                ))],
            ),
            (
                93 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Some(
                    Color::make_bright(common::yellow()),
                ))],
            ),
            (
                94 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Some(
                    Color::make_bright(common::blue()),
                ))],
            ),
            (
                95 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Some(
                    Color::make_bright(common::magenta()),
                ))],
            ),
            (
                96 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Some(
                    Color::make_bright(common::cyan()),
                ))],
            ),
            (
                97 as u8,
                vec![SelectGraphicRendition::ForgroundColor(Some(
                    Color::make_bright(common::white()),
                ))],
            ),
            (
                100 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Some(
                    Color::make_bright(common::black()),
                ))],
            ),
            (
                101 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Some(
                    Color::make_bright(common::red()),
                ))],
            ),
            (
                102 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Some(
                    Color::make_bright(common::green()),
                ))],
            ),
            (
                103 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Some(
                    Color::make_bright(common::yellow()),
                ))],
            ),
            (
                104 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Some(
                    Color::make_bright(common::blue()),
                ))],
            ),
            (
                105 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Some(
                    Color::make_bright(common::magenta()),
                ))],
            ),
            (
                106 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Some(
                    Color::make_bright(common::cyan()),
                ))],
            ),
            (
                107 as u8,
                vec![SelectGraphicRendition::BackgroundColor(Some(
                    Color::make_bright(common::white()),
                ))],
            ),
        ]);
        for i in 0..=255 as u8 {
            let mut args: Vec<u8> = Vec::new();
            args.push(i);
            let result = SelectGraphicRendition::from(&mut args);
            match results.get(&i) {
                Some(expected_result) => {
                    assert_eq!(&result, expected_result);
                }
                None => {
                    assert!(result.len() == 0)
                }
            }
        }
    }
    #[test]
    fn csi_get_args() {
        assert_eq!(ControlSequence::get_args(&mut "3".to_string()), vec![3]);
        assert_eq!(ControlSequence::get_args(&mut ";".to_string()), vec![0]);
        assert_eq!(
            ControlSequence::get_args(&mut "3;2".to_string()),
            vec![3, 2]
        );
        assert_eq!(
            ControlSequence::get_args(&mut "3;2;1".to_string()),
            vec![3, 2, 1]
        );
        assert_eq!(
            ControlSequence::get_args(&mut "255;;255".to_string()),
            vec![255, 0, 255]
        );
    }

    #[test]
    fn csi_from() {
        for n in [0 as u8, 100 as u8, 255 as u8] {
            for test_case in [
                (
                    format!("{}A", n).chars(),
                    Some(ControlSequence::CursorUp(n)),
                ),
                (
                    format!("{}B", n).chars(),
                    Some(ControlSequence::CursorDown(n)),
                ),
                (
                    format!("{}C", n).chars(),
                    Some(ControlSequence::CursorForward(n)),
                ),
                (
                    format!("{}D", n).chars(),
                    Some(ControlSequence::CursorBack(n)),
                ),
                (
                    format!("{}E", n).chars(),
                    Some(ControlSequence::CursorNextLine(n)),
                ),
                (
                    format!("{}F", n).chars(),
                    Some(ControlSequence::CursorPreviousLine(n)),
                ),
                (
                    format!("{}G", n).chars(),
                    Some(ControlSequence::CursorHorizontalAbsolute(n)),
                ),
                (
                    format!("{}H", n).chars(),
                    Some(ControlSequence::CursorPosition(n, 1)),
                ),
                (
                    format!("{}J", n).chars(),
                    Some(ControlSequence::EraseInDisplay(n)),
                ),
                (
                    format!("{}K", n).chars(),
                    Some(ControlSequence::EraseInLine(n)),
                ),
                (
                    format!("{}S", n).chars(),
                    Some(ControlSequence::ScrollUp(n)),
                ),
                (
                    format!("{}T", n).chars(),
                    Some(ControlSequence::ScrollDown(n)),
                ),
                (
                    format!("{}f", n).chars(),
                    Some(ControlSequence::HorizonalVerticalPosition(n, 1)),
                ),
                (format!("5i").chars(), Some(ControlSequence::AUXPortOn)),
                (format!("4i").chars(), Some(ControlSequence::AUXPortOff)),
                (
                    format!("6n").chars(),
                    Some(ControlSequence::DeviceStatusReport),
                ),
                (
                    format!("s").chars(),
                    Some(ControlSequence::SaveCursorPosistion),
                ),
                (
                    format!("u").chars(),
                    Some(ControlSequence::RestoreCursorPosistion),
                ),
                (format!("?25h").chars(), Some(ControlSequence::VT220Cursor)),
                (format!("?25l").chars(), Some(ControlSequence::HideCursor)),
                (
                    format!("?1004h").chars(),
                    Some(ControlSequence::EnableReportingFocus),
                ),
                (
                    format!("?1004l").chars(),
                    Some(ControlSequence::DisableReportingFocus),
                ),
                (
                    format!("?1049h").chars(),
                    Some(ControlSequence::EnableAltScreenBuf),
                ),
                (
                    format!("?1049l").chars(),
                    Some(ControlSequence::DisableAltScreenBuf),
                ),
                (
                    format!("?2004h").chars(),
                    Some(ControlSequence::BracketPasteMode),
                ),
                (
                    format!("?2004l").chars(),
                    Some(ControlSequence::NoBracketPasteMode),
                ),
            ] {
                let (mut test, result) = test_case;
                assert_eq!(ControlSequence::from(&mut test), result);
            }
        }
    }

    #[test]
    fn fe_from() {
        for test_case in [
            ("Ntest".chars(), Some(FeEscapeSequence::SingleShiftTwo)),
            ("Otest".chars(), Some(FeEscapeSequence::SingleShiftThree)),
            ("Ptest".chars(), Some(FeEscapeSequence::DeviceControlString)),
            ("\\test".chars(), Some(FeEscapeSequence::StringTerminator)),
            (
                "]test".chars(),
                Some(FeEscapeSequence::OperatingSystemCommand),
            ),
            ("Xtest".chars(), Some(FeEscapeSequence::StartOfString)),
            ("^test".chars(), Some(FeEscapeSequence::PrivacyMessage)),
            (
                "_test".chars(),
                Some(FeEscapeSequence::ApplicationProgramCommand),
            ),
            (
                "[5itest".chars(),
                Some(FeEscapeSequence::ControlSequence(
                    ControlSequence::AUXPortOn,
                )),
            ),
            (
                "[31mtest".chars(),
                Some(FeEscapeSequence::ControlSequence(
                    ControlSequence::SelectGraphicalRendition(vec![
                        SelectGraphicRendition::ForgroundColor(Color::from_index(1)),
                    ]),
                )),
            ),
        ] {
            let (mut test, result) = test_case;
            assert_eq!(FeEscapeSequence::from(&mut test), result)
        }
    }

    #[test]
    fn fe_extract_from() {
        for test_case in [
            ("Ntest", Some(FeEscapeSequence::SingleShiftTwo)),
            ("Otest", Some(FeEscapeSequence::SingleShiftThree)),
            ("Ptest", Some(FeEscapeSequence::DeviceControlString)),
            ("\\test", Some(FeEscapeSequence::StringTerminator)),
            ("]test", Some(FeEscapeSequence::OperatingSystemCommand)),
            ("Xtest", Some(FeEscapeSequence::StartOfString)),
            ("^test", Some(FeEscapeSequence::PrivacyMessage)),
            ("_test", Some(FeEscapeSequence::ApplicationProgramCommand)),
            (
                "[5itest",
                Some(FeEscapeSequence::ControlSequence(
                    ControlSequence::AUXPortOn,
                )),
            ),
            (
                "[31mtest",
                Some(FeEscapeSequence::ControlSequence(
                    ControlSequence::SelectGraphicalRendition(vec![
                        SelectGraphicRendition::ForgroundColor(Color::from_index(1)),
                    ]),
                )),
            ),
        ] {
            let (mut test, expect_result) = test_case;
            let (result_text, result) = FeEscapeSequence::extract_from(&mut test);
            assert_eq!(result_text, "test".to_string());
            assert_eq!(result, expect_result);
        }
    }

    #[test]
    fn test_from() {
        assert_eq!(super::Text::from("\u{001B}[m\u{001B}[32mThis is a \u{001B}[1mtest\u{001B}[22m and it should work\u{001B}[0m".to_string()),super::Text{
            text:vec![
                TextElement::Marker(FeEscapeSequence::ControlSequence(ControlSequence::SelectGraphicalRendition(vec![SelectGraphicRendition::Normal]))),
                TextElement::Marker(FeEscapeSequence::ControlSequence(ControlSequence::SelectGraphicalRendition(vec![SelectGraphicRendition::ForgroundColor(Color::from_index(2))]))),
                TextElement::Text("This is a ".to_string()),
                TextElement::Marker(FeEscapeSequence::ControlSequence(ControlSequence::SelectGraphicalRendition(vec![SelectGraphicRendition::Bold]))),
                TextElement::Text("test".to_string()),
                TextElement::Marker(FeEscapeSequence::ControlSequence(ControlSequence::SelectGraphicalRendition(vec![SelectGraphicRendition::NormalIntensity]))),
                TextElement::Text(" and it should work".to_string()),
                TextElement::Marker(FeEscapeSequence::ControlSequence(ControlSequence::SelectGraphicalRendition(vec![SelectGraphicRendition::Normal]))),
                ]
            }
        );
    }
}
