mod ast;
mod common;
mod generator;
mod parser;
mod scanner;
mod values;

use ast::ToAst;
use parser::Parser;
use scanner::Scanner;

/// Determines what type of output to generate
/// *  XHtml will generate valid xhtml
/// *  Html5 will generate valid html based on HTML5 spec
/// *  Html4 will generate valid html based on HTML4 spec
#[derive(Clone, Debug)]
pub enum HtmlFormat {
    XHtml(),
    Html5(),
    Html4(),
}

/// Converts the Haml that is contained in a reference string
/// into an owned string.
///
/// # Arguments
/// *  `haml` - A string slice that contains the Haml to be converted to HTML
/// *  `format` - A HtmlFormat enum that determines what kind of Html output
/// will be generated
///
/// ## Examples
///
/// ### Converting Haml to Html5
/// ```rust
/// use haml;
/// let html: String = haml::to_html("%span", Html5());
/// ```
///
/// ### Converting Haml to Html4
/// ```rust
/// use haml;
/// let html: String = haml::to_html("%span", Html4());
/// ```
///
/// ### Converting Haml to xhtml
/// ```rust
/// use haml;
/// let html: String = haml::to_html("%span", XHtml());
/// ```
pub fn to_html(haml: &str, format: HtmlFormat) -> String {
    let mut scanner = Scanner::new(haml);
    let tokens = scanner.get_tokens();
    let mut parser = Parser::new(tokens, format);
    let parsed_values = parser.parse();
    generator::to_html(&parsed_values)
}

/// Converts the Haml into an abstract syntax tree
///
/// # Arguments
/// *  `haml` - A string slice that contains the Haml to be parsed
/// *  `format` - A HtmlFormat enum to determine what type of HTML
/// output is generated
///
/// ## Examples
///
/// ### Creating an AST for HTML5
/// ```rust
/// use haml;
/// let ast = haml::to_ast("%span", Html5());
/// ```
///
/// ### Creating an AST for HTML4
/// ```rust
/// use haml;
/// let ast = haml::to_ast("%span", Html4());
/// ```
///
/// ### Creating an AST for XHTML
/// ```rust
/// use haml;
/// let ast = haml::to_ast("%span", XHtml());
/// ```
pub fn to_ast(haml: &str, format: HtmlFormat) -> String {
    let mut scanner = Scanner::new(haml);
    let tokens = scanner.get_tokens();
    let mut parser = Parser::new(tokens, format);
    parser.parse().to_ast()
}
