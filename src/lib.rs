mod ast;
mod common;
mod generator;
mod parser;
mod scanner;
mod values;

use ast::Arena;
use parser::Parser;
use scanner::Scanner;

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

/// Converts the Haml into an abstract syntax tree
/// 
/// ## Example
///
/// ```rust
/// use haml;
/// use haml::ast::Arena;
/// let ast: Arena = haml::to_ast("%span");
/// ```
pub fn to_ast(haml: &str) -> Arena {
    let mut scanner = Scanner::new(haml);
    let tokens = scanner.get_tokens();
    let mut parser = Parser::new(tokens);
    parser.parse().clone()
}
