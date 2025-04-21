use line_tokenizer::{LineToken, LineTokenizer};
use tokenizer::{Token, Tokenizer};
use transformers::{
    transform_title1, transform_title2, transform_title3, transform_title4, transform_unordered,
};

pub mod line_tokenizer;
pub mod tokenizer;
pub mod transformers;

pub fn transform(content: &str) -> String {
    let mut html = String::new();
    let mut tokenizer = Tokenizer::new(content);

    while let Some(token) = tokenizer.next() {
        match token {
            Token::Title1(value) => html.push_str(&transform_title1(&value)),
            Token::Title2(value) => html.push_str(&transform_title2(&value)),
            Token::Title3(value) => html.push_str(&transform_title3(&value)),
            Token::Title4(value) => html.push_str(&transform_title4(&value)),
            Token::Unordered(value) => {
                html.push_str("<ul class=\"md\">");
                html.push_str(&transform_unordered(value));
                html.push_str("</ul>");
            }
            Token::Ordered(value) => {
                html.push_str("<ol class=\"md\">");
                for item in value {
                    let item = transform_text(&item);
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
            Token::Quote(value) => {
                if value.len() > 0 {
                    html.push_str("<div class=\"md quote\">");
                    html.push_str(&transform_text(&value));
                    html.push_str("</div>");
                }
            }
            Token::Code((_, value)) => {
                if value.len() > 0 {
                    html.push_str(&format!("<code class=\"md\">{}</code>", value));
                }
            }
        }
    }
    html
}

pub fn transform_text(content: &str) -> String {
    println!("CONTENT: {}", content);
    let mut tokenizer = LineTokenizer::new(content);
    let mut html = String::new();
    while let Some(token) = tokenizer.next() {
        match token {
            LineToken::Code(value) => {
                html.push_str(&format!("<span class=\"md line-code\">{}</span>", value))
            }
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
