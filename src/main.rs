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
    PrimaryFont,
    AlternativeFont(u8),
    DoublyUnderlined,
    NormalIntensity,
    NotItalic,
    NotUnderlined,
    NotBlinking,
    ProportionalSpacing,
    NotReveresed,
    Reveal,
    NotCrossedOut,
    ForgroundColor(Color),
    DefaultForgroundColor,
    BackgroundColor(Color),
    DefaultBackgroundColor,
    DisableProportionalSpacing,
    Framed,
    Encircled,
    Overlined,
    NeitherFramedNorEncircled,
    NotOverlined,
    SetUnderlineColor(Color),
    DefaultUnderlineColor,
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

    pub fn from(args: Vec<u16>) -> Option<SelectGraphicRendition> {
        todo!("Add SGR Parsing");
        return None;
    }
}

enum ControlSequence {
    CursorUp(u16),
    CursorDown(u16),
    CursorForward(u16),
    CursorBack(u16),
    CursorNextLine(u16),
    CursorPreviousLine(u16),
    CursorHorizontalAbsolute(u16),
    CursorPosition(u16, u16),
    EraseInDisplay(u16),
    EraseInLine(u16),
    ScrollUp(u16),
    ScrollDown(u16),
    HorizonalVerticalPosition(u16, u16),
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
    pub fn get_args(text: &String) -> Vec<u16> {
        todo!("Implemnt args parsing");
        return Vec::new();
    }
    pub fn from(chars: &mut Chars) -> Option<ControlSequence> {
        let mut text_buffer = String::new();
        while let Some(c) = chars.next() {
            match c {
                'm' => {}
                'A' => {
                    let args = ControlSequence::get_args(&text_buffer);
                    return Some(ControlSequence::CursorUp(
                        args.get(0).unwrap_or(&(1 as u16)).clone(),
                    ));
                }
                'B' => {
                    let args = ControlSequence::get_args(&text_buffer);
                    return Some(ControlSequence::CursorDown(
                        args.get(0).unwrap_or(&(1 as u16)).clone(),
                    ));
                }
                'C' => {
                    let args = ControlSequence::get_args(&text_buffer);
                    return Some(ControlSequence::CursorForward(
                        args.get(0).unwrap_or(&(1 as u16)).clone(),
                    ));
                }
                'D' => {
                    let args = ControlSequence::get_args(&text_buffer);
                    return Some(ControlSequence::CursorBack(
                        args.get(0).unwrap_or(&(1 as u16)).clone(),
                    ));
                }
                'E' => {
                    let args = ControlSequence::get_args(&text_buffer);
                    return Some(ControlSequence::CursorNextLine(
                        args.get(0).unwrap_or(&(1 as u16)).clone(),
                    ));
                }
                'F' => {
                    let args = ControlSequence::get_args(&text_buffer);
                    return Some(ControlSequence::CursorPreviousLine(
                        args.get(0).unwrap_or(&(1 as u16)).clone(),
                    ));
                }
                'G' => {
                    let args = ControlSequence::get_args(&text_buffer);
                    return Some(ControlSequence::CursorHorizontalAbsolute(
                        args.get(0).unwrap_or(&(1 as u16)).clone(),
                    ));
                }
                'H' => {
                    let args = ControlSequence::get_args(&text_buffer);
                    return Some(ControlSequence::CursorPosition(
                        args.get(0).unwrap_or(&(1 as u16)).clone(),
                        args.get(1).unwrap_or(&(1 as u16)).clone(),
                    ));
                }
                'J' => {
                    let args = ControlSequence::get_args(&text_buffer);
                    return Some(ControlSequence::EraseInDisplay(
                        args.get(0).unwrap_or(&(1 as u16)).clone(),
                    ));
                }
                'K' => {
                    let args = ControlSequence::get_args(&text_buffer);
                    return Some(ControlSequence::EraseInLine(
                        args.get(0).unwrap_or(&(1 as u16)).clone(),
                    ));
                }
                'S' => {
                    let args = ControlSequence::get_args(&text_buffer);
                    return Some(ControlSequence::ScrollUp(
                        args.get(0).unwrap_or(&(1 as u16)).clone(),
                    ));
                }
                'T' => {
                    let args = ControlSequence::get_args(&text_buffer);
                    return Some(ControlSequence::ScrollDown(
                        args.get(0).unwrap_or(&(1 as u16)).clone(),
                    ));
                }
                'f' => {
                    let args = ControlSequence::get_args(&text_buffer);
                    return Some(ControlSequence::HorizonalVerticalPosition(
                        args.get(0).unwrap_or(&(1 as u16)).clone(),
                        args.get(1).unwrap_or(&(1 as u16)).clone(),
                    ));
                }
                'i' => {
                    let args = ControlSequence::get_args(&text_buffer);
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
        if chars.next().unwrap() == ESC {
            let esc_seq = FeEscapeSequence::from(&mut chars);
            return (chars.as_str().to_string(), esc_seq);
        }
        return (string.to_string(), None);
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
        let sequences = text.split(ESC);
        let mut text_buffer: ANSIText = ANSIText { text: Vec::new() };
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
