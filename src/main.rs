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

enum ControlSequences {
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
}

impl ControlSequences {
    pub fn from(chars: &mut Chars) -> ControlSequences {
        return ControlSequences::DeviceStatusReport;
    }
}

enum FeEscapeSequences {
    SingleShiftTwo,
    SingleShiftThree,
    DeviceControlString,
    ControlSequence(ControlSequences),
    StartOfString,
    PrivacyMessage,
    ApplicationProgramCommand,
}

impl FeEscapeSequences {
    pub fn from(chars: &mut Chars) -> Option<FeEscapeSequences> {
        return match chars.next() {
            Some(c) => match c {
                'N' | 'n' => Some(FeEscapeSequences::SingleShiftTwo),
                'O' | 'o' => Some(FeEscapeSequences::SingleShiftThree),
                'P' | 'p' => Some(FeEscapeSequences::DeviceControlString),
                '[' => Some(FeEscapeSequences::ControlSequence(ControlSequences::from(
                    chars,
                ))),
                'X' | 'x' => Some(FeEscapeSequences::StartOfString),
                '^' => Some(FeEscapeSequences::PrivacyMessage),
                '_' => Some(FeEscapeSequences::ApplicationProgramCommand),
                _ => None,
            },
            None => None,
        };
    }

    pub fn extract_from(string: &str) -> (String, Option<FeEscapeSequences>) {
        let mut chars = string.chars();
        if chars.next().unwrap() == ESC {
            let esc_seq = FeEscapeSequences::from(&mut chars);
            return (chars.as_str().to_string(), esc_seq);
        }
        return (string.to_string(), None);
    }
}

enum ANSITextElement {
    Text(String),
    EscapeSequence(FeEscapeSequences),
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
            let (text, opt_fe_sequence) = FeEscapeSequences::extract_from(sequence);
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
