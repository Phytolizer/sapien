use std::cmp::Ordering;
use std::fmt::Display;

use crate::plumbing::ObjectKind;

#[derive(Debug, Clone, PartialEq)]
pub struct TextSpan {
    pub start: usize,
    pub length: usize,
}

impl TextSpan {
    pub fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }

    pub fn end(&self) -> usize {
        self.start + self.length
    }

    pub(crate) fn from_bounds(start: usize, end: usize) -> TextSpan {
        TextSpan {
            start,
            length: end - start,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableSymbol {
    pub name: String,
    pub is_read_only: bool,
    pub kind: ObjectKind,
}

#[derive(Clone)]
pub struct SourceText {
    text: Vec<char>,
    pub lines: Vec<TextLine>,
}

impl SourceText {
    fn new(text: Vec<char>) -> Self {
        Self {
            lines: Self::parse_lines(&text),
            text,
        }
    }

    pub fn from(text: Vec<char>) -> Self {
        Self::new(text)
    }

    pub fn get_line_index(&self, position: usize) -> usize {
        let mut lower = 0isize;
        let mut upper = self.lines.len() as isize - 1;
        while lower <= upper {
            let index = lower + (upper - lower) / 2;
            let start = self.lines[index as usize].start;

            match start.cmp(&position) {
                Ordering::Less => lower = index + 1,
                Ordering::Equal => return index as usize,
                Ordering::Greater => upper = index - 1,
            }
        }
        (lower - 1) as usize
    }

    fn parse_lines(text: &[char]) -> Vec<TextLine> {
        let mut lines = Vec::new();
        let mut line_start = 0;
        let mut position = 0;
        while position < text.len() {
            let line_break_width = Self::get_line_break_width(text, position);

            if line_break_width == 0 {
                position += 1;
            } else {
                add_line(&mut lines, line_start, position, line_break_width);
                position += line_break_width;
                line_start = position;
            }
        }
        if position >= line_start {
            add_line(&mut lines, line_start, position, 0);
        }
        lines
    }

    fn get_line_break_width(text: &[char], i: usize) -> usize {
        let c = text[i];
        let l = if (i + 1) >= text.len() {
            '\0'
        } else {
            text[i + 1]
        };
        match (c, l) {
            ('\r', '\n') => 2,
            ('\r' | '\n', _) => 1,
            _ => 0,
        }
    }

    pub fn as_chars(&self) -> &[char] {
        &self.text
    }

    pub fn to_string_bounded(&self, start: usize, length: usize) -> String {
        self.text[start..start + length].iter().collect()
    }

    pub fn to_string_spanned(&self, span: TextSpan) -> String {
        self.to_string_bounded(span.start, span.length)
    }

    pub fn to_string_line(&self, line: &TextLine) -> String {
        self.to_string_spanned(line.span())
    }

    pub fn to_string_line_including_line_break(&self, line: &TextLine) -> String {
        self.to_string_spanned(line.span_including_line_break())
    }
}

impl Display for SourceText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text.iter().collect::<String>())
    }
}

fn add_line(
    lines: &mut Vec<TextLine>,
    line_start: usize,
    position: usize,
    line_break_width: usize,
) {
    lines.push(TextLine {
        start: line_start,
        length: position - line_start,
        length_including_line_break: position - line_start + line_break_width,
    });
}

#[derive(Clone)]
pub struct TextLine {
    pub start: usize,
    pub length: usize,
    pub length_including_line_break: usize,
}

impl TextLine {
    pub fn span(&self) -> TextSpan {
        TextSpan::new(self.start, self.length)
    }

    pub fn span_including_line_break(&self) -> TextSpan {
        TextSpan::new(self.start, self.length_including_line_break)
    }

    pub fn end(&self) -> usize {
        self.start + self.length
    }
}

#[cfg(test)]
mod tests {
    use super::SourceText;

    #[test]
    fn source_text_includes_last_line() {
        for (text, expected_line_count) in [(".", 1), (".\r\n", 2), (".\r\n\r\n", 3)] {
            let source_text = SourceText::from(text.chars().collect());
            assert_eq!(expected_line_count, source_text.lines.len());
        }
    }
}
