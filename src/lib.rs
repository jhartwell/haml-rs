mod ast;
mod common;
mod generator;
mod parser;
mod scanner;
mod values;

use parser::Parser;
use scanner::Scanner;
use values::Token;

/// Converts the Haml that is contained in a reference string
/// into an owned string.
///
/// ## Example
///
/// ```rust
/// use haml;
/// let html: String = haml::to_html("%span");
/// ```
pub fn to_html(haml: &str) -> String {
    let mut scanner = Scanner::new(haml);
    let tokens = scanner.get_tokens();
    let mut parser = Parser::new(tokens);
    let parsed_values = parser.parse();
    generator::to_html(&parsed_values)
}

pub fn tokenize<'a>(haml: &'a str) -> Vec<Token> {
    let mut scanner = Scanner::new(haml);
    scanner.get_tokens().clone()
}
