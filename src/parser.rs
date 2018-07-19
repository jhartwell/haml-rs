use ast::{Attribute, Html, HtmlElement};
use std::fmt;
use std::slice::Iter;
use values::Token;

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

#[derive(Debug, PartialEq)]
enum State {
    Start(),
    DocType(),
    BeginElement(),
    EndElement(),
    StartIdDiv(),
    StartClassDiv(),
    Element(),
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
            State::Text() => "Text",
            State::Comment() => "Comment",
            State::NewLine() => "NewLine",
            State::None() => "None",
            State::DocType() => "DocType",
            State::Start() => "Start",
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

    pub fn parse(&mut self) -> Vec<Html> {
        let mut current_state = State::Start();
        let mut elements: Vec<Html> = vec![];
        let mut element: Option<HtmlElement> = None;
        loop {
            match self.tokens.next() {
                Some(tok) => match tok {
                    Token::ForwardSlash() => current_state = State::Comment(),
                    Token::EndLine() => {
                        current_state = if current_state == State::DocType() {
                            State::DocType()
                        } else {
                            State::NewLine()
                        }
                    },
                    Token::Text(txt) => match current_state {
                        State::Start() => {
                            if txt == "!!!" {
                                loop {
                                    match self.tokens.next() {
                                        Some(tok) => match tok {
                                            Token::Text(doctype) => {
                                                current_state = State::DocType();
                                                elements.push(Html::Doctype(doctype.to_string()));
                                                break;
                                            },
                                            Token::EndLine() => break,
                                            _ => continue
                                        },
                                        None => {
                                            elements.push(Html::Doctype("".to_string()));
                                            current_state = State::DocType();
                                        }
                                    }
                                }
                            } else {
                                current_state = State::Text();
                                elements.push(Html::Text(txt.to_string()));
                                element = None;
                            }
                        },
                        State::DocType() => {
                            if txt == "!!!" {
                                loop {
                                    match self.tokens.next() {
                                        Some(tok) => match tok {
                                            Token::Text(doctype) => {
                                                current_state = State::DocType();
                                                elements.push(Html::Doctype(doctype.to_string()));
                                                break;
                                            },
                                            Token::EndLine() => break,
                                            _ => continue
                                        },
                                        None => {
                                            elements.push(Html::Doctype("".to_string()));
                                            current_state = State::DocType();
                                        }
                                    }
                                }
                            } else {
                                current_state = State::Text();
                                elements.push(Html::Text(txt.to_string()));
                                element = None;
                            }
                        }
                        State::Comment() => elements.push(Html::Comment(txt.to_string())),
                        _ => {
                            current_state = State::Text();
                            elements.push(Html::Text(txt.to_string()));
                            element = None;
                        }
                    },
                    Token::PercentSign() => {
                        if let Some(Token::Text(tag)) = self.tokens.next() {
                            current_state = State::Element();
                            element = Some(HtmlElement::new(tag.to_string()));
                        } else {
                            panic!("Expected tag name after \"%\"");
                        }
                    }
                    Token::Period() => match current_state {
                        State::Element() => match self.tokens.next() {
                            Some(tok) => match tok {
                                Token::Text(txt) => {
                                    if let Some(ref mut el) = element {
                                        el.add_attribute(Attribute::new(
                                            "class".to_string(),
                                            txt.to_string(),
                                        ));
                                    }
                                }
                                _ => panic!("Expecting class but didn't find text"),
                            },
                            _ => panic!("Expecting class but didn't find text"),
                        },
                        _ => {
                            let mut div = HtmlElement::new("div".to_string());
                            if let Some(Token::Text(txt)) = self.tokens.next() {
                                div.add_attribute(Attribute::new(
                                    "class".to_string(),
                                    txt.to_string(),
                                ));
                            } else {
                                panic!("Trying to parse div in class form (start with period) but couldn't find valid text for class name.");
                            }
                            current_state = State::Element();
                            element = Some(div);
                        }
                    },
                    Token::Hashtag() => match current_state {
                        State::Element() => match self.tokens.next() {
                            Some(tok) => match tok {
                                Token::Text(txt) => {
                                    if let Some(ref mut el) = element {
                                        el.add_attribute(Attribute::new(
                                            "id".to_string(),
                                            txt.to_string(),
                                        ));
                                    }
                                }
                                _ => panic!("Expected to find ID text when parsing element ID"),
                            },
                            _ => panic!("Expected ID but found nothing."),
                        },
                        _ => {
                            let mut div = HtmlElement::new("div".to_string());
                            if let Some(Token::Text(txt)) = self.tokens.next() {
                                div.add_attribute(Attribute::new(
                                    "id".to_string(),
                                    txt.to_string(),
                                ));
                            } else {
                                panic!("Trying to parse div in ID form (start with #) but couldn't find valid text for class name.");
                            }
                            current_state = State::Element();
                            element = Some(div);
                        }
                    },
                    Token::Indentation() => loop {
                        match self.tokens.next() {
                            Some(Token::Indentation()) => continue,
                            Some(Token::PercentSign()) => {}
                            None => break,
                            _ => panic!("Unexpected token in parsing"),
                        }
                    },
                    Token::Whitespace() => continue,
                    Token::OpenParen() => {
                        let mut attributes = self.parse_attributes();
                        match current_state {
                            State::Element() => match element {
                                Some(ref mut el) => {
                                    el.add_attributes(&mut attributes);
                                }
                                _ => continue,
                            },
                            _ => continue,
                        }
                    }
                    t => panic!(format!("{}: Unsupported feature", t.to_string())),
                },
                None => break,
            }
        }

        if let Some(el) = element {
            elements.push(Html::Element(el));
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
                    Token::Text(txt) => match current_state {
                        AttributeState::Start() => {
                            current_state = AttributeState::Id(txt.to_string());
                        }
                        AttributeState::Id(id) => {
                            &attributes.push(Attribute::new(id.to_string(), txt.to_string()));
                            current_state = AttributeState::Start();
                        }
                        _ => panic!("Invalid token when parsing attributes"),
                    },
                    Token::Equal() => match &current_state {
                        AttributeState::Id(_id) => continue,
                        _ => panic!("Invalid \"=\" when trying to parse attributes."),
                    },
                    Token::CloseParen() => break,
                    _ => panic!(format!(
                        "Invalid token when parsing attributes: {}",
                        tok.to_string()
                    )),
                }
            } else {
                break;
            }
        }
        attributes
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use scanner::Scanner;

    #[test]
    fn test_element() {
        let haml = "%span";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();
        assert_eq!(1, elements.len());
        let actual_element = elements.iter().nth(0).unwrap();
        match actual_element {
            Html::Element(el) => {
                assert_eq!("span", el.tag());

                assert_eq!(0, el.children().len());
                assert_eq!(0, el.attributes().len());
            }
            _ => panic!("Expecting element but found text or comment."),
        }
    }

    #[test]
    fn test_element_id_shorthand() {
        let haml = "%span#test";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();
        assert_eq!(1, elements.len());
        let actual_element = elements.iter().nth(0).unwrap();
        match actual_element {
            Html::Element(el) => {
                assert_eq!("span", el.tag());

                assert_eq!(0, el.children().len());
                assert_eq!(1, el.attributes().len());
                let actual_attribute = el.attributes().iter().nth(0).unwrap();
                assert_eq!("id", actual_attribute.key());
                assert_eq!("test", actual_attribute.value());
            }
            _ => panic!("Expecting element but found text or comment."),
        }
    }

    #[test]
    fn test_element_with_class_shorthand() {
        let haml = "%span.container";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        println!("{:?}", tokens);
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();
        assert_eq!(1, elements.len());
        let actual_element = elements.iter().nth(0).unwrap();
        match actual_element {
            Html::Element(el) => {
                assert_eq!("span", el.tag());
                assert_eq!(0, el.children().len());
                assert_eq!(1, el.attributes().len());
                let actual_attribute = el.attributes().iter().nth(0).unwrap();
                assert_eq!("class", actual_attribute.key());
                assert_eq!("container", actual_attribute.value());
            }
            _ => panic!("Expected element but found text or comment"),
        }
    }

    #[test]
    fn test_element_with_attributes() {
        let haml = "%span(id=\"test\")";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();
        assert_eq!(1, elements.len());
        let actual_element = elements.iter().nth(0).unwrap();
        match actual_element {
            Html::Element(el) => {
                assert_eq!("span", el.tag());
                assert_eq!(0, el.children().len());
                assert_eq!(1, el.attributes().len());
                let actual_attribute = el.attributes().iter().nth(0).unwrap();
                assert_eq!("id", actual_attribute.key());
                assert_eq!("test", actual_attribute.value());
            }
            _ => panic!("Expected element but found text or comment"),
        }
    }

    #[test]
    fn test_single_doctype() {
        let haml = "!!! Basic";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();
        assert_eq!(1, elements.len());
        let actual_element = elements.iter().nth(0).unwrap();
        match actual_element {
            Html::Doctype(txt) => {
                assert_eq!("Basic", txt);
            }
            _ => panic!("Expected doctype but found something else"),
        }
    }

    #[test]
    fn test_multiple_doctype() {
        let haml = "!!! Basic\n!!! 5";
        let mut scanner = Scanner::new(haml);
        
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);

        let elements = parser.parse();
        println!("{:?}", elements);
        assert_eq!(2, elements.len());
        let actual_first_element = elements.iter().nth(0).unwrap();
        match actual_first_element {
            Html::Doctype(txt) => {
                assert_eq!("Basic", txt);
            }
            _ => panic!(format!("Expected doctype but found {:?}", actual_first_element)),
        }

        let actual_second_element = elements.iter().nth(1).unwrap();
        match actual_second_element {
            Html::Doctype(txt) => {
                assert_eq!("5", txt);
            }
            _ => panic!(format!("Expected doctype but found {:?}", actual_second_element)),
        }
    }


    #[test]
    fn test_ignore_doctype_when_not_first() {
        let haml = "%span\n!!! Basic";
        let mut scanner = Scanner::new(haml);
        
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);

        let elements = parser.parse();
        assert_eq!(1, elements.len());
        let actual_first_element = elements.iter().nth(0).unwrap();
        match actual_first_element {
            Html::Element(el) => {
                assert_eq!("span", el.tag());
            }
            _ => panic!(format!("Expected Element but found {:?}", actual_first_element)),
        }
    }

    #[test]
    fn test_basic_text() {
        let haml = "test";
        let mut scanner = Scanner::new(haml);

        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);

        let elements = parser.parse();
        assert_eq!(1, elements.len());

                let actual_first_element = elements.iter().nth(0).unwrap();
        match actual_first_element {
            Html::Text(txt) => {
                assert_eq!("test", txt);
            }
            _ => panic!(format!("Expected text but found {:?}", actual_first_element)),
        }
    }


    #[test]
    fn test_bang_text() {
        let haml = "test!";
        let mut scanner = Scanner::new(haml);

        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);

        let elements = parser.parse();
        assert_eq!(1, elements.len());

                let actual_first_element = elements.iter().nth(0).unwrap();
        match actual_first_element {
            Html::Text(txt) => {
                assert_eq!("test!", txt);
            }
            _ => panic!(format!("Expected text but found {:?}", actual_first_element)),
        }
    }    

}
