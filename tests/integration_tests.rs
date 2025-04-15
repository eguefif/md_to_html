#[test]
fn test_transform() {
    let content = get_md_content();
    let expected = get_html_content();
    let result = md_to_html::transform(content);

    assert_eq!(result.as_str(), expected);
}

fn get_md_content() -> &'static str {
    return r#"

salut les enfants.__Hello le monde__

#test
##test hello world


Hello Paragraph 42

###test42
hello
####test42
_Yoooo de fou_
__*Hello you ici bas__*"#;
}

fn get_html_content() -> &'static str {
    return "\
<p class=\"md\">salut les enfants.<strong>Hello le monde</strong></p>\
<h1 class=\"md\">test</h1>\
<h2 class=\"md\">test hello world</h2>\
<p class=\"md\">Hello Paragraph 42</p>\
<h3 class=\"md\">test42</h3>\
hello<br class=\"md\" />\
<h4 class=\"md\">test42</h4>\
<em>Yoooo de fou</em>\
<em><strong>Hello you ici bas</strong></em>";
}
