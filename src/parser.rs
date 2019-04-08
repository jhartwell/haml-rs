use crate::common::{Element, Html, Text, Token};
use std::slice::Iter;

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
    current: Option<Token>,
}

// Element(String),
// ImpliedDiv(),
// StartAttributes(),
// EndAttributes(),
// Class(String),
// Id(String),
// Whitespace(u32),
// Text(String),
// Newline(),
// Equal(),
// Quoted(),
// Arrow(),
// Slash(),
// Colon(),

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            tokens: tokens.iter(),
            current: None,
        }
    }

    pub fn parse(&mut self) -> Vec<Box<dyn Html>> {
        let a: Vec<Box<Html>> = vec![];
        let mut current = self.tokens.next();
        let mut previous: Option<Token> = None;
        let mut current_element: Option<Box<dyn Html>> = None;
        loop {
            match current {
                Some(Token::Element(ref t)) => {
                    current_element = Some(Box::new(self.parse_element(t)));
                }
                Some(Token::ImpliedDiv()) => {
                    current_element = Some(Box::new(self.parse_element("div")));
                }
                Some(Token::Newline()) => break,
                _ => break,
            }
        }
        a
    }

    fn parse_attributes(&mut self) -> String {
        let mut current = self.tokens.next();
        let mut attr = String::new();
        loop {
            match current {
                Some(Token::Class(class)) => {
                    while let Some(t) = self.tokens.next() {

                    }
                },
                Some(Token::Id(id)) => attr.push_str(&id),
                Some(Token::EndAttributes()) => break,
                _ => break,
            }
        }
        attr
    }

    fn parse_attribute_value(&mut self, value: &str) -> String {
        let mut attr = String::new();
        let mut current = self.tokens.next();
        loop {
            match current {
                Some(Token::Equal()) => {
                    while let Some(t) = self.tokens.next() {
                        
                    }
                    break;
                }
                Some(Token::EndAttributes()) => break,
                _ => panic!("Error"),
            }
        }
        attr
    }

    fn parse_element(&mut self, tag: &str) -> impl Html {
        let mut t = Element::new(tag);
        let mut current = self.tokens.next();
        match current {
            Some(Token::StartAttributes()) => (), //self.parse_attributes(),
            _ => (),
        }
        t
    }
}
