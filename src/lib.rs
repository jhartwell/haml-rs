#![allow(dead_code)]
// mod arena;
// mod formatter;
// mod parser;
// mod regex;
// use formatter::HtmlFormatter;
// use parser::Parser;
mod lex;
mod parse;

use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
pub enum Format {
    Html4(),
    Html5(),
    Xml(),
    XHtml(),
}

pub fn to_html(haml: &str, format: &Format) -> String {
    // let mut parser = Parser::new(format);
    // let ast = parser.parse(haml);
    // let generator = formatter::get_formatter(format);
    // generator.generate(&ast)
    String::new()
}

use std::fmt;

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut format = "html5";
        match self {
            Format::XHtml() => format = "xhtml",
            Format::Html4() => format = "html4",
            Format::Html5() => format = "html5",
            Format::Xml() => format = "xml",
        }
        write!(f, "{}", format)
    }
}


#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Whitespace(),
    Text(String),
    OpenParen(),
    CloseParen(),
    OpenBrace(),
    CloseBrace(),
    PercentageSign(),
    Period(),
    Equal(),
    SingleQuote(),
    DoubleQuote(),
    ForwardSlash(),
    BackSlash(),
    Hashtag(),
    LessThan(),
    GreaterThan(),
    Exclamation(),
    Ampersand(),
    Tilde(),
    Newline(),
}

pub trait Haml {}

pub(crate) struct Declaration {
    value: String,
}

impl Declaration {
    pub fn new(value: &str) -> Declaration {
        Declaration {
            value: value.to_string(),
        }
    }
}

impl Haml for Declaration {}

pub(crate) struct Element {
    name: String,
    attributes: HashMap<String, Vec<String>>,
    attribute_order: BTreeSet<String>,
}

impl Element {
    pub fn new(name: &str) -> Element {
        Element {
            name: name.to_string(),
            attributes: HashMap::new(),
            attribute_order: BTreeSet::new(),
        }
    }
}
impl Haml for Element {}