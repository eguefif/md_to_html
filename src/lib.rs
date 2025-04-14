use tokenizer::{Token, Tokenizer};

mod tokenizer;

pub fn transform(content: &str) -> String {
    let mut tokenizer = Tokenizer::new(content);
    let mut html = String::new();

    while let Some(token) = tokenizer.next() {
        match token {
            Token::Title1(value) => html.push_str(&format!("<h1 class=\"md\">{}</h1>", value)),
            Token::Title2(value) => html.push_str(&format!("<h2 class=\"md\">{}</h2>", value)),
            Token::Title3(value) => html.push_str(&format!("<h3 class=\"md\">{}</h3>", value)),
            Token::Title4(value) => html.push_str(&format!("<h4 class=\"md\">{}</h4>", value)),
            _ => {}
        }
    }

    html
}
