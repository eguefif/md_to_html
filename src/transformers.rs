use crate::line_tokenizer::{LineToken, LineTokenizer};

pub fn transform_title1(entry: &str) -> String {
    format!("<h1 class=\"md\">{}</h1>", entry.trim())
}

pub fn transform_title2(entry: &str) -> String {
    format!("<h2 class=\"md\">{}</h2>", entry.trim())
}

pub fn transform_title3(entry: &str) -> String {
    format!("<h3 class=\"md\">{}</h3>", entry.trim())
}

pub fn transform_title4(entry: &str) -> String {
    format!("<h4 class=\"md\">{}</h4>", entry.trim())
}

pub fn transform_list(content: Vec<String>) -> String {
    let mut retval = String::new();
    for item in content {
        let item = transform_text(&item);
        retval.push_str(&format!("<li class=\"md\">{}</li>", item));
    }

    retval
}

pub fn transform_code(content: &str) -> String {
    format!("<code class=\"md\">{}</code>", content)
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
