use ast::{Html, HtmlDocument, HtmlElement};
use std::fmt;
use std::slice::Iter;
use values::Token;

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

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            tokens: tokens.iter(),
        }
    }

    pub fn parse(&mut self) -> Vec<Html> {
        self.do_parse()
    }

    fn next_text(&mut self) -> HtmlElement {
        match self.tokens.next() {
            Some(Token::Text(txt)) => HtmlElement::new(txt.to_string()),
            _ => panic!("Expected text"),
        }
    }

    fn do_parse(&mut self) -> Vec<Html> {
        let mut nodes = vec![];
        let mut element: Option<HtmlElement> = None;
        loop {
            match self.tokens.next() {
                Some(tok) => match tok {
                    Token::PercentSign() => {
                        element = Some(self.next_text());
                    }
                    Token::Period() => {
                        let mut class = String::new();
                        let key = "class".to_string();
                        match self.tokens.next() {
                            Some(Token::Text(txt)) => class = txt.to_string(),
                            _ => panic!("Expecting text value for class name"),
                        }
                        if let Some(ref mut el) = element {
                            el.add_attribute(key, class);
                        } else {
                            let mut el = HtmlElement::new("div".to_string());
                            el.add_attribute(key, class);
                            element = Some(el);
                        }
                    }
                    Token::Hashtag() => {
                        let mut id = String::new();
                        let key = "id".to_string();
                        match self.tokens.next() {
                            Some(Token::Text(txt)) => id = txt.to_string(),
                            _ => panic!("Expecting text value for id"),
                        }
                        if let Some(ref mut el) = element {
                            el.add_attribute(key, id);
                        } else {
                            let mut el = HtmlElement::new("div".to_string());
                            el.add_attribute(key, id);
                            element = Some(el);
                        }
                    }
                    Token::OpenParen() => {
                        if let Some(ref mut el) = element {
                            self.parse_attributes(el);
                        } else {
                            panic!("Unexpected \"(\" while parsing");
                        }
                    }
                    Token::ForwardSlash() => {
                        let comment = self.parse_comment();
                        if let Some(ref mut el) = element {
                            el.add_child(comment);
                        } else {
                            nodes.push(comment);
                        }
                    }
                    t => panic!(format!("Unsupported feature: {:?}", t)),
                },
                None => break,
            }
        }
        if let Some(el) = element {
            nodes.push(Html::Element(el));
        }
        nodes
    }

    fn parse_attributes(&mut self, element: &mut HtmlElement) {
        let mut at_id = true;
        let mut id = "";
        loop {
            match self.tokens.next() {
                Some(tok) => match tok {
                    Token::CloseParen() => break,
                    Token::Text(txt) => {
                        if at_id {
                            id = txt
                        } else {
                            element.add_attribute(id.to_string(), txt.to_string());
                            id = "";
                            at_id = true;
                        }
                    }
                    Token::Equal() => {
                        if at_id {
                            at_id = false;
                        } else {
                            panic!("Unexpected \"=\" when parsing attributes");
                        }
                    }
                    _ => continue,
                },
                None => break,
            }
        }
    }

    fn parse_comment(&mut self) -> Html {
        let mut comment_builder = String::new();
        loop {
            match self.tokens.next() {
                Some(Token::EndLine()) => break,
                Some(Token::Text(txt)) => comment_builder.push_str(txt),
                _ => continue,
            }
        }
        Html::Comment(comment_builder)
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
        let actual_element = elements.iter().nth(0).unwrap();
        match actual_element {
            Html::Element(el) => {
                assert_eq!("span", el.tag());
                assert_eq!(0, el.children().len());
                assert_eq!(0, el.attributes().size());
            }
            _ => panic!(format!("Expected element but found {:?}", actual_element)),
        }
    }

    #[test]
    fn test_element_with_html_attributes1() {
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
                assert_eq!(1, el.attributes().size());
                let id_attrs = el.attributes().get("id").unwrap();
                assert_eq!(1, id_attrs.len());
                assert_eq!("test", id_attrs.join(" "));
            }
            _ => panic!(format!("Expected element but found {:?}", actual_element)),
        }
    }

    #[test]
    fn test_element_with_html_attributes2() {
        let haml = "%span( id= \"test\")";
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
                assert_eq!(1, el.attributes().size());
                let id_attrs = el.attributes().get("id").unwrap();
                assert_eq!(1, id_attrs.len());
                assert_eq!("test", id_attrs.join(" "));
            }
            _ => panic!(format!("Expected element but found {:?}", actual_element)),
        }
    }

    #[test]
    fn test_element_with_html_attributes3() {
        let haml = "%span( class= \"test\" class=\"it\")";
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
                assert_eq!(1, el.attributes().size());
                let class_attrs = el.attributes().get("class").unwrap();
                assert_eq!(2, class_attrs.len());
                assert_eq!("test it", class_attrs.join(" "));
            }
            _ => panic!(format!("Expected element but found {:?}", actual_element)),
        }
    }

    #[test]
    fn test_element_with_dot_class_notation() {
        let haml = "%span.test";
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
                assert_eq!(1, el.attributes().size());
                let class_attrs = el.attributes().get("class").unwrap();
                assert_eq!(1, class_attrs.len());
                assert_eq!("test", class_attrs.join(" "));
            }
            _ => panic!(format!("Expected element but found {:?}", actual_element)),
        }
    }

    #[test]
    fn test_element_with_dot_class_notation_multiple_classes() {
        let haml = "%span.test.it";
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
                assert_eq!(1, el.attributes().size());
                let class_attrs = el.attributes().get("class").unwrap();
                assert_eq!(2, class_attrs.len());
                assert_eq!("test it", class_attrs.join(" "));
            }
            _ => panic!(format!("Expected element but found {:?}", actual_element)),
        }
    }

        #[test]
    fn test_element_with_hash_id_notation() {
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
                assert_eq!(1, el.attributes().size());
                let id_attrs = el.attributes().get("id").unwrap();
                assert_eq!(1, id_attrs.len());
                assert_eq!("test", id_attrs.join(" "));
            }
            _ => panic!(format!("Expected element but found {:?}", actual_element)),
        }
    }

    #[test]
    fn test_element_with_hash_id_notation_with_class() {
        let haml = "%span#test.it";
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
                assert_eq!(2, el.attributes().size());

                let id_attrs = el.attributes().get("id").unwrap();
                assert_eq!(1, id_attrs.len());
                assert_eq!("test", id_attrs.join(" "));

                let class_attrs = el.attributes().get("class").unwrap();
                assert_eq!(1, class_attrs.len());
                assert_eq!("it", class_attrs.join(" "));
            }
            _ => panic!(format!("Expected element but found {:?}", actual_element)),
        }
    }    

    #[test]
    fn test_div_with_id_syntax() {
        let haml = "#test";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();

        assert_eq!(1, elements.len());
        let actual_element = elements.iter().nth(0).unwrap();
        match actual_element {
            Html::Element(el) => {
                assert_eq!("div", el.tag());
                assert_eq!(0, el.children().len());
                assert_eq!(1, el.attributes().size());

                let id_attrs = el.attributes().get("id").unwrap();
                assert_eq!(1, id_attrs.len());
                assert_eq!("test", id_attrs.join(" "));
            }
            _ => panic!(format!("Expected element but found {:?}", actual_element)),
        }
    }        

    #[test]
    fn test_div_with_id_syntax_and_class() {
        let haml = "#test.container";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let elements = parser.parse();

        assert_eq!(1, elements.len());
        let actual_element = elements.iter().nth(0).unwrap();
        match actual_element {
            Html::Element(el) => {
                assert_eq!("div", el.tag());
                assert_eq!(0, el.children().len());
                assert_eq!(2, el.attributes().size());

                let id_attrs = el.attributes().get("id").unwrap();
                assert_eq!(1, id_attrs.len());
                assert_eq!("test", id_attrs.join(" "));

                let class_attrs = el.attributes().get("class").unwrap();
                assert_eq!(1, class_attrs.len());
                assert_eq!("container", class_attrs.join(" "));
            }
            _ => panic!(format!("Expected element but found {:?}", actual_element)),
        }
    }        
}
