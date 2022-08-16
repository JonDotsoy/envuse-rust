use serde::Serialize;

use super::super::error_kind::ErrorKind;
use super::super::node::Node;
use super::super::node_parser::NodeParser;
use super::super::token::PointerContext;
use super::literal::Literal;
use crate::parser::kind::Kind;
use crate::parser::token::Token;
use crate::utils::try_slice::try_slice_by_size;

#[derive(Debug, Clone, Serialize)]
pub struct InlineComment {
    pub ident_number: Box<Node>,
    pub comment: Box<Node>,
}

impl InlineComment {
    pub fn get_comment(&self) -> Literal {
        if let Kind::Literal(literal) = self.comment.clone().to_kind() {
            return literal;
        }

        panic!()
    }
}

impl From<InlineComment> for Kind {
    fn from(v: InlineComment) -> Self {
        Self::InlineComment(v)
    }
}

pub struct InlineCommentParser;

impl NodeParser for InlineCommentParser {
    fn parse<'a>(
        &self,
        payload: &'a [u8],
        pointer_context: &'a mut PointerContext,
    ) -> Result<Node, ErrorKind> {
        let start = pointer_context.clone();
        let _range_start = pointer_context.current_position();

        match try_slice_by_size(payload, pointer_context.current_position(), 1) {
            Some(a) if b"#" == a => (),
            _ => {
                return Err(ErrorKind::UnexpectedToken_deprecated);
            }
        }

        let ident_number_start_pointer_context = pointer_context.clone();
        let ident_number;

        loop {
            match try_slice_by_size(payload, pointer_context.current_position(), 1) {
                Some(a) if b"#" == a => {
                    pointer_context.move_columns(1);
                    continue;
                }
                _ => {
                    let token = Token {
                        span: pointer_context.create_span(ident_number_start_pointer_context),
                    };
                    let raw = token.slice_for_string(payload);
                    ident_number = Box::new(Node(token, Kind::from(Literal(raw))));
                    break;
                }
            }
        }

        let comment_start_pointer_context = pointer_context.clone();

        loop {
            match try_slice_by_size(payload, pointer_context.current_position(), 1) {
                Some(a) if b"\n" != a => {
                    pointer_context.move_columns(1);
                    continue;
                }
                _ => {
                    break;
                }
            }
        }

        let token = Token {
            span: pointer_context.create_span(start),
        };

        let token_comment = Token {
            span: pointer_context.create_span(comment_start_pointer_context),
        };

        let raw = String::from_utf8(token_comment.slice_for(payload).to_vec()).unwrap();

        let comment = Box::new(Node(token_comment, Kind::from(Literal(raw))));

        Ok(Node(
            token,
            Kind::from(InlineComment {
                ident_number,
                comment,
            }),
        ))
    }
}
