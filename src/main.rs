use clap::{Parser, ValueEnum};
use phf::phf_map;
use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
    str::Chars,
};

static ESC: char = 0x1B as char;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn from_index(index: u8) -> Color {
        todo!("Implement Index based Color generation");
        return Color {
            red: 0,
            green: 0,
            blue: 0,
        };
    }

    pub fn from_args(args: &Vec<u8>) -> Color {
        todo!("Implement Args based Color generation");
        return Color {
            red: 0,
            green: 0,
            blue: 0,
        };
    }
}

enum SelectGraphicRendition {
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
                30..=37 => Some(SelectGraphicRendition::ForgroundColor(Some(
                    Color::from_index(arg),
                ))),
                38 => Some(SelectGraphicRendition::ForgroundColor(Some(
                    Color::from_args(args),
                ))),
                39 => Some(SelectGraphicRendition::ForgroundColor(None)),
                40..=47 => Some(SelectGraphicRendition::BackgroundColor(Some(
                    Color::from_index(arg),
                ))),
                48 => Some(SelectGraphicRendition::ForgroundColor(Some(
                    Color::from_args(args),
                ))),
                49 => Some(SelectGraphicRendition::ForgroundColor(None)),
                50 => Some(SelectGraphicRendition::DisableProportionalSpacing),
                51 => Some(SelectGraphicRendition::Framed),
                52 => Some(SelectGraphicRendition::Encircled),
                53 => Some(SelectGraphicRendition::Overlined),
                54 => Some(SelectGraphicRendition::NeitherFramedNorEncircled),
                55 => Some(SelectGraphicRendition::NotOverlined),
                58 => Some(SelectGraphicRendition::SetUnderlineColor(Some(
                    Color::from_args(args),
                ))),
                59 => Some(SelectGraphicRendition::SetUnderlineColor(None)),
                60 => Some(SelectGraphicRendition::NeitherFramedNorEncircled),
                61 => Some(SelectGraphicRendition::NeitherFramedNorEncircled),
                62 => Some(SelectGraphicRendition::NeitherFramedNorEncircled),
                63 => Some(SelectGraphicRendition::NeitherFramedNorEncircled),
                64 => Some(SelectGraphicRendition::NeitherFramedNorEncircled),
                65 => Some(SelectGraphicRendition::NeitherFramedNorEncircled),
                74 => Some(SelectGraphicRendition::NeitherFramedNorEncircled),
                75 => Some(SelectGraphicRendition::NeitherFramedNorEncircled),
                90..=97 => Some(SelectGraphicRendition::ForgroundColor(Some(
                    Color::from_index(arg),
                ))),
                100..=107 => {
                    return Some(SelectGraphicRendition::BackgroundColor(Some(
                        Color::from_index(arg),
                    )))
                }
                _ => None,
            },
            None => Some(SelectGraphicRendition::Normal),
        };
    }
}

enum ControlSequence {
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
    pub fn from(chars: &mut Chars) -> Option<ControlSequence> {
        let mut text_buffer = String::new();
        while let Some(c) = chars.next() {
            match c {
                'm' => {}
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

enum FeEscapeSequence {
    SingleShiftTwo,
    SingleShiftThree,
    DeviceControlString,
    ControlSequence(ControlSequence),
    StartOfString,
    PrivacyMessage,
    ApplicationProgramCommand,
}

impl FeEscapeSequence {
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
                _ => None,
            },
            None => None,
        };
    }

    pub fn extract_from(string: &str) -> (String, Option<FeEscapeSequence>) {
        let mut chars = string.chars();
        let esc_seq = FeEscapeSequence::from(&mut chars);
        return (chars.as_str().to_string(), esc_seq);
    }
}

enum ANSITextElement {
    Text(String),
    EscapeSequence(FeEscapeSequence),
}

struct ANSIText {
    text: Vec<ANSITextElement>,
}

impl ANSIText {
    pub fn from(text: String) -> ANSIText {
        let mut sequences = text.split(ESC);
        let mut text_buffer: ANSIText = ANSIText { text: Vec::new() };
        text_buffer.text.push(ANSITextElement::Text(
            sequences.next().unwrap_or("").to_string(),
        ));
        for sequence in sequences {
            if sequence.len() <= 0 {
                continue;
            }
            let (text, opt_fe_sequence) = FeEscapeSequence::extract_from(sequence);
            if let Some(fe_sequence) = opt_fe_sequence {
                text_buffer
                    .text
                    .push(ANSITextElement::EscapeSequence(fe_sequence));
            }
            text_buffer.text.push(ANSITextElement::Text(text))
        }
        return text_buffer;
    }
}

/// This represents the styling of text that we support as part of our output
/// The idea is that all writers must be able to output these particular styles
/// with out worrying about the other support by ANSI
#[derive(PartialEq, Eq, PartialOrd)]
enum Fromatting {
    Bold,
    Faint,
    Italic,
    Underline,
    CrossedOut,
    ForgroundColor,
    BackgroundColor,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum OutputFormat {
    Text,
    Html,
}

#[derive(Parser, Debug)]
struct Cli {
    //This specifies the format that will be used to format the output.
    #[arg(long,short,value_enum,default_value_t=OutputFormat::Text)]
    format: OutputFormat,
    /// This specifes the output location of the programe if none is given then
    /// we will write to Standard Out.
    #[arg(long, short)]
    output: Option<String>,
    /// This specifes the files that we want to read in from to remove ANSI
    /// formatting and replace it with something else at the end of files read
    /// here we will read from standard in.
    #[arg(value_name = "FILE")]
    paths: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    for path in cli.paths.iter() {
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);
    }
}
