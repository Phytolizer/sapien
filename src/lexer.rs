use crate::plumbing::Object;
use crate::syntax::keyword_kind;
use crate::syntax::SyntaxKind;
use crate::syntax::SyntaxToken;

pub(crate) struct Lexer {
    input: Vec<char>,
    position: usize,
    start: usize,
    kind: SyntaxKind,
    value: Object,
    pub(crate) diagnostics: Vec<String>,
}

impl Lexer {
    pub(crate) fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            start: 0,
            kind: SyntaxKind::BadToken,
            value: Object::Null,
            diagnostics: Vec::new(),
        }
    }

    fn peek(&self, offset: usize) -> char {
        self.input
            .get(self.position + offset)
            .copied()
            .unwrap_or('\0')
    }

    fn current(&self) -> char {
        self.peek(0)
    }

    fn lookahead(&self) -> char {
        self.peek(1)
    }

    pub(crate) fn next_token(&mut self) -> SyntaxToken {
        self.start = self.position;
        self.value = Object::Null;
        match self.current() {
            '\0' => {
                self.kind = SyntaxKind::EndOfFileToken;
            }
            c if c.is_numeric() => {
                while self.current().is_numeric() {
                    self.position += 1;
                }
                let text = self.input[self.start..self.position]
                    .iter()
                    .collect::<String>();
                let value = match text.parse::<i64>() {
                    Ok(v) => v,
                    Err(_) => {
                        self.diagnostics
                            .push(format!("ERROR: invalid i64: {}", text));
                        0
                    }
                };
                self.value = Object::Number(value);
                self.kind = SyntaxKind::NumberToken;
            }
            c if c.is_alphabetic() => {
                while self.current().is_alphabetic() {
                    self.position += 1;
                }
                let text = self.input[self.start..self.position]
                    .iter()
                    .collect::<String>();
                self.kind = keyword_kind(&text);
            }
            c if c.is_whitespace() => {
                while self.current().is_whitespace() {
                    self.position += 1;
                }
                self.kind = SyntaxKind::WhitespaceToken;
            }
            '+' => {
                self.position += 1;
                self.kind = SyntaxKind::PlusToken;
            }
            '-' => {
                self.position += 1;
                self.kind = SyntaxKind::MinusToken;
            }
            '*' => {
                self.position += 1;
                self.kind = SyntaxKind::StarToken;
            }
            '/' => {
                self.position += 1;
                self.kind = SyntaxKind::SlashToken;
            }
            '(' => {
                self.position += 1;
                self.kind = SyntaxKind::OpenParenthesisToken;
            }
            ')' => {
                self.position += 1;
                self.kind = SyntaxKind::CloseParenthesisToken;
            }
            '!' if self.lookahead() == '=' => {
                self.position += 2;
                self.kind = SyntaxKind::BangEqualsToken;
            }
            '!' => {
                self.position += 1;
                self.kind = SyntaxKind::BangToken;
            }
            '&' if self.lookahead() == '&' => {
                self.position += 2;
                self.kind = SyntaxKind::AmpersandAmpersandToken;
            }
            '|' if self.lookahead() == '|' => {
                self.position += 2;
                self.kind = SyntaxKind::PipePipeToken;
            }
            '=' if self.lookahead() == '=' => {
                self.position += 2;
                self.kind = SyntaxKind::EqualsEqualsToken;
            }
            _ => {
                self.diagnostics
                    .push(format!("ERROR: bad character input: '{}'", self.current()));
                self.position += 1;
                self.kind = SyntaxKind::BadToken;
            }
        };
        SyntaxToken::new(
            self.kind,
            self.start,
            self.input[self.start..self.position].iter().collect(),
            self.value.clone(),
        )
    }
}
