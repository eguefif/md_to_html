use std::iter::{Iterator, Peekable};
use std::str::Chars;

pub enum Token {
    Text(String),
    Title1(String),
    Title2(String),
    Title3(String),
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
        match &raw_title[..3] {
            "###" => Some(Token::Title3(raw_title[..3].to_string())),
            "##" => Some(Token::Title2(raw_title[..3].to_string())),
            "#" => Some(Token::Title1(raw_title[..3].to_string())),
            _ => None,
        }
    }

    fn get_text_token(&mut self) -> Option<Token> {
        None
    }
}

impl<'_> Iterator for Tokenizer<'_> {
    type Item = Token;
    pub fn next(&mut self) -> Option<Self::Item> {
        if let Ok(next) = self.iter.next() {
            match next {
                '#' => self.get_title_token(),
                _ => self.get_text_token(),
            }
        } else {
            None
        }
    }
}
