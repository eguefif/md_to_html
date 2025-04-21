use tokenizer::{Token, Tokenizer};
use transformers::{
    transform_code, transform_list, transform_text, transform_title1, transform_title2,
    transform_title3, transform_title4,
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
                html.push_str(&transform_list(value));
                html.push_str("</ul>");
            }
            Token::Ordered(value) => {
                html.push_str("<ol class=\"md\">");
                html.push_str(&transform_list(value));
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
                    html.push_str(&transform_code(&value));
                }
            }
        }
    }
    html
}
