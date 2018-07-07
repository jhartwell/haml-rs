use values::{Token, TokenValue};
use std::slice::Iter;
use ast::{HtmlElement, Element};

pub struct Parser<'a> {
    tokens: &'a Vec<TokenValue>,
    iter: Iter<'a, TokenValue>,
    current_token: Option<TokenValue>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<TokenValue>) -> Parser<'a> {
        Parser {
            tokens,
            iter: tokens.into_iter(),
            current_token: None,
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = HtmlElement;

    fn next(&mut self) -> Option<HtmlElement> {
        Some(HtmlElement::Element(
            Element::new("test".to_string())
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
}