use serde::Serialize;

use super::tokenizer::{Span, Token};

/// Expression
///
/// Used
#[derive(Debug, Serialize)]
pub enum Expression {
    Document {
        span: Span,
        executable: Option<String>,
        elements: Vec<Expression>,
    },
    CommentBlock {
        span: Span,
        raw: Vec<String>,
    },
    Variable {
        span: Span,
        comment: Box<Option<Expression>>,
        name: String,
        variable_type: Option<String>,
        default_value: Box<Option<Expression>>,
    },
    DefaultValue {
        span: Span,
        value: String,
    },
}

impl Expression {
    fn to_span(&self) -> Span {
        match self {
            Expression::Document { span, .. } => span.clone(),
            Expression::CommentBlock { span, .. } => span.clone(),
            Expression::Variable { span, .. } => span.clone(),
            Expression::DefaultValue { span, .. } => span.clone(),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Cursor<T>
where
    T: AsRef<Vec<Token>>,
{
    index: usize,
    items: T,
}

impl<T> Cursor<T>
where
    T: AsRef<Vec<Token>>,
{
    fn new(items: T) -> Self {
        Self { index: 0, items }
    }

    fn select_current(&self) -> Option<&Token> {
        self.items.as_ref().get(self.index)
    }

    fn select_next(&self) -> Option<&Token> {
        self.items.as_ref().get(self.index + 1)
    }

    fn select_nexts(&self, steps: usize) -> Option<&Token> {
        self.items.as_ref().get(self.index + steps)
    }

    fn select_prev(&self) -> Option<&Token> {
        self.items.as_ref().get(self.index - 1)
    }

    fn next(&mut self) -> Option<&Token> {
        self.forward(1);
        self.select_prev()
    }

    fn forward(&mut self, steps: usize) {
        self.index = self.index + steps;
    }
}

pub struct AST {}

impl AST {
    pub fn parse(tokens: Vec<Token>) -> Expression {
        let mut executable: Option<String> = None;
        let ref mut token_cursor = Cursor::new(tokens);
        let mut span_start: usize = 0;
        let mut span_end: usize = 0;

        if let Some(token) = token_cursor.select_current() {
            if token.kind == "comment" && token.raw.starts_with("#!") {
                span_start = token.span.start;
                span_end = token.span.end;
                executable = Some(token.raw.to_string());
                token_cursor.forward(1);
            }
        }

        let elements = Self::parse_items(token_cursor);

        if let Some(expression) = elements.last() {
            span_end = expression.to_span().end;
        }

        Expression::Document {
            executable,
            elements,
            span: Span {
                start: span_start,
                end: span_end,
            },
        }
    }

    fn parse_items(tokens_cursor: &mut Cursor<Vec<Token>>) -> Vec<Expression> {
        let mut vec: Vec<Expression> = vec![];
        let mut block_comment: Option<Expression> = None;

        while let Some(token) = tokens_cursor.select_current() {
            if token.kind == "newline" || token.kind == "space" {
                tokens_cursor.forward(1);
                continue;
            }
            if token.kind == "comment" {
                vec.push(Self::parse_items_block_comment(tokens_cursor));
                continue;
            }
            if token.kind == "keyword" {
                vec.push(Self::parse_items_variable(tokens_cursor, None));
                continue;
            }
            // tokens_cursor.forward(1);
            dbg!(token);
            todo!("Unexpected type")
        }

        vec
    }

    fn parse_items_block_comment(tokens_cursor: &mut Cursor<Vec<Token>>) -> Expression {
        let span_start = tokens_cursor.select_current().unwrap().span.start;
        let mut span_end = tokens_cursor.select_current().unwrap().span.end;
        let mut raw: Vec<String> = vec![];

        while let Some(token) = tokens_cursor.select_current() {
            if token.kind == "space" {
                tokens_cursor.forward(1);
                continue;
            }

            if token.kind != "comment" {
                break;
            }

            raw.push(token.raw.to_string());
            span_end = token.span.end;

            if let Some(token) = tokens_cursor.select_next() {
                if token.kind == "newline" {
                    tokens_cursor.forward(1);
                }
            }

            tokens_cursor.forward(1);
        }

        let comment = Expression::CommentBlock {
            span: Span {
                start: span_start,
                end: span_end,
            },
            raw,
        };

        if let Some(token) = tokens_cursor.select_current() {
            if token.kind == "keyword" {
                return Self::parse_items_variable(tokens_cursor, Some(comment));
            }
        }

        comment
    }

    fn parse_items_variable(
        tokens_cursor: &mut Cursor<Vec<Token>>,
        comment: Option<Expression>,
    ) -> Expression {
        let name: String = tokens_cursor.select_current().unwrap().raw.to_string();
        let span_start = tokens_cursor.select_current().unwrap().span.start;
        let mut span_end = tokens_cursor.select_current().unwrap().span.end;
        let mut variable_type: Option<String> = None;
        let mut default_value: Option<Expression> = None;
        tokens_cursor.forward(1);

        while let Some(token) = tokens_cursor.select_current() {
            if token.kind == "space" {
                tokens_cursor.forward(1);
                continue;
            }

            if token.kind == "newline" {
                tokens_cursor.forward(1);
                break;
            }

            if token.kind == "colon" {
                let mut forward_steps = 1;
                if let Some(token) = tokens_cursor.select_nexts(forward_steps) {
                    forward_steps = forward_steps + 1;
                    if let Some(token) = tokens_cursor.select_nexts(forward_steps) {
                        if token.kind == "space" {
                            forward_steps = forward_steps + 1;
                        }
                    }

                    if let Some(token) = tokens_cursor.select_nexts(forward_steps) {
                        if token.kind == "keyword" {
                            forward_steps = forward_steps + 1;
                            variable_type = Some(token.raw.to_string());
                            span_end = token.span.end;
                            tokens_cursor.forward(forward_steps);
                            continue;
                        }
                    }

                    // dbg!(forward_steps);
                    // dbg!(token);
                    // dbg!(tokens_cursor.select_nexts(forward_steps));
                    // tokens_cursor.forward(1);
                    // todo!();
                }
            }

            if token.kind == "equal" {
                let mut forward_steps = 1;
                if let Some(token) = tokens_cursor.select_nexts(1) {
                    if token.kind == "space" {
                        forward_steps = forward_steps + 1;
                    }
                }
                tokens_cursor.forward(forward_steps);
                default_value = Some(Self::parse_items_default_value(tokens_cursor));
                continue;
            }

            dbg!(token);
            todo!()
        }

        Expression::Variable {
            span: Span {
                start: span_start,
                end: span_end,
            },
            comment: Box::new(comment),
            name: name,
            variable_type,
            default_value: Box::new(default_value),
        }
    }

    fn parse_items_default_value(tokens_cursor: &mut Cursor<Vec<Token>>) -> Expression {
        match tokens_cursor.select_current() {
            Some(token) if token.kind == "string" => {
                Self::parse_items_default_value_string(tokens_cursor)
            }
            Some(token) if token.kind == "number" => {
                Self::parse_items_default_value_number(tokens_cursor)
            }
            _ => {
                panic!("Unexpected token");
            }
        }
    }

    fn parse_items_default_value_string(tokens_cursor: &mut Cursor<Vec<Token>>) -> Expression {
        let token = tokens_cursor.select_current().unwrap();
        let a = Expression::DefaultValue {
            span: token.span.clone(),
            value: token.raw.to_string(),
        };
        tokens_cursor.forward(1);
        return a;
    }

    fn parse_items_default_value_number(tokens_cursor: &mut Cursor<Vec<Token>>) -> Expression {
        let token = tokens_cursor.select_current().unwrap();
        let a = Expression::DefaultValue {
            span: token.span.clone(),
            value: token.raw.to_string(),
        };
        tokens_cursor.forward(1);
        return a;
    }
}
