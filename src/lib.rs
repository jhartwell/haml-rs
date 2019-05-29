#![allow(dead_code)]
mod generator;
mod parser;
mod regex;

use generator::Generator;
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
    let generator = Generator::new(&ast);
    generator.to_html()
}