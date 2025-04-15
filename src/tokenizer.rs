use std::iter::{Iterator, Peekable};
use std::str::Chars;

const SPECIAL_CHAR: &[char] = &['\n', '_', '*', '#'];

#[derive(Debug)]
pub enum Token {
    Text(String),
    Paragraph(String),
    LineFeed,
    Title1(String),
    Title2(String),
    Title3(String),
    Title4(String),
    Em(String),
    Bold(String),
    EmBold(String),
}

pub struct Tokenizer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            iter: content.chars().peekable(),
        }
    }

    fn get_title_token(&mut self) -> Option<Token> {
        let mut raw_title = String::new();
        loop {
            if let Some(next) = self.iter.next() {
                if next == '\n' {
                    break;
                }
                raw_title.push(next);
            } else {
                break;
            }
        }
        if raw_title.starts_with("###") {
            Some(Token::Title4(raw_title[3..].to_string()))
        } else if raw_title.starts_with("##") {
            Some(Token::Title3(raw_title[2..].to_string()))
        } else if raw_title.starts_with("#") {
            Some(Token::Title2(raw_title[1..].to_string()))
        } else {
            Some(Token::Title1(raw_title))
        }
    }

    fn get_text_token(&mut self, next: char) -> Option<Token> {
        let mut text = String::new();
        text.push(next);
        while let Some(peek) = self.iter.peek() {
            if SPECIAL_CHAR.contains(peek) {
                break;
            }
            text.push(*peek);
            self.iter.next();
        }
        Some(Token::Text(text))
    }

    fn handle_line_feed(&mut self) -> Option<Token> {
        if let Some(peek) = self.iter.peek() {
            if *peek == '\n' {
                self.iter.next();
                self.get_paragraph()
            } else {
                Some(Token::LineFeed)
            }
        } else {
            Some(Token::LineFeed)
        }
    }

    fn get_paragraph(&mut self) -> Option<Token> {
        let mut content = String::new();
        while let Some(next) = self.iter.next() {
            if let Some(peek) = self.iter.peek() {
                if next == '\n' && *peek == '\n' {
                    self.iter.next();
                    break;
                }
            }
            content.push(next);
        }
        Some(Token::Paragraph(content))
    }

    fn handle_emphasize(&mut self) -> Option<Token> {
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
            1 => Some(Token::Em(content)),
            2 => Some(Token::Bold(content)),
            3 => Some(Token::EmBold(content)),
            _ => None,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.iter.next() {
            match next {
                '#' => self.get_title_token(),
                '\n' => self.handle_line_feed(),
                '_' | '*' => self.handle_emphasize(),
                _ => self.get_text_token(next),
            }
        } else {
            None
        }
    }
}
