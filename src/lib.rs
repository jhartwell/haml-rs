#![allow(dead_code)]
mod generator;
mod parser;
mod regex;

use generator::Generator;
use parser::Parser;

pub fn to_html(haml: &str) -> String {
    let mut parser = Parser::new();
    let ast = parser.parse(haml);
    let generator = Generator::new(&ast);
    generator.to_html()
}
