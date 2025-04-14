use tokenizer::{Token, Tokenizer};

mod tokenizer;

pub fn transform(content: &str) -> String {
    let mut tokenizer = Tokenizer::new(content);
    let mut html = String::new();

    for token in tokenizer.next() {
        match token {
            Token::Title1(value) => html.push_str(&format!("<h1>{}</h1>", value)),
            _ => {}
        }
    }

    html
}
