# MD to HTML converter

This is a simple MD formatted content to HTML converter library.

## Description

Caution, this project does not check user input at all. It is subject to XSS injection attacks. I use this converter in one of my project in a controlled environment. This project should not be used in an environment where user input is not controlled first.

This library epxose a `transform` function that convert a md content into HTML. It adds to each html tags a `md` class.

## Getting Started

Add the github repo to your `Cargo.toml` file.

### Example

```rust
fn main() {
    let content = "# My Title";
    let html = md_to_html::transform(content).unwrap();
    println!("{}", html);
}
```
It will print:
```bash
<h1 class="md">My Title</h1>
```

## Authors

Emmanuel Guefif

## Version History

* 0.1
    * List of supported MD token:
        * Title token: "#"
        * Paragraph
        * Ordered list
        * Unordered list
        * Codes token
        * Quote
        * Emphasize token for bold and italic font weight
        * Urls


## License

This project is licensed under the [MIT] License - see the LICENSE.md file for details
