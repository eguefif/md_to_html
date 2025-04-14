#[test]
fn test_transform() {
    let content = get_md_content();
    let result = md_to_html::transform(&content);
    let expect = get_html_content();

    assert_eq!(result.as_str(), expect);
}

fn get_md_content() -> &'static str {
    return "\
#test
##test hello world
###test42
####test42
    ";
}

fn get_html_content() -> &'static str {
    return "\
<h1 class=\"md\">test</h1>\
<h2 class=\"md\">test hello world</h2>\
<h3 class=\"md\">test42</h3>\
<h4 class=\"md\">test42</h4>\
    ";
}
