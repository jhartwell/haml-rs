use std::collections::HashMap;
use ast::Element;

pub struct Span {
    children: Option<Vec<Box<Element>>>,
    inner_text: Option<String>,
    is_dirty: bool,
}

impl Element for Span {

}