use ast::{Arena, Html, HtmlDocument, HtmlElement, Node};
use std::fmt;
use std::rc::Rc;
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
    arena: Arena,
}

pub struct Parsed(Option<Html>, u32);

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            tokens: tokens.iter(),
            arena: Arena::new(),
        }
    }

    pub fn parse(&mut self) -> &Arena {
        let mut previous_indent = 0;
        let mut current_index: usize = 0;
        loop {
            match self.do_parse() {
                Parsed(Some(html), indent) => {
                    if indent == previous_indent {
                        let parent_id = self.arena.parent(current_index);
                        if parent_id != current_index {
                            let sibling_id = self.arena.new_node(html);
                            self.arena.add_sibling(current_index, sibling_id);
                            self.arena.add_child(sibling_id, parent_id);
                            current_index = sibling_id;
                        } else {
                            current_index = self.arena.new_node(html);
                        }
                    } else if indent > previous_indent {    
                        let child_id = self.arena.new_node(html);                   
                        self.arena.add_child(child_id, current_index);
                        current_index = child_id;
                        previous_indent = indent;
                    } else if indent < previous_indent {
                        let parent_id = self.arena.parent(current_index);
                        let sibling_id = self.arena.new_node(html);
                        self.arena.add_sibling(parent_id, sibling_id);
                        previous_indent = indent;
                        current_index = sibling_id;
                    } else {
                    }
                }
                _ => break,
            }
        }
        &self.arena
    }

    fn next_text(&mut self) -> HtmlElement {
        match self.tokens.next() {
            Some(Token::Text(txt)) => HtmlElement::new(txt.to_string()),
            _ => panic!("Expected text"),
        }
    }

    fn do_parse(&mut self) -> Parsed {
        let mut element: Option<Html> = None;
        let mut current_indent = 0;
        loop {
            match self.tokens.next() {
                Some(tok) => match tok {
                    Token::PercentSign() => {
                        element = Some(Html::Element(self.next_text()));
                    }
                    Token::Period() => {
                        let mut class = String::new();
                        let key = "class".to_string();
                        match self.tokens.next() {
                            Some(Token::Text(txt)) => class = txt.to_string(),
                            _ => panic!("Expecting text value for class name"),
                        }
                        if let Some(Html::Element(ref mut el)) = element {
                            el.add_attribute(key, class);
                        } else {
                            let mut el = HtmlElement::new("div".to_string());
                            el.add_attribute(key, class);
                            element = Some(Html::Element(el));
                        }
                    }
                    Token::Hashtag() => {
                        let mut id = String::new();
                        let key = "id".to_string();
                        match self.tokens.next() {
                            Some(Token::Text(txt)) => id = txt.to_string(),
                            _ => panic!("Expecting text value for id"),
                        }
                        if let Some(Html::Element(ref mut el)) = element {
                            el.add_attribute(key, id);
                        } else {
                            let mut el = HtmlElement::new("div".to_string());
                            el.add_attribute(key, id);
                            element = Some(Html::Element(el));
                        }
                    }
                    Token::OpenParen() => {
                        if let Some(Html::Element(ref mut el)) = element {
                            self.parse_attributes(el);
                        } else {
                            panic!("Unexpected \"(\" while parsing");
                        }
                    }
                    Token::ForwardSlash() => {
                        let comment = self.parse_comment();
                        element = Some(comment);
                    }
                    Token::EndLine() => break,
                    Token::Indentation(indent) => current_indent = *indent,
                    Token::Whitespace() => continue,
                    Token::Text(txt) => element = Some(Html::Text(txt.clone())),
                    t => panic!(format!("Unsupported feature: {:?}", t)),
                },
                None => break,
            }
        }
        Parsed(element, current_indent)
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
                None => break,
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
        let arena = parser.parse();

        assert_eq!(1, arena.len());
        let node = arena.node_at(0);
        assert_eq!(0, node.parent());
        assert_eq!(None, node.next_sibling());
        assert_eq!(None, node.previous_sibling());
        assert_eq!(0, node.children().len());
    }

    #[test]
    fn test_basic_children() {
        let haml = "%span\n  %a";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let arena = parser.parse();
        
        assert_eq!(2, arena.len());
        let node = arena.node_at(0);
        println!("{:?}", node);
        assert_eq!(0, node.parent());
        assert_eq!(None, node.next_sibling());
        assert_eq!(None, node.previous_sibling());
        assert_eq!(1, node.children().len());

        let child_id = node.children().iter().nth(0).unwrap();
        let child_node = arena.node_at(*child_id);
        assert_eq!(0, child_node.parent());
        assert_eq!(None, child_node.next_sibling());
        assert_eq!(None, child_node.previous_sibling());
        assert_eq!(0, child_node.children().len());
    }

    #[test]
    fn test_nested_children() {
        let haml = "%div\n  %span\n    %a";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let arena = parser.parse();

        assert_eq!(3, arena.len());
        let node = arena.node_at(0);
        assert_eq!(0, node.parent());
        assert_eq!(None, node.next_sibling());
        assert_eq!(None, node.previous_sibling());
        assert_eq!(1, node.children().len());

        let child_id = *node.children().iter().nth(0).unwrap();
        let child_node = arena.node_at(child_id);
        assert_eq!(0, child_node.parent());
        assert_eq!(None, child_node.next_sibling());
        assert_eq!(None, child_node.previous_sibling());
        assert_eq!(1, child_node.children().len());

        let grandchild_id = *child_node.children().iter().nth(0).unwrap();
        let grandchild_node = arena.node_at(grandchild_id);
        assert_eq!(child_id, grandchild_node.parent());
        assert_eq!(None, grandchild_node.next_sibling());
        assert_eq!(None, grandchild_node.previous_sibling());
        assert_eq!(0, grandchild_node.children().len());
    }

    #[test]
    fn test_siblings() {
        let haml = "%div\n  %span\n  %a";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let arena = parser.parse();

        assert_eq!(3, arena.len());
        let node = arena.node_at(0);
        assert_eq!(0, node.parent());
        assert_eq!(None, node.next_sibling());
        assert_eq!(None, node.previous_sibling());
        assert_eq!(2, node.children().len());

        let child_id1 = *node.children().iter().nth(0).unwrap();
        let child_node1 = arena.node_at(child_id1);
        assert_eq!(0, child_node1.parent());
        assert_eq!(Some(2), child_node1.next_sibling());
        assert_eq!(None, child_node1.previous_sibling());
        assert_eq!(0, child_node1.children().len());

        let child_id2 = child_node1.next_sibling().unwrap();
        let child_node2 = arena.node_at(child_id2);
        assert_eq!(0, child_node2.parent());
        assert_eq!(None, child_node2.next_sibling());
        assert_eq!(Some(child_id1), child_node2.previous_sibling());
        assert_eq!(0, child_node2.children().len());
    }

    #[test]
    fn test_comment() {
        let haml = "/ comment";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let arena = parser.parse();

        assert_eq!(1, arena.len());
        let node = arena.node_at(0);
        assert_eq!(0, node.parent());
        assert_eq!(None, node.next_sibling());
        assert_eq!(None, node.previous_sibling());
        assert_eq!(0, node.children().len());
    }
}
