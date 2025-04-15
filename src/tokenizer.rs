use std::iter::{Iterator, Peekable};
use std::str::Chars;

pub enum Token {
    Text(String),
    Title1(String),
    Title2(String),
    Title3(String),
    Title4(String),
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

    fn get_text_token(&mut self) -> Option<Token> {
        None
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.iter.next() {
            match next {
                '#' => self.get_title_token(),
                _ => self.get_text_token(),
            }
        } else {
            None
        }
    }
}
