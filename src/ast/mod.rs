pub mod div;

use std::collections::HashMap;
use std::fmt::Debug;

pub trait Element : Debug {
    fn add_child(&mut self, child: Box<Element>);
    fn children(&self) -> &Option<Vec<Box<Element>>>;
    fn add_attributes(&mut self, attributes: HashMap<String, String>);
    fn attributes(&self) -> &Option<HashMap<String, String>>;
    fn set_inner_text(&mut self, text: &str);
    fn inner_text(&self) -> &Option<String>;
    fn is_dirty(&self) -> bool;
    fn to_html(&mut self) -> &str;
}

impl PartialEq for Element {
    fn eq(&self, rhs: &Element) -> bool {
        if self.attributes() == rhs.attributes()
        && self.inner_text() == rhs.inner_text()
        && self.is_dirty() == rhs.is_dirty() 
        && self.children() == rhs.children() {
            true
        } else {
            false
        }
    }
}
