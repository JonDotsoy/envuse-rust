use std::{
    ops::{Add, RangeBounds, RangeInclusive, RangeToInclusive},
    usize, vec,
};

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
    fn current_char(&self) -> Option<char> {
        self.payload.chars().nth(self.index)
    }

    fn current_char_expected(&self, val: char) {
        if !self.current_matches_char(val) {
            todo!(
                "Require error expected the char {} (ascii code {})",
                val,
                val as usize
            );
        }
    }

    fn current_matches_char(&self, val: char) -> bool {
        if let Some(c) = self.current_char() {
            c == val
        } else {
            false
        }
    }

    pub fn current_matches_range_char<T>(&self, vec_ranges: &Vec<T>) -> bool
    where
        T: RangeBounds<char> + std::fmt::Debug,
    {
        let current_char = self.current_char();
        if let Some(current_char) = current_char {
            for e in vec_ranges {
                if e.contains(&current_char) {
                    return true;
                }
            }
        }

        return false;
    }

    fn current_range_char_expected<T>(&self, vec_ranges: &Vec<T>)
    where
        T: RangeBounds<char> + std::fmt::Debug,
    {
        if !self.current_matches_range_char(&vec_ranges) {
            todo!("Require error expected the char {:?}", &vec_ranges);
        }
    }

    fn forward(&mut self, positions: usize) {
        self.index = self.index + positions;
    }

    fn get_by_span(&self, span: &Span) -> String {
        unsafe { self.payload.get_unchecked(span.start..span.end).to_string() }
    }

    fn has_current<'a>(&'a self) -> bool {
        self.payload.len() > self.index
    }

    fn has_next<'a>(&'a self) -> bool {
        (self.payload.len() > (self.index + 1)).clone()
    }

    fn new<A>(index: usize, payload: A) -> Self
    where
        A: ToString,
    {
        Self {
            payload: payload.to_string(),
            index,
        }
    }

    fn next_char(&self) -> Option<char> {
        self.payload.chars().nth(self.index + 1)
    }

    fn next_matches_char(&self, val: char) -> bool {
        if let Some(c) = self.next_char() {
            c == val
        } else {
            false
        }
    }

    pub fn next_matches_range_char<T>(&self, vec_ranges: &Vec<T>) -> bool
    where
        T: RangeBounds<char> + std::fmt::Debug,
    {
        let current_char = self.next_char();
        if let Some(current_char) = current_char {
            for e in vec_ranges {
                if e.contains(&current_char) {
                    return true;
                }
            }
        }

        return false;
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
                tokens.extend(Self::parse_newline(cursor));
                continue;
            }

            if cursor.current_matches_range_char(&vec![' '..=' ', '\t'..='\t']) {
                tokens.extend(Self::parse_spaces(cursor));
                continue;
            }

            if cursor.current_matches_char(':') {
                tokens.extend(Self::parse_colon(cursor));
                continue;
            }

            if cursor.current_matches_char('=') {
                tokens.extend(Self::parse_equal(cursor));
                continue;
            }

            if cursor.current_matches_char('"') {
                tokens.extend(Self::parse_string(cursor));
                continue;
            }

            if cursor.current_matches_range_char(&vec!['a'..='z', 'A'..='Z', '_'..='_']) {
                tokens.extend(Self::parse_keyword(cursor));
                continue;
            }

            if cursor.current_matches_range_char(&vec!['0'..='9']) {
                tokens.extend(Self::parse_number(cursor));
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

    fn parse_newline(cursor: &mut Cursor) -> Vec<Token> {
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

    fn parse_spaces(cursor: &mut Cursor) -> Vec<Token> {
        cursor.current_range_char_expected(&vec![' '..=' ', '\t'..='\t']);
        let span_start = cursor.index;
        while cursor.has_current() {
            if cursor.current_matches_range_char(&vec![' '..=' ', '\t'..='\t']) {
                cursor.forward(1);
                continue;
            }
            break;
        }
        let span = Span {
            start: span_start,
            end: cursor.index,
        };
        let newline_token = Token::new("space", &cursor, span);
        vec![newline_token]
    }

    fn parse_equal(cursor: &mut Cursor) -> Vec<Token> {
        cursor.current_char_expected('=');
        let span_start = cursor.index;
        cursor.forward(1);
        let span = Span {
            start: span_start,
            end: cursor.index,
        };
        let newline_token = Token::new("equal", &cursor, span);
        vec![newline_token]
    }

    fn parse_colon(cursor: &mut Cursor) -> Vec<Token> {
        cursor.current_char_expected(':');
        let span_start = cursor.index;
        cursor.forward(1);
        let span = Span {
            start: span_start,
            end: cursor.index,
        };
        let newline_token = Token::new("colon", &cursor, span);
        vec![newline_token]
    }

    fn parse_string(cursor: &mut Cursor) -> Vec<Token> {
        cursor.current_char_expected('"');
        cursor.forward(1);
        let span_start = cursor.index;

        while cursor.has_current() {
            if cursor.current_matches_char('\\') {
                cursor.forward(2);
                continue;
            }
            if cursor.current_matches_char('"') {
                break;
            }
            cursor.forward(1);
        }

        cursor.current_char_expected('"');

        let span = Span {
            start: span_start,
            end: cursor.index,
        };
        cursor.forward(1);
        let newline_token = Token::new("string", &cursor, span);
        vec![newline_token]
    }

    fn parse_keyword(cursor: &mut Cursor) -> Vec<Token> {
        cursor.current_range_char_expected(&vec!['a'..='z', 'A'..='Z', '0'..='9', '_'..='_']);
        let span_start = cursor.index;

        while cursor.has_current() {
            if !cursor.current_matches_range_char(&vec!['a'..='z', 'A'..='Z', '0'..='9', '_'..='_'])
            {
                break;
            }
            cursor.forward(1);
        }

        let span = Span {
            start: span_start,
            end: cursor.index,
        };
        let newline_token = Token::new("keyword", &cursor, span);
        vec![newline_token]
    }

    fn parse_number(cursor: &mut Cursor) -> Vec<Token> {
        cursor.current_range_char_expected(&vec!['0'..='9']);
        let span_start = cursor.index;
        let mut decimal = false;

        while cursor.has_current() {
            if cursor.current_matches_range_char(&vec!['0'..='9']) {
                cursor.forward(1);
                continue;
            }
            if cursor.current_matches_char('_') {
                if !cursor.next_matches_range_char(&vec!['0'..='9']) {
                    todo!("Only one undesrcore is allowed as numeric separator");
                }
                cursor.forward(1);
                continue;
            }
            if cursor.current_matches_char('.') {
                if decimal {
                    todo!("Unexpected number");
                }
                if !cursor.next_matches_range_char(&vec!['0'..='9']) {
                    todo!("Invalid or unexpected token");
                }
                decimal = true;
                cursor.forward(1);
                continue;
            }
            break;
        }

        let span = Span {
            start: span_start,
            end: cursor.index,
        };
        let newline_token = Token::new("number", &cursor, span);
        vec![newline_token]
    }
}
