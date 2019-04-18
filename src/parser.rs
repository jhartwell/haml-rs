use crate::common::{Element, Html, Text, Token};
use std::any::Any;
use std::slice::Iter;

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
    current: Option<Token>,
}

type StringReturn<'a> = (&'a Token, String);
type HtmlReturn<'a> = (&'a Token, Box<dyn Html>);

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

    fn parse_el(&mut self) -> Option<Box<dyn Html>> {
        let mut current = self.tokens.next();
        match current {
            Some(Token::Whitespace(_)) => None,
            Some(Token::StartAttributes()) => self.parse_attr(),
            _ => None,
        }
    }

    fn parse_key(&mut self) -> (&Token, String) {
        let mut current = self.tokens.next();
        let mut output = String::new();
        loop {
            match current {
                Some(Token::Whitespace(_)) => continue,
                Some(Token::Text(ref key)) => output.push_str(key),
                _ => break,
            }
            current = self.tokens.next();
        }
        (current.unwrap(), output)
    }

    fn parse_value(&mut self) -> StringReturn {
        let mut current = self.tokens.next();
        let mut output = String::new();
        loop {
            match current {
                Some(Token::Whitespace(_)) => continue,
                Some(Token::Text(ref value)) => output.push_str(value),
                _ => break,
            }
        }
        (current.unwrap(), output)
    }

    fn parse_assign(&mut self) -> StringReturn {
        let (current, key) = self.parse_key();
        if current == &Token::Equal() {
            let (current, value) = self.parse_value();
            (current, format!("{} = {}", key, value))
        } else {
            panic!("Oops");
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
            current = self.tokens.next();
        }
        a
    }

    fn parse_element(&mut self, tag: &str, index: u32) -> HtmlReturn {
        let mut t = Element::new(tag);
        let mut current = self.tokens.next();
        let mut current_spaces = index;
        loop {
            match current {
                Some(Token::StartAttributes()) => loop {
                    let (cur, attr) = self.parse_assign();
                    t.add_attributes(&attr);
                    current = Some(cur);
                    if cur == &Token::EndAttributes() {
                        break;
                    }
                },
                Some(Token::Newline()) => {
                    let next = self.tokens.next();
                    match next {
                        Some(Token::Whitespace(spaces)) => {
                            if *spaces > index {
                                current_spaces = *spaces;
                            } else {
                                current = next;
                                break;
                            }
                        }
                    }
                }
                Some(Token::Text(ref text)) => t.add_child(Text::boxed(text.to_string())),
                Some(Token::Element(ref el)) => {
                    let (cur, html) = self.parse_element(el, current_spaces);
                    t.add_child(html);
                    current = Some(cur);
                }
                Some(Token::Class(ref class)) => {
                    let (cur, html) = self.parse_element("div", current_spaces);
                    (*html as Element).add_attributes(format!("class = '{}'", class));
                    t.add_child(html);
                    current = Some(cur);
                }
                Some(Token::Id(ref id)) => {
                    let (cur, html) = self.parse_element("div", current_spaces);
                    t.add_child(html);
                    current = Some(cur);
                }
                _ => break,
            }
        }
        Box::new(t)
    }
}
