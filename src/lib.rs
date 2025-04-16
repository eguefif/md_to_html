use line_tokenizer::{LineToken, LineTokenizer};
use tokenizer::{Token, Tokenizer};

pub mod line_tokenizer;
pub mod tokenizer;

pub fn transform(content: &str) -> String {
    let mut html = String::new();
    let mut tokenizer = Tokenizer::new(content);

    while let Some(token) = tokenizer.next() {
        match token {
            Token::Title1(value) => {
                html.push_str(&format!("<h1 class=\"md\">{}</h1>", value.trim()))
            }
            Token::Title2(value) => {
                html.push_str(&format!("<h2 class=\"md\">{}</h2>", value.trim()))
            }
            Token::Title3(value) => {
                html.push_str(&format!("<h3 class=\"md\">{}</h3>", value.trim()))
            }
            Token::Title4(value) => {
                html.push_str(&format!("<h4 class=\"md\">{}</h4>", value.trim()))
            }
            Token::Unordered(value) => {
                html.push_str("<ul class=\"md\">");
                for item in value {
                    html.push_str(&format!("<li class=\"md\">{}</li>", item));
                }
                html.push_str("</ul>");
            }
            Token::Ordered(value) => {
                html.push_str("<ol class=\"md\">");
                for item in value {
                    html.push_str(&format!("<li class=\"md\">{}</li>", item));
                }
                html.push_str("</ol>");
            }
            Token::Paragraph(value) => {
                if value.len() > 0 {
                    html.push_str("<p class=\"md\">");
                    html.push_str(&transform_text(&value));
                    html.push_str("</p>");
                }
            }
        }
    }
    html
}

fn transform_text(content: &str) -> String {
    let mut tokenizer = LineTokenizer::new(content);
    let mut html = String::new();
    while let Some(token) = tokenizer.next() {
        match token {
            LineToken::Em(value) => html.push_str(&format!("<em class=\"md\">{}</em>", value)),
            LineToken::Bold(value) => {
                html.push_str(&format!("<strong class=\"md\">{}</strong>", value))
            }
            LineToken::EmBold(value) => html.push_str(&format!(
                "<em class=\"md\"><strong class=\"md\">{}</strong></em>",
                value
            )),
            LineToken::Text(value) => {
                html.push_str(&value);
            }
            LineToken::LF => html.push_str("<br/>"),
            LineToken::Url(value) => html.push_str(&format!(
                "<a href=\"{}\" class=\"md\">{}</a>",
                value.url, value.a
            )),
        }
    }
    html
}
