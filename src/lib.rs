#![allow(dead_code)]
mod parser;
mod regex;

use parser::Parser;

pub enum Format {
    Html4(),
    Html5(),
    Xml(),
    XHtml(),
}

pub fn to_html(haml: &str, format: &Format) -> String {
    let mut parser = Parser::new();
    let ast = parser.parse(haml, format);
    ast.to_html()
}