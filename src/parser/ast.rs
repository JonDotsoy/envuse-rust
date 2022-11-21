use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

use crate::syntax_error::SyntaxError;

use super::{span::Span, tokenizer::Token};

/// Expression
///
/// Used
#[derive(Debug, Serialize, Deserialize)]
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
        options_variable_type: Option<BTreeMap<String, Option<Expression>>>,
        default_value: Box<Option<Expression>>,
        nullable: bool,
    },
    DefaultValue {
        span: Span,
        value: String,
    },
    OptionValue {
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
            Expression::OptionValue { span, .. } => span.clone(),
        }
    }
}

#[derive(Debug, Clone)]
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

    fn assert_current_kind<S: ToString, const U: usize>(
        &self,
        kinds: [S; U],
    ) -> Result<&Token, SyntaxError> {
        let kinds_str = kinds.map(|k| k.to_string());
        if let Some(token) = self.select_current() {
            if kinds_str.contains(&token.kind) {
                Ok(token)
            } else {
                Err(SyntaxError::new(
                    format!("Unexpected token {}", token.kind),
                    token.span.clone(),
                ))
            }
        } else {
            Err(SyntaxError::new(
                "Unexpected token",
                Span {
                    start: self.index,
                    end: self.index + 1,
                },
            ))
        }
    }

    fn forward_some_kind<S: ToString, const Z: usize>(&mut self, kinds: [S; Z]) {
        let kinds_str = kinds.map(|k| k.to_string());
        while let Some(token) = self.select_current() {
            if kinds_str.contains(&token.kind) {
                self.forward(1);
                continue;
            }
            break;
        }
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
    pub fn parse(tokens: Vec<Token>) -> Result<Expression, SyntaxError> {
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

        let elements = Self::parse_expressions_list(token_cursor)?;

        if let Some(expression) = elements.last() {
            span_end = expression.to_span().end;
        }

        Ok(Expression::Document {
            executable,
            elements,
            span: Span {
                start: span_start,
                end: span_end,
            },
        })
    }

    fn parse_expressions_list(
        tokens_cursor: &mut Cursor<Vec<Token>>,
    ) -> Result<Vec<Expression>, SyntaxError> {
        let mut vec: Vec<Expression> = vec![];

        while let Some(token) = tokens_cursor.select_current() {
            if token.kind == "newline" || token.kind == "space" {
                tokens_cursor.forward(1);
                continue;
            }
            if token.kind == "comment" {
                vec.push(Self::parse_block_comment(tokens_cursor)?);
                continue;
            }
            if token.kind == "keyword" {
                vec.push(Self::parse_variable(tokens_cursor, None)?);
                continue;
            }
            return Err(SyntaxError::new("Unexpected type", token.span.clone()));
        }

        Ok(vec)
    }

    fn parse_block_comment(
        tokens_cursor: &mut Cursor<Vec<Token>>,
    ) -> Result<Expression, SyntaxError> {
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
                return Ok(Self::parse_variable(tokens_cursor, Some(comment))?);
            }
        }

        Ok(comment)
    }

    fn parse_variable(
        tokens_cursor: &mut Cursor<Vec<Token>>,
        comment: Option<Expression>,
    ) -> Result<Expression, SyntaxError> {
        // Assert Start with keyword
        tokens_cursor.assert_current_kind(["keyword"])?;

        let name: String = tokens_cursor.select_current().unwrap().raw.to_string();
        let span_start: usize = tokens_cursor.select_current().unwrap().span.start;
        let mut span_end: usize = tokens_cursor.select_current().unwrap().span.end;
        let mut variable_type: Option<String> = None;
        let mut options_variable_type: Option<BTreeMap<String, Option<Expression>>> = None;
        let mut default_value: Option<Expression> = None;
        let mut nullable: bool = false;
        tokens_cursor.forward(1);

        tokens_cursor.forward_some_kind(["space"]);

        if tokens_cursor.assert_current_kind(["colon"]).is_ok() {
            tokens_cursor.forward(1);
            tokens_cursor.forward_some_kind(["space"]);
            tokens_cursor.assert_current_kind(["keyword"])?;
            let variable_type_token = tokens_cursor.select_current().unwrap();
            variable_type = Some(variable_type_token.raw.to_string());
            span_end = variable_type_token.span.end;
            tokens_cursor.forward(1);
            tokens_cursor.forward_some_kind(["space"]);

            if tokens_cursor.assert_current_kind(["less_than"]).is_ok() {
                tokens_cursor.forward(1);
                tokens_cursor.forward_some_kind(["space", "newline"]);

                let mut options: BTreeMap<String, Option<Expression>> = Default::default();

                while tokens_cursor.assert_current_kind(["keyword"]).is_ok() {
                    let option_key = tokens_cursor.select_current().unwrap().raw.to_string();
                    tokens_cursor.forward(1);
                    tokens_cursor.forward_some_kind(["space", "newline"]);
                    if tokens_cursor.assert_current_kind(["equal"]).is_ok() {
                        tokens_cursor.assert_current_kind(["equal"])?;
                        tokens_cursor.forward(1);
                        tokens_cursor.forward_some_kind(["space"]);
                        let value_expression = Self::parse_items_option_value(tokens_cursor)?;
                        options.insert(option_key, Some(value_expression));
                        tokens_cursor.forward_some_kind(["space", "newline"]);
                    } else {
                        options.insert(option_key, None);
                    }
                }

                let greater_than = tokens_cursor.assert_current_kind(["greater_than"])?;
                span_end = greater_than.span.end;
                tokens_cursor.forward(1);

                options_variable_type = Some(options);

                tokens_cursor.forward_some_kind(["space"]);
            }

            if let Ok(token) = tokens_cursor.assert_current_kind(["question_mark"]) {
                span_end = token.span.end;
                tokens_cursor.forward(1);
                nullable = true;
            }
        }

        if tokens_cursor.assert_current_kind(["equal"]).is_ok() {
            tokens_cursor.forward(1);
            tokens_cursor.forward_some_kind(["space"]);
            let default_value_expression = Self::parse_items_default_value(tokens_cursor)?;
            span_end = default_value_expression.to_span().end;
            default_value = Some(default_value_expression);
        }

        Ok(Expression::Variable {
            span: Span {
                start: span_start,
                end: span_end,
            },
            comment: Box::new(comment),
            name: name,
            variable_type,
            options_variable_type,
            default_value: Box::new(default_value),
            nullable,
        })
    }

    fn parse_items_default_value(
        tokens_cursor: &mut Cursor<Vec<Token>>,
    ) -> Result<Expression, SyntaxError> {
        match tokens_cursor.select_current() {
            Some(token) if token.kind == "string" => {
                Ok(Self::parse_items_default_value_string(tokens_cursor)?)
            }
            Some(token) if token.kind == "number" => {
                Ok(Self::parse_items_default_value_number(tokens_cursor)?)
            }
            Some(token) => Err(SyntaxError::new("Unexpected token", token.span.clone())),
            _ => {
                Err(SyntaxError::new(
                    "Unexpected token",
                    Span {
                        start: tokens_cursor.index,
                        end: tokens_cursor.index + 1,
                    },
                ))
                // panic!("Unexpected token");
            }
        }
    }

    fn parse_items_default_value_string(
        tokens_cursor: &mut Cursor<Vec<Token>>,
    ) -> Result<Expression, SyntaxError> {
        let token = tokens_cursor.select_current().unwrap();
        let a = Expression::DefaultValue {
            span: token.span.clone(),
            value: token.raw.to_string(),
        };
        tokens_cursor.forward(1);
        return Ok(a);
    }

    fn parse_items_default_value_number(
        tokens_cursor: &mut Cursor<Vec<Token>>,
    ) -> Result<Expression, SyntaxError> {
        let token = tokens_cursor.select_current().unwrap();
        let a = Expression::DefaultValue {
            span: token.span.clone(),
            value: token.raw.to_string(),
        };
        tokens_cursor.forward(1);
        return Ok(a);
    }

    fn parse_items_option_value(
        tokens_cursor: &mut Cursor<Vec<Token>>,
    ) -> Result<Expression, SyntaxError> {
        match tokens_cursor.select_current() {
            Some(token) if token.kind == "string" => {
                Ok(Self::parse_items_option_value_string(tokens_cursor)?)
            }
            Some(token) if token.kind == "number" => {
                Ok(Self::parse_items_option_value_number(tokens_cursor)?)
            }
            Some(token) => Err(SyntaxError::new("Unexpected token", token.span.clone())),
            _ => {
                Err(SyntaxError::new(
                    "Unexpected token",
                    Span {
                        start: tokens_cursor.index,
                        end: tokens_cursor.index + 1,
                    },
                ))
                // panic!("Unexpected token");
            }
        }
    }

    fn parse_items_option_value_string(
        tokens_cursor: &mut Cursor<Vec<Token>>,
    ) -> Result<Expression, SyntaxError> {
        let token = tokens_cursor.select_current().unwrap();
        let a = Expression::OptionValue {
            span: token.span.clone(),
            value: token.raw.to_string(),
        };
        tokens_cursor.forward(1);
        return Ok(a);
    }

    fn parse_items_option_value_number(
        tokens_cursor: &mut Cursor<Vec<Token>>,
    ) -> Result<Expression, SyntaxError> {
        let token = tokens_cursor.select_current().unwrap();
        let a = Expression::OptionValue {
            span: token.span.clone(),
            value: token.raw.to_string(),
        };
        tokens_cursor.forward(1);
        return Ok(a);
    }
}
