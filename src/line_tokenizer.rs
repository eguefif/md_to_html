use std::iter::{Iterator, Peekable};
use std::str::Chars;

pub struct Link {
    pub url: String,
    pub a: String,
}

pub enum LineToken {
    LF,
    Em(String),
    Bold(String),
    EmBold(String),
    Text(String),
    Url(Link),
    Code(String),
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
        let mut kind = 0;
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
        while let Some(peek) = self.iter.peek() {
            if counter == 0 {
                break;
            }
            if ['*', '_'].contains(peek) {
                counter -= 1;
            }
            let next = self.iter.next().unwrap();
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
            if ['[', '*', '_', '\n', '`'].contains(peek) {
                break;
            }
            text.push(*peek);
            self.iter.next();
        }
        Some(LineToken::Text(text))
    }

    fn handle_link(&mut self) -> Option<LineToken> {
        let mut url = String::new();
        let mut a = String::new();
        self.iter.next();
        while let Some(next) = self.iter.next_if(|peek| *peek != ']') {
            a.push(next);
        }
        self.iter.next();
        self.iter.next();
        while let Some(next) = self.iter.next_if(|peek| *peek != ')') {
            url.push(next);
        }
        self.iter.next();

        Some(LineToken::Url(Link { url, a }))
    }
    fn handle_line_code(&mut self) -> Option<LineToken> {
        let mut code = String::new();
        self.iter.next();
        while let Some(next) = self.iter.next() {
            if next == '`' {
                break;
            }
            code.push(next);
        }
        Some(LineToken::Code(code))
    }
}

impl<'a> Iterator for LineTokenizer<'a> {
    type Item = LineToken;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(peek) = self.iter.peek() {
            match peek {
                '*' | '_' => self.handle_emphasize(),
                '`' => self.handle_line_code(),
                '\n' => {
                    self.iter.next();
                    return Some(LineToken::LF);
                }
                '[' => self.handle_link(),
                _ => self.get_text(),
            }
        } else {
            None
        }
    }
}
