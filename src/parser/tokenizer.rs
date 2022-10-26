use std::{ops::Add, usize, vec};

#[derive(Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct Token {
    pub kind: String,
    pub raw: String,
    pub span: Span,
}

impl Token {
    fn new<A: ToString>(kind: A, cursor: &Cursor, span: Span) -> Self {
        Self {
            kind: kind.to_string(),
            raw: cursor.get_by_span(&span),
            span,
        }
    }
}

#[derive(Debug)]
struct Cursor {
    payload: String,
    index: usize,
}

impl Cursor {
    fn new<A>(index: usize, payload: A) -> Self
    where
        A: ToString,
    {
        Self {
            payload: payload.to_string(),
            index,
        }
    }

    fn has_next<'a>(&'a self) -> bool {
        (self.payload.len() > (self.index + 1)).clone()
    }

    fn has_current<'a>(&'a self) -> bool {
        self.payload.len() > self.index
    }

    fn current_matches_char(&self, val: char) -> bool {
        if let Some(c) = self.current_char() {
            c == val
        } else {
            false
        }
    }

    fn next_matches_char(&self, val: char) -> bool {
        if let Some(c) = self.next_char() {
            c == val
        } else {
            false
        }
    }

    fn next_matches_range_char(&self, range: std::ops::RangeInclusive<char>) -> bool {
        matches!(self.next_char(), range)
    }

    fn next_char(&self) -> Option<char> {
        self.payload.chars().nth(self.index + 1)
    }

    fn current_char(&self) -> Option<char> {
        self.payload.chars().nth(self.index)
    }

    fn forward(&mut self, positions: usize) {
        self.index = self.index + positions;
    }

    fn current_char_expected(&self, val: char) {
        if let Some(c) = self.current_char() {
            if c != val {
                todo!("Require error expected the char {}", val);
            }
        } else {
            todo!("Require error: cannot found current char");
        }
    }

    fn get_by_span(&self, span: &Span) -> String {
        unsafe { self.payload.get_unchecked(span.start..span.end).to_string() }
    }
}

pub struct Tokenizer {}

impl Tokenizer {
    pub fn parse<A>(payload: A) -> Vec<Token>
    where
        A: ToString,
    {
        let ref mut cursor = Cursor::new(0, payload);
        return Self::parse_by_cursor(cursor);
    }

    fn parse_by_cursor(cursor: &mut Cursor) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while cursor.has_current() {
            if cursor.current_matches_char('#') {
                tokens.extend(Self::parse_comment(cursor));
                continue;
            }

            if cursor.current_matches_char('\n') {
                tokens.extend(Self::parse_new_line(cursor));
                continue;
            }

            dbg!(&cursor);
            dbg!(&cursor.current_char());
            dbg!(&tokens);
            todo!();
        }

        tokens
    }

    fn parse_comment(cursor: &mut Cursor) -> Vec<Token> {
        cursor.current_char_expected('#');
        let span_start = cursor.index;

        while cursor.has_current() {
            cursor.forward(1);
            if cursor.current_matches_char('\n') {
                break;
            }
        }

        let span = Span {
            start: span_start,
            end: cursor.index,
        };
        let comment_token = Token::new("comment", &cursor, span);
        vec![comment_token]
    }

    fn parse_new_line(cursor: &mut Cursor) -> Vec<Token> {
        cursor.current_char_expected('\n');
        let span_start = cursor.index;
        cursor.forward(1);
        let span = Span {
            start: span_start,
            end: cursor.index,
        };
        let newline_token = Token::new("newline", &cursor, span);
        vec![newline_token]
    }
}
