use ast::{Arena, Html, HtmlElement};
use std::slice::Iter;
use std::iter::Peekable;
use values::Token;

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    arena: Arena,
    current_token: Option<&'a Token>,
    current_position: u32,
    fresh_line: bool,
}

pub struct Parsed(Option<Html>);

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            tokens: tokens.iter().peekable(),
            arena: Arena::new(),
            current_token: None,
            current_position: 0,
            fresh_line: true,
        }
    }

    pub fn parse(&mut self) -> &Arena {
        let mut previous_indent = 0;
        let mut current_index: usize = 0;
        let mut root_node = true;
        loop {
            match self.do_parse() {
                Parsed(Some(html)) => {
                    if !root_node {
                        let parent_id = self.arena.parent(current_index);
                        let sibling_id = self.arena.new_node(html, 0);
                        self.arena.add_sibling(current_index, sibling_id);
                        self.arena.add_child(sibling_id, parent_id);
                        current_index = sibling_id;
                    } else {
                        root_node = false;
                        current_index = self.arena.new_node(html, 0);
                    }
                }
                _ => break,
            }
        }
        &self.arena
    }

    fn next_text(&mut self) -> HtmlElement {
        self.current_position += 1;
        match self.tokens.next() {
            Some(Token::Text(txt)) => {
                let mut text: &str = txt;
                if txt.ends_with("<") {
                    if let Some((i, _)) = txt.char_indices().rev().nth(0) {
                        text = &txt[..i];
                    }
                }
                HtmlElement::new(text.to_string())
            }
            _ => panic!("Expected text"),
        }
    }

    fn next_token(&mut self) -> &Token {
        let token = self.tokens.next();
        if let Some(Token::EndLine()) = token {
            self.current_position = 0;
        } else {
            self.current_position += 1;
        }
    }

    fn do_parse(&mut self) -> Parsed {
        let mut element: Option<Html> = None;
        let mut token: Option<&Token> = None;
        let mut just_added_element = false;
        let mut multi_line_element = false;
        let mut current_position = 0;
        let mut element_start_position: Option<u32> = None;
        loop {
            if self.current_token == None {
                token = self.next_token();
                self.current_position += 1;
            } else {
                token = self.current_token;
                self.current_token = None;
            }
            match token {
                Some(tok) => match tok {
                    Token::PercentSign() => {
                        element = Some(Html::Element(self.next_text()));
                        just_added_element = true;
                        element_start_position = Some(self.current_position);
                    }
                    Token::Period() => {
                        element_start_position = Some(self.current_position);
                        let mut class = String::new();
                        let key = "class".to_string();
                        match self.tokens.next() {
                            Some(Token::Text(txt)) => {
                                self.current_position += 1;
                                class = txt.to_string();
                            },
                            _ => panic!("Expecting text value for class name"),
                        }
                        if let Some(Html::Element(ref mut el)) = element {
                            el.add_attribute(key, class);
                        } else {
                            let mut el = HtmlElement::new("div".to_string());
                            el.add_attribute(key, class);
                            element = Some(Html::Element(el));
                        }
                    },
                    Token::Dash() => {
                        self.parse_silent_comment();
                        
                    }
                    Token::Hashtag() => {
                        let mut id = String::new();
                        let key = "id".to_string();
                        self.current_position += 1;
                        element_start_position = Some(self.current_position);
                        match self.tokens.next() {
                            Some(Token::Text(txt)) => {
                                self.current_position += 1;
                                id = txt.to_string();
                            },
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
                        self.current_position += 1;
                        if let Some(Html::Element(ref mut el)) = element {
                            self.parse_attributes(el);
                        } else {
                            panic!("Unexpected \"(\" while parsing");
                        }
                    }
                    Token::ForwardSlash() => {
                        let comment = self.parse_comment();
                        element = Some(comment);
                        break;
                    }
                    Token::EndLine() => {
                        self.current_position = 0;
                        match element {
                        Some(Html::Element(ref mut el)) => {
                            if !just_added_element {
                                el.body.push('\n');
                            } else {
                                just_added_element = false;
                            }
                        }
                        _ => {
                            loop {
                                if let Some(Token::Whitespace()) = self.tokens.peek() {
                                    self.fresh_line = true;
                                    self.tokens.next();
                                    self.current_position += 1;
                                }
                            }
                            if let Some(Token::Whitespace()) = self.tokens.peek() P
                        },
                    }},
                    Token::DocType() => loop {
                        match self.tokens.next() {
                            Some(Token::Text(ref text)) => {
                                element = Some(Html::Doctype(text.to_string()));
                                break;
                            }
                            Some(Token::Whitespace()) => continue,
                            Some(Token::EndLine()) => break,
                            None => break,
                            Some(tok) => panic!(format!("Expecting Text but found {:?}", tok)),
                        }
                    },
                    Token::Whitespace() => continue,
                    Token::Text(txt) => {
                        let mut text_builder = txt.clone();
                        loop {
                            match self.tokens.next() {
                                Some(Token::Whitespace()) => text_builder.push(' '),
                                Some(Token::Text(ref text)) => text_builder.push_str(&text),
                                Some(tok) => {
                                    self.current_token = Some(tok);
                                    break;
                                }
                                None => break,
                            }
                        }
                        if let Some(Html::Element(ref mut ele)) = element {
                            ele.body.push_str(&text_builder);
                        } else {
                            element = Some(Html::Text(text_builder));
                        }
                    }
                    Token::OpenCurlyBrace() => {
                        if let Some(Html::Element(ref mut el)) = element {
                            self.parse_ruby_attributes(el);
                        } else {
                            panic!("Unexpected \"{\" while parsing");
                        }
                    }
                    t => panic!(format!("Unsupported feature: {:?}", t)),
                },
                None => break,
            }
        }
        Parsed(element)
    }

    fn parse_silent_comment(&mut self) -> Html {
        if let Some(Token::EndLine()) = self.current_token {
                            element_start_position = Some(self.current_position);
                            if let Some(Token::Hashtag()) = self.tokens.peek() {
                                self.current_position += 1;
                                self.tokens.next();
                                element = Some(Html::SilentComment())
                            }
                        }
    }
    fn parse_ruby_attributes(&mut self, element: &mut HtmlElement) {
        let mut id = "";
        loop {
            match self.tokens.next() {
                Some(tok) => match tok {
                    Token::ClosedCurlyBrace() => break,
                    Token::Colon() => {
                        match self.tokens.next() {
                            Some(Token::Text(ref text)) => {
                                self.current_position += 1;
                             id = text;
                        }
                            Some(tok) => panic!(format!("Expected an identifier after a colon when parsing attributes but found {:?}", tok)),
                            None => panic!("Unexpected end of file when parsing attributes"),
                        }
                    }
                    Token::Arrow() => {
                        loop {
                            self.current_position += 1;
                            match self.tokens.next() {
                                Some(Token::Whitespace()) => continue,
                                Some(Token::Text(ref value)) => {
                                    match id {
                                        "" => panic!("Found a value for an attribute but no attribute id."),
                                        i => element.add_attribute(i.to_string(), value.to_string()),
                                    }
                                    break;
                                },
                                Some(Token::OpenBrace()) => {
                                    loop {
                                        self.current_position += 1;
                                        match self.tokens.next() {
                                            Some(Token::Text(ref text)) => element.add_attribute(id.to_string(), text.to_string()),
                                            Some(Token::Whitespace()) => continue,
                                            Some(Token::Comma()) => continue,
                                            Some(Token::ClosedBrace()) => break,
                                            Some(tok) => panic!(format!("Unexpected token {:?} in attribute array.", tok )),
                                            None => panic!("Ran out of tokens while parsing attributes"),
                                        }
                                    }
                                    break;
                                }
                                Some(tok) => panic!(format!("Expecting value after attribute id in attributes but found {:?}", tok)),
                                None => panic!("Expecting value after \"=>\""),
                            }
                        }
                    }
                    Token::Comma() => id = "",
                    Token::Text(ref text) => id = text,
                    _ => continue,
                },
                None => break,
            }
        }
    }

    fn parse_attributes(&mut self, element: &mut HtmlElement) {
        let mut at_id = true;
        let mut id = "";
        while let Some(tok) = self.tokens.next() {
            self.current_position += 1;
            match tok {
                Token::CloseParen() => break,
                Token::Text(txt) => {
                    if at_id {
                        id = txt
                    } else {
                        let attribute_value = match element.tag() {
                            "input" => {
                                if txt == "true" {
                                    "checked".to_string()
                                } else {
                                    txt.to_string()
                                }
                            }
                            _ => txt.to_string(),
                        };
                        element.add_attribute(id.to_string(), attribute_value);
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
            }
        }
    }

    fn parse_comment(&mut self) -> Html {
        let mut comment_builder = String::new();
        let mut has_newline = false;
        let mut last_token_newline = false;
        loop {
            self.current_position += 1;
            match self.tokens.next() {
                Some(Token::EndLine()) => {
                    self.current_position = 0;
                    has_newline = true;
                    last_token_newline = true;
                    comment_builder.push('\n');
                }
                Some(Token::Text(txt)) => {
                    last_token_newline = false;
                    comment_builder.push_str(txt);
                }
                Some(Token::Whitespace()) => {
                    if !last_token_newline {
                        comment_builder.push(' ');
                    }
                }
                None => break,
                _ => last_token_newline = false,
            }
        }
        if has_newline {
            comment_builder.push('\n');
        }
        Html::Comment(comment_builder.to_string())
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

        let node = arena.node_at(0);
        assert_eq!(None, node.next_sibling());
        assert_eq!(0, node.children().len());
    }

    #[test]
    fn test_basic_children() {
        let haml = "%span\n  %a";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let arena = parser.parse();

        let node = arena.node_at(0);
        assert_eq!(None, node.next_sibling());
        assert_eq!(1, node.children().len());

        let child_id = node.children().iter().nth(0).unwrap();
        let child_node = arena.node_at(*child_id);
        assert_eq!(None, child_node.next_sibling());
        assert_eq!(0, child_node.children().len());
    }

    #[test]
    fn test_nested_children() {
        let haml = "%div\n  %span\n    %a";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let arena = parser.parse();

        let node = arena.node_at(0);
        assert_eq!(None, node.next_sibling());
        assert_eq!(1, node.children().len());

        let child_id = *node.children().iter().nth(0).unwrap();
        let child_node = arena.node_at(child_id);
        assert_eq!(None, child_node.next_sibling());
        assert_eq!(1, child_node.children().len());

        let grandchild_id = *child_node.children().iter().nth(0).unwrap();
        let grandchild_node = arena.node_at(grandchild_id);
        assert_eq!(None, grandchild_node.next_sibling());
        assert_eq!(0, grandchild_node.children().len());
    }

    #[test]
    fn test_siblings() {
        let haml = "%div\n  %span\n  %a";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let arena = parser.parse();

        let node = arena.node_at(0);
        assert_eq!(None, node.next_sibling());
        assert_eq!(2, node.children().len());

        let child_id1 = *node.children().iter().nth(0).unwrap();
        let child_node1 = arena.node_at(child_id1);
        assert_eq!(Some(2), child_node1.next_sibling());
        assert_eq!(0, child_node1.children().len());

        let child_id2 = child_node1.next_sibling().unwrap();
        let child_node2 = arena.node_at(child_id2);
        assert_eq!(None, child_node2.next_sibling());
        assert_eq!(0, child_node2.children().len());
    }

    #[test]
    fn test_comment() {
        let haml = "/ comment";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let arena = parser.parse();

        let node = arena.node_at(0);
        assert_eq!(None, node.next_sibling());
        assert_eq!(0, node.children().len());
    }

    #[test]
    fn test_nested_text() {
        let haml = "%span\n  text";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let arena = parser.parse();

        let node = arena.node_at(0);
        assert_eq!(None, node.next_sibling());
        assert_eq!(1, node.children().len());
    }

    #[test]
    fn test_doctype() {
        let haml = "!!! 5";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let arena = parser.parse();

        let node = arena.node_at(0);
        assert_eq!(None, node.next_sibling());
        assert_eq!(0, node.children().len());
    }

    #[test]
    fn test_ruby_attribute() {
        let haml = "%span{:id => \"test\"}";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let arena = parser.parse();

        let node = arena.node_at(0);
        assert_eq!(None, node.next_sibling());
        assert_eq!(0, node.children().len());
    }

    #[test]
    fn test_ruby_attributes() {
        let haml = "%span{:id => \"test\", :class => \"container\"}";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let arena = parser.parse();

        let node = arena.node_at(0);
        assert_eq!(None, node.next_sibling());
        assert_eq!(0, node.children().len());
    }

    #[test]
    fn test_ruby_attributes_with_array() {
        let haml = "%span{:id => \"test\", :class => [\"container\", \"box\"]}";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);

        let arena = parser.parse();

        let node = arena.node_at(0);
        assert_eq!(None, node.next_sibling());
        assert_eq!(0, node.children().len());
    }
}
