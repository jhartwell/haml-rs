use ast::{Attribute, Comment, Element, Html, Text};
use std::slice::Iter;
use values::Token;
use std::fmt;

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
}


#[derive(Debug, PartialEq, Clone)]
enum AttributeState {
    Start(),
    Id(String),
    Value(),
    End(),
}

#[derive(Debug)]
enum State {
    BeginElement(),
    EndElement(),
    StartIdDiv(),
    StartClassDiv(),
    AttributeState(AttributeState),
    Element(),
    Attributes(Vec<Attribute>),
    Children(Vec<Box<dyn Html>>),
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
            State::Element() => "Element",
            State::Attributes(_attrs) => "Attributes",
            State::Children(_chiildren) => "Children",
            State::Text() => "Text",
            State::Comment() => "Comment",
            State::NewLine() => "NewLine",
            State::AttributeState(_as) => "AttributeState",
            State::None() => "None",
        };
        write!(f, "{}", output)
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            tokens: tokens.iter(),
        }
    }

    pub fn parse(&mut self) -> Vec<Box<dyn Html>> {
        let mut current_state = State::None();
        let mut elements: Vec<Box<dyn Html>> = vec![];
        let mut element: Option<Box<dyn Html>> = None;
        loop {
            match self.tokens.next() {
                Some(tok) => match tok {
                    Token::Backslash() => current_state = State::Comment(),
                    Token::EndLine() => current_state = State::NewLine(),
                    Token::Text(txt) => {
                        match current_state {
                            State::Comment() => elements.push(Comment::boxed(txt.to_string())),
                            _ => {
                                current_state = State::Text();
                                elements.push(Text::boxed(txt.to_string()));
                                element = None;
                            },
                        }
                    },
                    Token::PercentSign() => {
                        if let Some(Token::Text(tag)) = self.tokens.next() {
                            let mut el = Element::boxed(tag.to_string());
                            current_state = State::Element();
                            // elements.push(el);
                            element = Some(el);
                            
                        } else {
                            panic!("Expected tag name after \"%\"");
                        }
                    },
                    Token::Period() => {
                        let mut div = Element::boxed("div".to_string());
                        if let Some(Token::Text(txt)) = self.tokens.next() {
                            div.add_attribute(Attribute::new("class".to_string(), txt.to_string()));
                        } else {    
                            panic!("Trying to parse div in class form (start with period) but couldn't find valid text for class name.");
                        }
                        current_state = State::Element();
                        elements.push(div);
                    },
                    Token::Hashtag() => {
                        let mut div = Element::boxed("div".to_string());
                        if let Some(Token::Text(txt)) = self.tokens.next() {
                            div.add_attribute(Attribute::new("id".to_string(), txt.to_string()));
                        } else {    
                            panic!("Trying to parse div in ID form (start with #) but couldn't find valid text for class name.");
                        }
                        current_state = State::Element();                        
                        elements.push(div);
                    },
                    Token::Indentation() => loop {
                        match self.tokens.next() {
                            Some(Token::Indentation()) => continue,
                            Some(Token::PercentSign()) => {
                            },
                            None => break,
                            _ => panic!("Unexpected token in parsing"),
                        }
                    },
                    Token::Whitespace() => continue,
                    Token::OpenParen() => {
                        let mut attributes = self.parse_attributes();
                        match current_state {
                            State::Element() => {
                                match &element {
                                    Some(ref mut el) => {
                                        el.add_attributes(&mut attributes);
                                        elements.push(el);
                                    },
                                    _ => continue,
                                }
                            },
                            _ => continue,
                        }
                    },
                    t => panic!(format!("{}: Unsupported feature", t.to_string())),
                },
                None => break,
            }
        }
        elements
    }

    fn parse_attributes(&mut self) -> Vec<Attribute> {
        let mut attributes = vec![];
        let mut current_state = AttributeState::Start();
        loop {
            if let Some(tok) = self.tokens.next() {
                match tok {
                    Token::Whitespace() => continue,
                    Token::Text(txt) => {
                        match current_state {
                            AttributeState::Start() => {
                                current_state = AttributeState::Id(txt.to_string());
                            },
                            AttributeState::Id(id) => {
                                &attributes.push(Attribute::new(id.to_string(), txt.to_string()));
                                current_state = AttributeState::Start();
                            },
                            _ => panic!("Invalid token when parsing attributes"),
                        }
                    },
                    Token::Equal() => {
                        match &current_state {
                            AttributeState::Id(_id) => continue,
                            _ => panic!("Invalid \"=\" when trying to parse attributes."),
                        }
                    },
                    Token::CloseParen() => break,
                    _ => panic!("Invalid token when parsing attributes"),
                }
            } else {
                break;
            }
        }
        //el.add_attributes(&mut attributes);
        attributes
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use scanner::Scanner;

    #[test]
    fn test_basic_element() {
        let haml = "%span";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();

        assert_eq!(1, elements.len());
        assert_eq!(
            &Some("span".to_string()),
            elements.iter().nth(0).unwrap().tag()
        );
    }

    #[test]
    fn test_basic_element_with_single_attribute() {
        let haml = "%span( id=\"test\")";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();

        assert_eq!(1, elements.len());
        let actual_element = elements.iter().nth(0).unwrap();
        assert_eq!(&Some("span".to_string()), actual_element.tag());
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
    fn test_comment_only_one_line() {
        let haml = "\\ comment\n%span";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();

        assert_eq!(2, elements.len());
        let actual_first_element = elements.iter().nth(0).unwrap();
        assert_eq!(&None, actual_first_element.tag());
        assert_eq!("<!-- comment -->", actual_first_element.to_html());

        let actual_second_element = elements.iter().nth(1).unwrap();
        assert_eq!(&Some("span".to_string()), actual_second_element.tag());
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
