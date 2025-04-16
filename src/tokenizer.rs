use std::iter::{Iterator, Peekable};
use std::str::Chars;

const SPECIAL_LINE_CHAR: &[char] = &['-', '*', '#', '+', '1'];

#[derive(Debug)]
pub enum Token {
    Paragraph(String),
    Title1(String),
    Title2(String),
    Title3(String),
    Title4(String),
    Unordered(Vec<String>),
    Ordered(Vec<String>),
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
        self.iter.next();
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

    fn get_paragraph(&mut self) -> Option<Token> {
        let mut paragraph = String::new();
        loop {
            let line = self.get_next_line();
            if line.len() == 0 {
                break;
            }
            paragraph.push_str(&line);
            if self.is_special_char() || self.is_linefeed() {
                break;
            }
            paragraph.push('\n');
        }
        Some(Token::Paragraph(paragraph))
    }

    fn is_special_char(&mut self) -> bool {
        if let Some(peek) = self.iter.peek() {
            if SPECIAL_LINE_CHAR.contains(peek) {
                return true;
            }
        }
        false
    }

    fn is_linefeed(&mut self) -> bool {
        if let Some('\n') = self.iter.peek() {
            self.iter.next();
            return true;
        }
        false
    }

    fn get_next_line(&mut self) -> String {
        let mut line = String::new();
        while let Some(next) = self.iter.next() {
            if next == '\n' {
                break;
            }
            line.push(next);
        }
        line
    }

    fn get_special_line(&mut self) -> Option<Token> {
        if let Some(peek) = self.iter.peek() {
            match peek {
                '#' => self.get_title_token(),
                '-' | '*' | '+' => self.get_unordered_list(),
                '1' => self.get_ordered_list(),
                _ => None,
            }
        } else {
            None
        }
    }

    fn get_unordered_list(&mut self) -> Option<Token> {
        let mut list = Vec::new();
        loop {
            let item = self.get_next_line();
            if item.len() > 1 {
                list.push(item[2..].to_string());
            } else {
                list.push(String::new())
            }
            if self.is_unordered_list_end() {
                break;
            }
        }

        Some(Token::Unordered(list))
    }
    fn is_unordered_list_end(&mut self) -> bool {
        if let Some(peek) = self.iter.peek() {
            if ['-', '*', '+'].contains(peek) {
                return false;
            }
        }
        true
    }

    fn get_ordered_list(&mut self) -> Option<Token> {
        let mut list = Vec::new();
        loop {
            let item = self.get_next_line();
            if item.len() > 1 {
                let item = self.skip_ordered_head(&item);
                list.push(item);
            } else {
                list.push(String::new())
            }
            if self.is_ordered_list_end() {
                break;
            }
        }

        Some(Token::Ordered(list))
    }

    fn is_ordered_list_end(&mut self) -> bool {
        if let Some(peek) = self.iter.peek() {
            if peek.is_digit(10) {
                return false;
            }
        }
        true
    }

    fn skip_ordered_head(&mut self, item: &str) -> String {
        let mut item_iter = item.chars().peekable();
        while let Some(peek) = item_iter.peek() {
            if !peek.is_digit(10) {
                break;
            }
            item_iter.next();
        }
        item_iter.next();
        item_iter.next();
        item_iter.collect::<String>()
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(peek) = self.iter.peek() {
            if !SPECIAL_LINE_CHAR.contains(peek) {
                return self.get_paragraph();
            } else {
                return self.get_special_line();
            }
        }
        None
    }
}
