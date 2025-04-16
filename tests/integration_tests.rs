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

Hello _World_ in second paragraph
Hello **World** in second paragraph
Hello *__World__* in second paragraph
# Hello World
## Hello World
### Hello World
#### Hello World
* Item1
* Item2
+ Item3
- Item4
"#;
}

fn get_html_content() -> &'static str {
    return "\
<p class=\"md\">salut les enfants.<br/>Hello</p>\
<p class=\"md\">\
Hello <em class=\"md\">World</em> in second paragraph<br/>\
Hello <strong class=\"md\">World</strong> in second paragraph<br/>\
Hello <em class=\"md\"><strong class=\"md\">World</strong></em> in second paragraph\
</p>\
<h1 class=\"md\">Hello World</h1>\
<h2 class=\"md\">Hello World</h2>\
<h3 class=\"md\">Hello World</h3>\
<h4 class=\"md\">Hello World</h4>\
<ul class=\"md\">\
<li class=\"md\">Item1</li>\
<li class=\"md\">Item2</li>\
<li class=\"md\">Item3</li>\
<li class=\"md\">Item4</li>\
</ul>\
";
}
