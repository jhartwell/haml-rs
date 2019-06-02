use crate::arena::{Arena, ArenaItem};
use crate::formatter::HtmlFormatter;
use crate::parser::Haml;

#[derive(Debug)]
pub struct XmlFormatter;

impl XmlFormatter {
    pub fn new() -> XmlFormatter {
        XmlFormatter {}
    }
}

impl HtmlFormatter for XmlFormatter {
    fn generate(&self, arena: &Arena) -> String {
        String::new()
    }
}
