use crate::common;
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for element in self.text.text.iter() {
            match element {
                internal_format::TextElement::Text(t) => {
                    if let Err(e) = fmt.write_str(t.as_str()) {
                        return Err(e);
                    }
                }
                internal_format::TextElement::Marker(m) => match m {
                    common::Toggle::Set(f) => match f {
                        internal_format::Fromatting::Bold => {
                            if let Err(e) = fmt.write_str("<b>") {
                                return Err(e);
                            }
                        }
                        internal_format::Fromatting::Faint => {
                            if let Err(e) = fmt.write_str("<span style=\"font-weight:lighter;\">") {
                                return Err(e);
                            }
                        }
                        internal_format::Fromatting::Italic => {
                            if let Err(e) = fmt.write_str("<span style=\"font-style:italic;\">") {
                                return Err(e);
                            }
                        }
                        internal_format::Fromatting::Underline(o) => match o {
                            Some(color) => {
                                if let Err(e) = fmt.write_str(format!("<u style=\"-webkit-text-decoration-color:#{:x?}{:x?}{:x?}\">",color.red,color.green,color.blue).as_str()) {
                                        return Err(e);
                                    }
                            }
                            None => {
                                if let Err(e) = fmt.write_str(
                                    "<u style=\"-webkit-text-decoration-color:#{:x?}{:x?}{:x?}\">",
                                ) {
                                    return Err(e);
                                }
                            }
                        },
                        internal_format::Fromatting::CrossedOut => {
                            if let Err(e) = fmt.write_str("<s>") {
                                return Err(e);
                            }
                        }
                        internal_format::Fromatting::ForgroundColor(o) => {
                            if let Some(color) = o {
                                if let Err(e) = fmt.write_str(
                                    format!(
                                        "<span style=\"color=#{:x?}{:x?}{:x?}\">",
                                        color.red, color.green, color.blue
                                    )
                                    .as_str(),
                                ) {
                                    return Err(e);
                                }
                            }
                        }
                        internal_format::Fromatting::BackgroundColor(o) => {
                            if let Some(color) = o {
                                if let Err(e) = fmt.write_str(
                                    format!(
                                        "<span style=\"background-color=#{:x?}{:x?}{:x?}\">",
                                        color.red, color.green, color.blue
                                    )
                                    .as_str(),
                                ) {
                                    return Err(e);
                                }
                            }
                        }
                    },
                    common::Toggle::UnSet(f) => match f {
                        internal_format::Fromatting::Bold => {
                            if let Err(e) = fmt.write_str("</b>") {
                                return Err(e);
                            }
                        }
                        internal_format::Fromatting::Faint => {
                            if let Err(e) = fmt.write_str("</span>") {
                                return Err(e);
                            }
                        }
                        internal_format::Fromatting::Italic => {
                            if let Err(e) = fmt.write_str("</span>") {
                                return Err(e);
                            }
                        }
                        internal_format::Fromatting::Underline(_o) => {
                            if let Err(e) = fmt.write_str("</u>") {
                                return Err(e);
                            }
                        }
                        internal_format::Fromatting::CrossedOut => {
                            if let Err(e) = fmt.write_str("</s>") {
                                return Err(e);
                            }
                        }
                        internal_format::Fromatting::ForgroundColor(_o) => {
                            if let Err(e) = fmt.write_str("</span>") {
                                return Err(e);
                            }
                        }
                        internal_format::Fromatting::BackgroundColor(_o) => {
                            if let Err(e) = fmt.write_str("</span>") {
                                return Err(e);
                            }
                        }
                    },
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
