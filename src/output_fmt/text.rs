use crate::internal_format;
use crate::output_fmt;

pub struct TextWriter {
    text: internal_format::Text,
}

impl TextWriter {
    pub fn from_text(txt: internal_format::Text) -> TextWriter {
        TextWriter { text: txt }
    }
}

impl std::fmt::Display for TextWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for element in self.text.text.iter() {
            if let internal_format::TextElement::Text(t) = element {
                match f.write_str(t.as_str()) {
                    Err(e) => return Err(e),
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

impl output_fmt::Formatter for TextWriter {}

#[cfg(test)]
mod test {
    use crate::{internal_format, internal_format::TextElement, output_fmt::text::TextWriter};

    #[test]
    pub fn text_writer_fmt() {
        let test_cases = [
            (internal_format::Text { text: vec![] }, "".to_string()),
            (
                internal_format::Text {
                    text: vec![TextElement::Text("test".to_string())],
                },
                "test".to_string(),
            ),
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
                    ],
                },
                "test".to_string(),
            ),
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
                "test test_tmp.".to_string(),
            ),
        ];
        for test_case in test_cases {
            let (test, expected_result) = test_case;
            assert_eq!(TextWriter { text: test }.to_string(), expected_result)
        }
    }

    #[test]
    pub fn text_writer_from_text() {
        let test_cases = [
            (internal_format::Text { text: vec![] }, "".to_string()),
            (
                internal_format::Text {
                    text: vec![TextElement::Text("test".to_string())],
                },
                "test".to_string(),
            ),
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
                    ],
                },
                "test".to_string(),
            ),
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
                "test test_tmp.".to_string(),
            ),
        ];
        for test_case in test_cases {
            let (test, expected_result) = test_case;
            assert_eq!(TextWriter::from_text(test).to_string(), expected_result)
        }
    }
}
