#![allow(dead_code)]
mod arena;
mod formatter;
mod parser;
mod regex;
use formatter::HtmlFormatter;
use parser::Parser;

#[derive(Debug)]
pub enum Format {
    Html4(),
    Html5(),
    Xml(),
    XHtml(),
}

pub fn to_html(haml: &str, format: &Format) -> String {
    let mut parser = Parser::new(format);
    let ast = parser.parse(haml);
    let generator = formatter::get_formatter(format);
    generator.generate(&ast)
}
