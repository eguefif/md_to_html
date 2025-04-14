#[test]
fn test_transform() {
    let content = get_md_content();
    let result = md_to_html::transform(&content);
    let expect = get_html_content();

    assert_eq!(result.as_str(), expect);
}

fn get_md_content() -> &'static str {
    return "\
#test\
    ";
}

fn get_html_content() -> &'static str {
    return "\
<h1>test</h1>\
    ";
}
