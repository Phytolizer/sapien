use crate::plumbing::Object;
use crate::syntax::SyntaxKind;
use crate::syntax::SyntaxToken;

pub(crate) struct Lexer {
    input: Vec<char>,
    position: usize,
    start: usize,
    kind: SyntaxKind,
    value: Object,
}

impl Lexer {
    pub(crate) fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            start: 0,
            kind: SyntaxKind::BadToken,
            value: Object::Null,
        }
    }

    fn current(&self) -> char {
        self.input.get(self.position).copied().unwrap_or('\0')
    }

    pub(crate) fn next_token(&mut self) -> SyntaxToken {
        self.start = self.position;
        self.value = Object::Null;
        self.kind = match self.current() {
            '\0' => SyntaxKind::EndOfFileToken,
            c if c.is_numeric() => {
                while self.current().is_numeric() {
                    self.position += 1;
                }
                let text = self.input[self.start..self.position]
                    .iter()
                    .collect::<String>();
                let value = text.parse::<i64>().unwrap();
                self.value = Object::Number(value);
                SyntaxKind::NumberToken
            }
            c if c.is_whitespace() => {
                while self.current().is_whitespace() {
                    self.position += 1;
                }
                SyntaxKind::WhitespaceToken
            }
            '+' => {
                self.position += 1;
                SyntaxKind::PlusToken
            }
            '-' => {
                self.position += 1;
                SyntaxKind::MinusToken
            }
            '*' => {
                self.position += 1;
                SyntaxKind::StarToken
            }
            '/' => {
                self.position += 1;
                SyntaxKind::SlashToken
            }
            '(' => {
                self.position += 1;
                SyntaxKind::OpenParenthesisToken
            }
            ')' => {
                self.position += 1;
                SyntaxKind::CloseParenthesisToken
            }
            _ => {
                self.position += 1;
                SyntaxKind::BadToken
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
