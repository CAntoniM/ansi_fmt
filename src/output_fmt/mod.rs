pub mod html;
pub mod text;

use clap::ValueEnum;

use crate::internal_format;

use self::html::HtmlWriter;
use self::text::TextWriter;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OutputFormat {
    Text,
    Html,
}

pub trait Formatter: ToString {}

pub fn from(fmt: OutputFormat, text: internal_format::Text) -> Option<Box<dyn Formatter>> {
    match fmt {
        OutputFormat::Text => Some(Box::new(TextWriter::from_text(text))),
        OutputFormat::Html => Some(Box::new(HtmlWriter::from_text(text))),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use crate::{
        common::test, internal_format, internal_format::TextElement, output_fmt::text::TextWriter,
    };

    use super::OutputFormat;

    #[test]
    pub fn from() {
        let test_cases = [
            (
                (
                    internal_format::Text {
                        text: vec![
                            TextElement::Marker(crate::common::Toggle::Set(
                                internal_format::Fromatting::Bold,
                            )),
                            TextElement::Text("test".to_string()),
                            TextElement::Marker(crate::common::Toggle::UnSet(
                                internal_format::Fromatting::Bold,
                            )),
                            TextElement::Text(" test_tmp.".to_string()),
                        ],
                    },
                    OutputFormat::Text,
                ),
                "test test_tmp.".to_string(),
            ),
            (
                (
                    internal_format::Text {
                        text: vec![
                            TextElement::Marker(crate::common::Toggle::Set(
                                internal_format::Fromatting::Bold,
                            )),
                            TextElement::Text("test".to_string()),
                            TextElement::Marker(crate::common::Toggle::UnSet(
                                internal_format::Fromatting::Bold,
                            )),
                            TextElement::Text(" test_tmp.".to_string()),
                        ],
                    },
                    OutputFormat::Html,
                ),
                "<b>test</b> test_tmp.".to_string(),
            ),
        ];
        for test_case in test_cases {
            let (test, expected_result) = test_case;
            let (text, fmt) = test;
            assert_eq!(
                crate::output_fmt::from(fmt, text).unwrap().to_string(),
                expected_result
            )
        }
    }
}
