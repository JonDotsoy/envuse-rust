use super::super::parser::span::Span;

pub struct DisplaySyntaxDebugOptions {
    pub location: Option<String>,
    pub print_full: bool,
    pub print_underscore_error: bool,
}

impl DisplaySyntaxDebugOptions {
    pub fn new() -> Self {
        Self {
            print_full: false,
            print_underscore_error: true,
            location: None,
        }
    }
}

impl Default for DisplaySyntaxDebugOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    line: usize,
    str: String,
    span: Span,
}

#[derive(Debug, Clone)]
pub struct DisplaySyntax {
    pub message: String,
    pub span: Span,
}

impl DisplaySyntax {
    pub fn new<T: ToString>(message: T, span: Span) -> Self {
        Self {
            message: message.to_string(),
            span,
        }
    }

    pub fn helper_create_lines<T: ToString>(payload: &T) -> Vec<Line> {
        let mut lines: Vec<Line> = vec![];
        let mut current_line = Line {
            line: 1,
            str: String::new(),
            span: Span { start: 0, end: 0 },
        };

        for (index, ch) in payload.to_string().chars().enumerate() {
            current_line.span.end = index;
            if ch == '\n' {
                lines.push(current_line.clone());
                current_line = Line {
                    line: current_line.line + 1,
                    str: String::new(),
                    span: Span {
                        start: index + 1,
                        end: index + 1,
                    },
                };
                continue;
            }
            current_line.str.push(ch);
        }
        lines.push(current_line.clone());

        lines
    }

    pub fn debug_payload<T: ToString>(&self, payload: &T) -> String {
        Self::debug_payload_configurable(&self, payload, &Default::default())
    }

    pub fn debug_payload_configurable<T: ToString>(
        &self,
        payload: &T,
        options: &DisplaySyntaxDebugOptions,
    ) -> String {
        let mut buff = String::new();
        let mut at_line_pos: Option<String> = None;
        buff.push_str(format!("{}\n", self.message).as_str());
        buff.push('\n');

        // dbg!(&self.span);
        for line in Self::helper_create_lines(payload) {
            let err_start_inline =
                self.span.start >= line.span.start && self.span.start <= line.span.end;
            let err_end_inline = self.span.end >= line.span.start && self.span.end <= line.span.end;
            let err_cover_line: bool =
                self.span.start <= line.span.start && self.span.end >= line.span.end;

            if !options.print_full && !(err_start_inline || err_end_inline || err_cover_line) {
                continue;
            }

            buff.push_str(
                format!(
                    "{} {:>4} | {}\n",
                    {
                        if err_start_inline || err_end_inline || err_cover_line {
                            ">"
                        } else {
                            " "
                        }
                    },
                    line.line.to_string().as_str(),
                    // line.span.start,
                    // line.span.end,
                    line.str.as_str(),
                )
                .as_str(),
            );

            let err_subline_start: usize = {
                if err_start_inline {
                    self.span.start - line.span.start
                } else {
                    0
                }
            };

            let err_subline_end: usize = {
                if err_end_inline {
                    (line.span.end - line.span.start) - (line.span.end - self.span.end)
                } else {
                    line.span.end - line.span.start
                }
            };

            if options.print_underscore_error
                && (err_start_inline || err_end_inline || err_cover_line)
            {
                buff.push_str(
                    format!(
                        "{}{}\n",
                        " ".repeat(9 + err_subline_start),
                        "▀".repeat(err_subline_end - err_subline_start)
                    )
                    .as_str(),
                )
            }

            if at_line_pos.is_none() && (err_start_inline || err_end_inline || err_cover_line) {
                at_line_pos = Some(format!(
                    "{}:{}",
                    // options.location.clone().unwrap(),
                    line.line,
                    err_subline_start + 1
                ))
            }
        }

        if let Some(pos) = at_line_pos {
            buff.push_str(&format!(
                "    at {}:{}\n",
                options.location.clone().unwrap_or("<unknown>".to_string()),
                pos,
            ))
        }

        buff
    }
}
