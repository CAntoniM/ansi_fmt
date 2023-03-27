use crate::internal_format;
use crate::output_fmt;

pub struct HtmlWriter {
    text: internal_format::Text,
}

impl HtmlWriter {
    pub fn from_text(txt: internal_format::Text) -> HtmlWriter {
        HtmlWriter { text: txt }
    }
}

impl std::fmt::Display for HtmlWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for element in self.text.text.iter() {
            match element {
                internal_format::TextElement::Text(t) => {
                    if let Err(e) = f.write_str(t.as_str()) {
                        return Err(e);
                    }
                }
                internal_format::TextElement::Marker(m) => match m {
                    _ => {}
                },
            }
        }
        Ok(())
    }
}

impl output_fmt::Formatter for HtmlWriter {}

#[cfg(test)]
mod test {
    use crate::{internal_format, internal_format::TextElement, output_fmt::html::HtmlWriter};

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
            assert_eq!(HtmlWriter { text: test }.to_string(), expected_result)
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
                "<b>test</b>".to_string(),
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
                "<b>test</b> test_tmp.".to_string(),
            ),
        ];
        for test_case in test_cases {
            let (test, expected_result) = test_case;
            assert_eq!(HtmlWriter::from_text(test).to_string(), expected_result)
        }
    }
}
