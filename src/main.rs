use std::io::Write;

fn main() {
    let content = get_content();
    let html = md_to_html::transform(&content);
    write_html(html);
}

fn get_content() -> String {
    return std::fs::read_to_string("article.md").unwrap();
}

fn write_html(html: String) {
    let content = format!(
        "
    <html>
<head>
<link rel=\"stylesheet\" href=\"/home/eguefif/lab/career_manager/html/website/dev/css/styles.css\">
</head>
    <body>

            div class=\"content\" style=\"margin: auto\">
<div class=\"article\">
    <div class=\"article-header\">
        <h1>Websocket example</h1>
        <div class=\"article-date\">
            Saturday 5,
        </div>
    </div>
    <div class=\"article-content\">
{}
    </div>
</div>
            </div>
        </body>
    </html>
    ",
        html
    );
    let mut file = std::fs::File::create("index.html").unwrap();
    let _ = file.write_all(content.as_bytes());
}
