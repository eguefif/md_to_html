use crate::transform_text;

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

pub fn transform_unordered(content: Vec<String>) -> String {
    let mut retval = String::new();
    for item in content {
        let item = transform_text(&item);
        retval.push_str(&format!("<li class=\"md\">{}</li>", item));
    }

    retval
}
