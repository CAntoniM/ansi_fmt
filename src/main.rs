use clap::{Parser,ValueEnum};
use std::{path::PathBuf, fs::File, io::{self, BufRead}};
use phf::phf_map;

/// This is the map that is responceable for relating the character given as
/// the FE escape seqeunce to the function that is designed to process them.
/// 
/// These functions should remove the escape seqeunce from the String given
/// and if they can come out with some formatting change this can be adding or
/// removing the specified format
static FE_HANDLERS: phf::Map<char,fn(String,usize) -> (String,Option<(Fromatting,bool)>)> = phf_map! {
    '[' => |line: String, start: usize| -> (String,Option<(Fromatting,bool)>) {
        (line.clone(),None)
    },
    'N' => |line: String, start: usize| -> (String,Option<(Fromatting,bool)>) {
        let mut can_copy = false;
        let mut output_string = String::new();
        for (index, character) in line.chars().enumerate() {
            if index < start {
                output_string.push(character);
                continue;
            }
            if character == 'N'{
                can_copy = true;
            }
            if can_copy {
                output_string.push(character);
            }
        }
        return (output_string,None)
    },
    'O' => |line: String, start: usize| -> (String,Option<(Fromatting,bool)>) {
        let mut can_copy = false;
        let mut output_string = String::new();
        for (index, character) in line.chars().enumerate() {
            if index < start {
                output_string.push(character);
                continue;
            }
            if character == 'O'{
                can_copy = true;
            }
            if can_copy {
                output_string.push(character);
            }
        }
        return (output_string,None)
    },
    'P' => |line: String, start: usize| -> (String,Option<(Fromatting,bool)>) {
        let mut can_copy = false;
        let mut expected_next_char: Option<char> = None;
        let mut output_string = String::new();
        for (index, character) in line.chars().enumerate() {
            if index < start {
                output_string.push(character);
                continue;
            }
            if character == '\u{001b}' {
                expected_next_char = Some('\\')
            }
            if Some('\\') == expected_next_char {
                can_copy = true
            }
            if can_copy {
                output_string.push(character);
            }
        }
        return (output_string,None)
    },
};

static ESC: char = 0x1B as char;

#[derive(Clone, Copy,PartialEq, Eq, PartialOrd, Ord,ValueEnum,Debug)]
enum OutputFormat {
    Text,
    Html,
}

#[derive(Parser,Debug)]
struct Cli {
    //This specifies the format that will be used to format the output.
    #[arg(long,short,value_enum,default_value_t=OutputFormat::Text)]
    format: OutputFormat,
    /// This specifes the output location of the programe if none is given then
    /// we will write to Standard Out.
    #[arg(long,short)]
    output: Option<String>,
    /// This specifes the files that we want to read in from to remove ANSI 
    /// formatting and replace it with something else at the end of files read
    /// here we will read from standard in.
    #[arg(value_name="FILE")]
    paths: Vec<PathBuf>,

}

#[derive(Debug)]
struct TextPosistion {
    line: usize,
    char: usize,
}

impl TextPosistion {
    pub fn new() -> TextPosistion {
        TextPosistion {
            line: 0,
            char: 0,
        }
    }
}

enum Fromatting {
    Bold,
}


struct FormatBlock {
    start_posistion: TextPosistion,
    end_posistion: TextPosistion,
    style: Option<Fromatting>
}

impl FormatBlock {
    pub fn new() -> FormatBlock {
        FormatBlock { 
            start_posistion: TextPosistion::new(),
            end_posistion: TextPosistion::new(),
            style: None
        }
    }

}

struct FormatedTextFile {
    text: Vec<String>,
    style_blocks: Vec<FormatBlock>
}

impl FormatedTextFile {
    pub fn new() -> FormatedTextFile {
        FormatedTextFile { 
            text: Vec::new(),
            style_blocks: Vec::new()
        }
    }

    pub fn get_formatting_at(&mut self, index: usize) -> Vec<FormatBlock> {
        return Vec::new()
    }

    pub fn read_line(&mut self,mut line: String) {
        for escape_sequence in [r"\e",r"\033",r"\u001b",r"\x1B"] {
            line = line.replace(escape_sequence,ESC.to_string().as_str());
        }

        let mut index : usize
        while Some(index) = line.find(ESC){
            let next_char = line.as_bytes()[index+1] as char;
            match FE_HANDLERS.get(&next_char) {
                Some(handler) => {
                    let option_format: Option<(Fromatting,bool)>;
                    (line,option_format) = handler(line.clone(),index);
                    match option_format {
                        Some((fmt,is_removing)) => {
                            if ! is_removing {
                                let fmts = self.get_formatting_at(index);

                            }
                        },                        
                        None => {
                            continue;
                        }
                    }
                }
                None => {
                    continue;
                }
            }           
        }
        self.text.push(line);
    }

}

fn main() {
    let cli = Cli::parse();
    for path in cli.paths.iter() {
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);
        let mut fmt_txt = FormatedTextFile::new();
        
        for line in reader.lines() {
            fmt_txt.read_line(line.unwrap());
        }
    }
}
