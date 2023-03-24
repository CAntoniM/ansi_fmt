use std::str::Chars;

/// This is an alias for the ASCII Escape character
static ESC: char = 0x1B as char;

/// A simple reprensentation of a 24 bit color code used is most apps
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {

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

    pub fn from_index(index: u8) -> Option<Color> {
        return match index {
            0 => Some(Color::black()),
            1 => Some(Color::red()),
            2 => Some(Color::green()),
            3 => Some(Color::yellow()),
            4 => Some(Color::blue()),
            5 => Some(Color::magenta()),
            6 => Some(Color::cyan()),
            7 => Some(Color::white()),
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
                    None => Some(Color::black()),
                },
                _ => None,
            },
            None => None,
        };
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

/// this is a representation of the SelectGraphicsRendidtion sequences that are commonly used in terminals.
/// we should aim to be as accurate to the most terminals as possible and cover as many values as possible
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
    pub fn from(args: &mut Vec<u8>) -> Option<SelectGraphicRendition> {
        args.reverse();
        return match args.pop() {
            Some(arg) => match arg {
                0 => Some(SelectGraphicRendition::Normal),
                1 => Some(SelectGraphicRendition::Bold),
                2 => Some(SelectGraphicRendition::Italic),
                3 => Some(SelectGraphicRendition::Faint),
                4 => Some(SelectGraphicRendition::Italic),
                5 => Some(SelectGraphicRendition::Underline),
                6 => Some(SelectGraphicRendition::SlowBlink),
                7 => Some(SelectGraphicRendition::RapidBlink),
                8 => Some(SelectGraphicRendition::Invert),
                9 => Some(SelectGraphicRendition::Conceal),
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
                48 => Some(SelectGraphicRendition::ForgroundColor(Color::from_args(
                    args,
                ))),
                49 => Some(SelectGraphicRendition::ForgroundColor(None)),
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
                74 => Some(SelectGraphicRendition::Superscript),
                75 => Some(SelectGraphicRendition::Subscript),
                75 => Some(SelectGraphicRendition::NethirSuperOrSubScript),
                90..=97 => match Color::from_index(arg - 90) {
                    Some(c) => Some(SelectGraphicRendition::ForgroundColor(Some(
                        Color::make_bright(c),
                    ))),
                    None => Some(SelectGraphicRendition::ForgroundColor(None)),
                },
                100..=107 => match Color::from_index(arg - 90) {
                    Some(c) => Some(SelectGraphicRendition::BackgroundColor(Some(
                        Color::make_bright(c),
                    ))),
                    None => Some(SelectGraphicRendition::BackgroundColor(None)),
                },
                _ => None,
            },
            None => Some(SelectGraphicRendition::Normal),
        };
    }
}

/// This is the list of valid control sequences that are valid as part of the FeEscape Sequence
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
    SelectGraphicalRendition(SelectGraphicRendition),
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
        for arg_str in text.split(';') {
            match arg_str.parse::<u8>() {
                Ok(arg) => args.push(arg),
                Err(_) => {}
            }
        }
        return Vec::new();
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
                    return match SelectGraphicRendition::from(&mut args) {
                        Some(sgr) => Some(ControlSequence::SelectGraphicalRendition(sgr)),
                        None => None
                    }
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
pub enum FeEscapeSequence {
    SingleShiftTwo,
    SingleShiftThree,
    DeviceControlString,
    ControlSequence(ControlSequence),
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

/// An element of a ANSI Complient string containing either a section of text or an escape sequence
pub enum TextElement {
    Text(String),
    EscapeSequence(FeEscapeSequence),
}

/// a ANSI Complient string
pub struct Text {
    text: Vec<TextElement>,
}

impl Text {
    // returns a new fully allocated ansi text struct
    pub fn new() -> Text {
        Text { text: Vec::new() }
    }

    // This will is a wrapper function allowing you to instant a ansi::Text and read in a string into it
    pub fn from(text: String) -> Text {
        let mut ansi_text = Text::new();
        ansi_text.read(text);
        return ansi_text
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
        self.text.push(TextElement::Text(
            sequences.next().unwrap_or("").to_string(),
        ));
        for sequence in sequences {
            if sequence.len() <= 0 {
                continue;
            }
            let (text, opt_fe_sequence) = FeEscapeSequence::extract_from(sequence);
            if let Some(fe_sequence) = opt_fe_sequence {
                self.text.push(TextElement::EscapeSequence(fe_sequence));
            }
            self.text.push(TextElement::Text(text))
        }
    }

    /// This clears the buffer of that is held internally is the same as allocating a new struct however it allocation than the creating a new vector.
    pub fn flush(&mut self) {
        self.text.clear()
    }
}