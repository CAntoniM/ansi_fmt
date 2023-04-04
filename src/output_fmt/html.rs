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

fn remove_fmt(
    fmt: &internal_format::Fromatting,
    current_active_formats: &mut Vec<internal_format::Fromatting>,
) {
    let mut i: usize = 0;
    for cur_fmt in current_active_formats.iter() {
        if fmt == cur_fmt {
            current_active_formats.remove(i);
            break;
        }
        i = i + 1;
    }
}

fn contains_fmt(
    fmt: &internal_format::Fromatting,
    current_active_formats: &Vec<internal_format::Fromatting>,
) -> bool {
    for format in current_active_formats.iter() {
        match format {
            internal_format::Fromatting::BackgroundColor(_) => {
                if fmt.clone() == internal_format::Fromatting::BackgroundColor(None) {
                    return true;
                }
            }
            internal_format::Fromatting::ForgroundColor(_) => {
                if fmt.clone() == internal_format::Fromatting::ForgroundColor(None) {
                    return true;
                }
            }
            internal_format::Fromatting::Underline(_) => {
                if fmt.clone() == internal_format::Fromatting::Underline(None) {
                    return true;
                }
            }
            _ => {
                if fmt.clone() == format.clone() {
                    return true;
                }
            }
        }
    }
    return false;
}

fn formats_to_styles(current_active_formats: &Vec<internal_format::Fromatting>) -> String {
    let mut output_buffer = String::new();
    for fmt in current_active_formats.iter() {
        match fmt {
            internal_format::Fromatting::Bold => output_buffer.push_str("font-weight:bold"),
            internal_format::Fromatting::Faint => output_buffer.push_str("font-weight:lighter"),
            internal_format::Fromatting::Italic => output_buffer.push_str("font-style:italic"),
            internal_format::Fromatting::Underline(Some(c)) => output_buffer.push_str(
                format!(
                    "font-decoration:line-through;text-decoration-color:#{:x}{:x}{:x}",
                    c.red(),
                    c.blue(),
                    c.green()
                )
                .as_str(),
            ),
            internal_format::Fromatting::Underline(None) => {
                output_buffer.push_str("font-decoration:line-through")
            }
            internal_format::Fromatting::CrossedOut => {
                output_buffer.push_str("font-decoration:line-through")
            }
            internal_format::Fromatting::ForgroundColor(None) => {
                output_buffer.push_str("color:inherit")
            }
            internal_format::Fromatting::BackgroundColor(None) => {
                output_buffer.push_str("background-color:inherit")
            }
            internal_format::Fromatting::ForgroundColor(Some(c)) => output_buffer
                .push_str(format!("color:#{:x}{:x}{:x}", c.red(), c.blue(), c.green()).as_str()),
            internal_format::Fromatting::BackgroundColor(Some(c)) => output_buffer.push_str(
                format!(
                    "background-color:#{:x}{:x}{:x}",
                    c.red(),
                    c.blue(),
                    c.green()
                )
                .as_str(),
            ),
        };
        output_buffer.push(';');
    }
    return output_buffer;
}

impl std::fmt::Display for HtmlWriter {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current_active_fmt: Vec<internal_format::Fromatting> = Vec::new();
        for element in self.text.text.iter() {
            match element {
                internal_format::TextElement::Text(t) => {
                    let fmt_string: String;
                    if current_active_fmt.len() > 0 {
                        fmt_string = format!(
                            "<span style=\"{}\">{}</span>",
                            formats_to_styles(&current_active_fmt),
                            t
                        )
                    } else {
                        fmt_string = t.clone();
                    }
                    if let Err(e) = fmt.write_str(&fmt_string.as_str()) {
                        return Err(e);
                    }
                }
                internal_format::TextElement::Marker(m) => match m {
                    common::Toggle::Set(f) => current_active_fmt.push(f.clone()),
                    common::Toggle::UnSet(f) => {
                        if contains_fmt(f, &current_active_fmt) {
                            remove_fmt(f, &mut current_active_fmt);
                        }
                    }
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
                "<span style=\"font-weight:bold;\">test</span>".to_string(),
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
                "<span style=\"font-weight:bold;\">test</span> test_tmp.".to_string(),
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
                "<span style=\"font-weight:bold;\">test</span>".to_string(),
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
                "<span style=\"font-weight:bold;\">test</span> test_tmp.".to_string(),
            ),
        ];
        for test_case in test_cases {
            let (test, expected_result) = test_case;
            assert_eq!(HtmlWriter::from_text(test).to_string(), expected_result)
        }
    }
}
