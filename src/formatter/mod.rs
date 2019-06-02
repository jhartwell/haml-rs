use std::fmt::Debug;

use crate::arena::Arena;
use crate::Format;

pub mod html4_formatter;
pub mod html5_formatter;
pub mod xhtml_formatter;
pub mod xml_formatter;

use html4_formatter::Html4Formatter;
use html5_formatter::Html5Formatter;
use xhtml_formatter::XHtmlFormatter;
use xml_formatter::XmlFormatter;

pub trait HtmlFormatter: Debug {
    fn generate(&self, arena: &Arena) -> String;
}

pub fn get_formatter(format: &Format) -> Box<dyn HtmlFormatter> {
    match format {
        Format::Html4() => Box::new(Html4Formatter::new()),
        Format::Html5() => Box::new(Html5Formatter::new()),
        Format::XHtml() => Box::new(XHtmlFormatter::new()),
        Format::Xml() => Box::new(XmlFormatter::new()),
    }
}
