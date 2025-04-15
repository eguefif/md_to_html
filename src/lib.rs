use tokenizer::{Token, Tokenizer};

pub mod tokenizer;
pub fn transform(content: &str) -> String {
    let mut html = String::new();
    let mut tokenizer = Tokenizer::new(content);

    while let Some(token) = tokenizer.next() {
        match token {
            Token::Title1(value) => html.push_str(&format!("<h1 class=\"md\">{}</h1>", value)),
            Token::Title2(value) => html.push_str(&format!("<h2 class=\"md\">{}</h2>", value)),
            Token::Title3(value) => html.push_str(&format!("<h3 class=\"md\">{}</h3>", value)),
            Token::Title4(value) => html.push_str(&format!("<h4 class=\"md\">{}</h4>", value)),
            Token::Em(value) => html.push_str(&format!("<em>{}</em>", value)),
            Token::Bold(value) => html.push_str(&format!("<strong>{}</strong>", value)),
            Token::EmBold(value) => html.push_str(&format!("<em><strong>{}</strong></em>", value)),
            Token::LineFeed => html.push_str(&format!("<br class=\"md\" />")),
            Token::Text(value) => html.push_str(&value),
            Token::Paragraph(value) => {
                html.push_str("<p class=\"md\">");
                html.push_str(&transform(&value));
                html.push_str("</p>");
            }
        }
    }
    html
}
