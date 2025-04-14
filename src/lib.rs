use tokenizer::Tokenizer;

mod tokenizer;

pub fn transform(content: &str) -> String {
    let mut tokenizer = Tokenizer::new(content);
    let mut html = String::new();

    html
}
