#[test]
fn test_code_snippet() {
    let content = get_md_code();
    let expected = get_html_code();
    let result = md_to_html::transform(content);

    assert_eq!(result.as_str(), expected);
}

fn get_md_code() -> &'static str {
    return "\
```
let a = 5;
function lambda(x: String) {
        println!(\"this is some code\");
        return x;
}
let b = lambda(5) + a;
```\
";
}

fn get_html_code() -> &'static str {
    return "\
<code class=\"md\">let a = 5;
function lambda(x: String) {
        println!(\"this is some code\");
        return x;
}
let b = lambda(5) + a;
</code>\
";
}

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

1. Item1
1. Item2
3. Item3
5. Item4
[Link to](http://www.fakewebsite.com)
You can click on [Link to](http://www.fakewebsite.com)
1. Link: [test](http://test.com)
1. Link2: [test2](http://test2.com)
>Hello, I'm a quote
>That goes over two lines
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
<ol class=\"md\">\
<li class=\"md\">Item1</li>\
<li class=\"md\">Item2</li>\
<li class=\"md\">Item3</li>\
<li class=\"md\">Item4</li>\
</ol>\
<p class=\"md\"><a href=\"http://www.fakewebsite.com\" class=\"md\">Link to</a><br/>\
You can click on <a href=\"http://www.fakewebsite.com\" class=\"md\">Link to</a></p>\
<ol class=\"md\">\
<li class=\"md\">Link: <a href=\"http://test.com\" class=\"md\">test</a></li>\
<li class=\"md\">Link2: <a href=\"http://test2.com\" class=\"md\">test2</a></li>\
</ol>\
<div class=\"md quote\">Hello, I'm a quote<br/>That goes over two lines</div>\
";
}
