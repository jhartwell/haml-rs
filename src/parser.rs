use ast::div::Div;
use ast::Html;
use values::{TokenValue};
use std::slice::Iter;

pub struct Parser<'a> {
    tokens: &'a Vec<TokenValue>,
    iter: Iter<'a, TokenValue>,
}

impl<'a> Iterator for Parser<'a> {
    type Item = Box<dyn Html>;

    fn next(&mut self) -> Option<Box<dyn Html>> {
        Some(Div::boxed())
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<TokenValue>) -> Parser<'a> {
        Parser {
            tokens,
            iter: tokens.iter(),
        }
    }

    pub fn parse(&mut self) -> Vec<Box<dyn Html>> {
        let mut values = vec![];
        loop {
            if let Some(val) = self.next() {
                values.push(val);
            } else {
                break;
            }
        }
        values
    }
}