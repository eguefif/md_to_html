use std::iter::{Iterator, Peekable};
use std::str::Chars;

pub enum LineToken {
    LF,
    Em(String),
    Bold(String),
    EmBold(String),
    Text(String),
}

pub struct LineTokenizer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> LineTokenizer<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            iter: content.chars().peekable(),
        }
    }

    fn handle_emphasize(&mut self) -> Option<LineToken> {
        let mut kind = 1;
        let mut content = String::new();
        while let Some(peek) = self.iter.peek() {
            if ['*', '_'].contains(peek) && kind < 3 {
                self.iter.next();
                kind += 1;
            } else {
                break;
            }
        }
        let mut counter = kind;
        while let Some(next) = self.iter.next() {
            if counter == 0 {
                break;
            }
            if ['*', '_'].contains(&next) {
                counter -= 1;
            }
            content.push(next);
        }
        let content = (&content[..content.len() - kind]).to_string();
        match kind {
            1 => Some(LineToken::Em(content)),
            2 => Some(LineToken::Bold(content)),
            3 => Some(LineToken::EmBold(content)),
            _ => None,
        }
    }

    fn get_text(&mut self) -> Option<LineToken> {
        let mut text = String::new();
        while let Some(peek) = self.iter.peek() {
            if ['*', '_', '\n'].contains(peek) {
                break;
            }
            text.push(*peek);
            self.iter.next();
        }
        Some(LineToken::Text(text))
    }
}

impl<'a> Iterator for LineTokenizer<'a> {
    type Item = LineToken;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(peek) = self.iter.peek() {
            if ['*', '_'].contains(peek) {
                self.handle_emphasize()
            } else if *peek == '\n' {
                self.iter.next();
                return Some(LineToken::LF);
            } else {
                self.get_text()
            }
        } else {
            None
        }
    }
}
