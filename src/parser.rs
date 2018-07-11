use ast::{Attribute, Comment, Element, Html, Text};
use std::slice::Iter;
use values::Token;

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
    current: State,
}

enum State {
    BeginElement(),
    EndElement(),
    Element(Box<dyn Html>),
    AttributeId(),
    AttributeValue(),
    Text(),
    Comment(),
    None(),
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            tokens: tokens.iter(),
            current: State::None(),
        }
    }

    pub fn parse(&mut self) -> Vec<Box<dyn Html>> {
        let mut elements : Vec<Box<dyn Html>> = vec![];
        let current_state = State::None();
        loop {
            match self.tokens.next() {
                Some(tok) => {
                    match tok {
                        Token::PercentSign() => {
                            current_state = State::Element(self.parse_element());
                        }
                        Token::Indentation() => {
                            loop {
                                match self.tokens.next() {
                                    Some(Token::Indentation()) => continue,
                                    Some(Token::PercentSign()) => {
                                        if let State::Element(el) = current_state {
                                            el.add_child(self.parse_element());
                                        } else {
                                            panic!("Unexpected token in parsing");
                                        }
                                    },
                                    None => break,
                                    _ => panic!("Unexpected token in parsing"),

                                }
                            }
                        },
                        _ => panic!("Unsupported feature"),
                    }
                }, 
                None => break,
            }
            if let State::Element(ref el) = current_state {
                elements.push();
            }
        }
        elements
    }

    fn parse_element(&mut self) -> Box<dyn Html> {
        let mut tag: &str = "";
        let mut id: &str = "";
        let mut value: &str = "";
        let mut attributes = vec![];
        let mut current_state = State::BeginElement();
        loop {
            if let Some(tok) = self.tokens.next() {
                match tok {
                    Token::Text(txt) => {
                        match current_state {
                            State::BeginElement() => {
                                tag = txt;
                                current_state = State::Element(Element::boxed(tag.to_string()));
                            },
                            State::AttributeId() => {
                                id = txt;
                                current_state = State::AttributeId();
                            },
                            State::AttributeValue() => {
                                value = txt;
                                attributes.push(
                                    Attribute::new(id.to_string(), value.to_string())
                                );
                                id = "";
                                value = "";
                                current_state = State::AttributeValue();
                            },
                            _ => panic!("Unexpected value in element"),
                        }
                    },
                    Token::OpenParen() => {
                        match current_state {
                            State::BeginElement() => panic!("Invalid state when parsing element"),
                            State::Element(_el) => current_state = State::AttributeId(),
                            _ => panic!("Invalid state when parsing element"),
                        }
                    },
                    Token::CloseParen() => {
                        match current_state {
                            State::AttributeValue() => break,
                            _ => panic!("Invalid state when parsing element"),
                        }
                    }
                    _ => break,
                }
            }
        }
        if let State::Element(el) = current_state {
            if attributes.len() > 0 {
                
            }
        } else {
            panic!("Invalid state when parsing element");
        }
            
        
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use scanner::Scanner;
    use values::Token;

    #[test]
    fn test_basic_element() {
        let haml = "%div";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();
        assert_eq!(1, elements.len());
        assert_eq!(
            &Some("div".to_string()),
            elements.iter().nth(0).unwrap().tag()
        );
    }

    #[test]
    fn test_basic_element_with_attributes() {
        let haml = "%div( id=\"test\")";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();
        let actual_element = elements.iter().nth(0).unwrap();
        assert_eq!(1, elements.len());
        assert_eq!(&Some("div".to_string()), actual_element.tag());
        assert_eq!(1, actual_element.attributes().len());
        let actual_attribute = actual_element.attributes().iter().nth(0).unwrap();
        assert_eq!("id", actual_attribute.key());
        assert_eq!("test", actual_attribute.value());
    }

}
