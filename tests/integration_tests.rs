#[test]
fn test_transform() {
    let content = get_md_content();
    let expected = get_html_content();
    let result = md_to_html::transform(content);

    assert_eq!(result.as_str(), expected);
}

fn get_md_content() -> &'static str {
    return r#"salut les enfants.
Hello

Hello World in second paragraph
Hello World in second paragraph
Hello World in second paragraph
# Hello World
## Hello World
### Hello World
#### Hello World
"#;
}

fn get_html_content() -> &'static str {
    return "\
<p class=\"md\">salut les enfants.<br/>Hello</p>\
<p class=\"md\">\
Hello World in second paragraph<br/>\
Hello World in second paragraph<br/>\
Hello World in second paragraph\
</p>\
<h1 class=\"md\">Hello World</h1>\
<h2 class=\"md\">Hello World</h2>\
<h3 class=\"md\">Hello World</h3>\
<h4 class=\"md\">Hello World</h4>\
";
}
