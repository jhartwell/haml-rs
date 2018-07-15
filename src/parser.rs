use ast::{Attribute, Comment, Element, Html, Text};
use std::slice::Iter;
use values::Token;
use std::fmt;

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
    current: State,
    elements: Vec<Box<dyn Html>>,
}

#[derive(Debug)]
enum State {
    BeginElement(),
    EndElement(),
    StartIdDiv(),
    StartClassDiv(),
    Element(Box<dyn Html>),
    NewAttribute(),
    AttributeId(),
    AttributeAssign(),
    AttributeValue(),
    Children(),
    Text(),
    Comment(),
    NewLine(),
    None(),
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            State::BeginElement() => "BeginElement",
            State::EndElement() => "EndElement",
            State::StartIdDiv() => "StartIdDiv",
            State::StartClassDiv() => "StartClassDiv",
            State::Element(_el) => "Element",
            State::NewAttribute() => "New Attribute",
            State::AttributeId() => "AttributeId",
            State::AttributeAssign() => "AttributeAssign",
            State::AttributeValue() => "AttributeValue",
            State::Children() => "Children",
            State::Text() => "Text",
            State::Comment() => "Comment",
            State::NewLine() => "NewLine",
            State::None() => "None",
        };
        write!(f, "{}", output)
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            tokens: tokens.iter(),
            current: State::None(),
            elements: vec![],
        }
    }

    pub fn parse(&mut self) -> &Vec<Box<dyn Html>> {
        if self.elements.len() > 0 {
            return &self.elements;
        }
        let mut current_state = State::None();
        loop {
            match self.tokens.next() {
                Some(tok) => match tok {
                    Token::Backslash() => current_state = State::Comment(),
                    Token::Text(txt) => {
                        match current_state {
                            State::Comment() => self.elements.push(Comment::boxed(txt.to_string())),
                            _ => {
                                current_state = State::Text();
                                self.elements.push(Text::boxed(txt.to_string()));
                            },
                        }
                    },
                    Token::PercentSign() => {
                        let el = self.parse_element();
                        current_state = State::Element(el);
                    },
                    Token::Period() => {
                        let el = self.parse_div(State::StartClassDiv());
                        current_state = State::Element(el);
                    },
                    Token::Hashtag() => {
                        let el = self.parse_div(State::StartIdDiv());
                        current_state = State::Element(el);
                    },
                    Token::Indentation() => loop {
                        match self.tokens.next() {
                            Some(Token::Indentation()) => continue,
                            Some(Token::PercentSign()) => {
                                if let State::Element(ref mut el) = current_state {
                                    el.add_child(self.parse_element());
                                } else {
                                    panic!("Unexpected token in parsing");
                                }
                            }
                            None => break,
                            _ => panic!("Unexpected token in parsing"),
                        }
                    },
                    Token::Whitespace() => continue,
                    t => panic!(format!("{}: Unsupported feature", t.to_string())),
                },
                None => {
                    if let State::Element(el) = current_state {
                        self.elements.push(el);
                    }
                    break;
                }
            }
        }
        &self.elements
    }

    fn parse_div(&mut self, state: State) -> Box<dyn Html> {
        
        let mut div = Element::boxed("div".to_string());
        match state {
            State::StartClassDiv() => {
                if let Some(Token::Text(txt)) = self.tokens.next() {
                    div.add_attribute(Attribute::new("class".to_string(), txt.to_string()));
                } else {
                    panic!("Trying to parse div in class form (start with period) but couldn't find valid text for class name.");
                }
            },
            State::StartIdDiv() => {
                if let Some(Token::Text(txt)) = self.tokens.next() {
                    div.add_attribute(Attribute::new("id".to_string(), txt.to_string()));
                } else {
                    panic!("Trying to parse div in ID form (start with #) but couldn't find valid text for class name.");
                }
            },
            _ => panic!("Invalid state for parsing straight div element"),
        }
        div
    }
    fn parse_element(&mut self) -> Box<dyn Html> {
        let mut id: &str = "";
        let mut attributes = vec![];
        let mut children : Vec<Box<dyn Html>> = vec![];
        let mut current_state = State::BeginElement();

        let mut element: Option<Box<dyn Html>> = None;
        loop {
            if let Some(tok) = self.tokens.next() {
                match tok {
                    Token::Text(txt) => match current_state {
                        State::BeginElement() => current_state = State::Element(Element::boxed(txt.to_string())),
                        State::NewAttribute() => {
                            id = txt;
                            current_state = State::AttributeId();
                        },
                        State::AttributeAssign() => {
                            &attributes.push(Attribute::new(id.to_string(), txt.to_string()));
                            id = "";
                            current_state = State::AttributeValue();
                        },
                        State::Children() => {
                            &children.push(Text::boxed(txt.to_string()));
                        },
                        State::Element(el) => {
                            current_state = State::Children();
                            element = Some(el);
                            &children.push(Text::boxed(txt.to_string()));
                        },
                        _ => panic!("Unexpected value in element"),
                    },
                    Token::OpenParen() => match current_state {
                        State::BeginElement() => panic!("Invalid state when parsing element"),
                        State::Element(el) => {
                            element = Some(el);
                            current_state = State::NewAttribute();
                        }
                        _ => panic!("Invalid token \"(\" when parsing element"),
                    },
                    Token::CloseParen() => match current_state {
                        State::AttributeValue() => current_state = State::Children(),
                        _ => panic!("Invalid token \")\" when parsing element"),
                    },
                    Token::Whitespace() => {
                        match current_state {
                            State::Element(el) => {
                                element = Some(el);
                                current_state = State::Children();
                            },
                            State::AttributeValue() => current_state = State::NewAttribute(),
                            _ => continue,
                        }
                    },
                    Token::Equal() => match current_state {
                        State::AttributeId() => current_state = State::AttributeAssign(),
                        _ => panic!("Invalid token \"=\" when parsing element"),
                    },
                    Token::EndLine() => break,
                    _ => break,
                }
            } else {
                break;
            }
        }
        if let State::Element(el) = current_state {
            element = Some(el);
        }

        if let Some(ref mut el) = element {
            el.add_attributes(attributes);
            el.add_children(children);
        }

        match element {
            Some(el) => el,
            None => panic!("No elements were found when parsing element"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use scanner::Scanner;

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
    fn test_basic_element_with_single_attribute() {
        let haml = "%div( id=\"test\")";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();

        assert_eq!(1, elements.len());
        let actual_element = elements.iter().nth(0).unwrap();
        assert_eq!(&Some("div".to_string()), actual_element.tag());
        assert_eq!(1, actual_element.attributes().len());
        let actual_attribute = actual_element.attributes().iter().nth(0).unwrap();
        assert_eq!("id", actual_attribute.key());
        assert_eq!("test", actual_attribute.value());
    }

    #[test]
    fn test_basic_element_with_multiple_attributes() {
        let haml = "%div( id=\"test\" class=\"container\")";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();

        assert_eq!(1, elements.len());
        let actual_element = elements.iter().nth(0).unwrap();
        assert_eq!(&Some("div".to_string()), actual_element.tag());
        assert_eq!(2, actual_element.attributes().len());
        let actual_attribute_first = actual_element.attributes().iter().nth(0).unwrap();
        assert_eq!("id", actual_attribute_first.key());
        assert_eq!("test", actual_attribute_first.value());

        let actual_attribute_second = actual_element.attributes().iter().nth(1).unwrap();
        assert_eq!("class", actual_attribute_second.key());
        assert_eq!("container", actual_attribute_second.value());        
    }

    #[test]
    fn test_element_with_same_line_child() {
        let haml = "%div text";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();

        assert_eq!(1, elements.len());
        let actual_element = elements.iter().nth(0).unwrap();
        assert_eq!(0, actual_element.attributes().len());
        assert_eq!(1, actual_element.children().len());
    }

    #[test]
    fn test_element_with_attributes_and_same_line_child() {
        let haml = "%span(class=\"container\") text";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();

        assert_eq!(1, elements.len());
        let actual_element = elements.iter().nth(0).unwrap();
        assert_eq!(1, actual_element.attributes().len());
        assert_eq!(1, actual_element.children().len());
    }

    #[test]
    fn test_element_with_newline_child() {
        let haml = "%div\n  %span";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();

        assert_eq!(1, elements.len());
        let actual_element = elements.iter().nth(0).unwrap();
        assert_eq!(0, actual_element.attributes().len());
        assert_eq!(1, actual_element.children().len());
    }

    #[test]
    fn test_basic_text() {
        let haml = "text";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
       
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();

        assert_eq!(1, elements.len());
        let actual_element = elements.iter().nth(0).unwrap();
        assert_eq!(&None, actual_element.tag());
        assert_eq!("text", actual_element.to_html());

    }

    #[test]
    fn test_basic_comment() {
        let haml = "\\ comment";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();
        assert_eq!(1, elements.len());
        let actual_element = elements.iter().nth(0).unwrap();
        assert_eq!(&None, actual_element.tag());
        assert_eq!("<!-- comment -->", actual_element.to_html());
    }

    #[test]
    fn test_basic_class_div() {
        let haml = ".container";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();
        assert_eq!(1, elements.len());

        let actual_element = elements.iter().nth(0).unwrap();
        assert_eq!(&Some("div".to_string()), actual_element.tag());
        assert_eq!(1, actual_element.attributes().len());
        let actual_attribute = actual_element.attributes().iter().nth(0).unwrap();
        assert_eq!("class", actual_attribute.key());
        assert_eq!("container", actual_attribute.value());
    }

    #[test]
    fn test_basic_id_div() {
        let haml = "#test";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();
        assert_eq!(1, elements.len());

        let actual_element = elements.iter().nth(0).unwrap();
        assert_eq!(&Some("div".to_string()), actual_element.tag());
        assert_eq!(1, actual_element.attributes().len());
        let actual_attribute = actual_element.attributes().iter().nth(0).unwrap();
        assert_eq!("id", actual_attribute.key());
        assert_eq!("test", actual_attribute.value());
    }

}
