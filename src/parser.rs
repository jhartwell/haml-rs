use super::HtmlFormat;
use ast::{Arena, CssElement, Html, HtmlElement};
use std::iter::Peekable;
use std::slice::Iter;
use values::{Tok, Token};

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
        let mut classes = vec![];
        match self.tokens.next() {
            Some(token) => match token.value {
                Tok::Text(ref txt) => {
                    let mut text: &str = txt;
                    let mut items = text.split('.').into_iter();
                    let tag = items.nth(0).unwrap();
                    for class in items {
                        classes.push(class.to_string());
                    }
                    if txt.ends_with("<") {
                        if let Some((i, _)) = txt.char_indices().rev().nth(0) {
                            text = &txt[..i];
                        }
                    }
                    let mut element = HtmlElement::new(tag.to_string());
                    if !classes.is_empty() {
                        for class in classes {
                            element.add_attribute("class".to_string(), class.to_string());
                        }
                    }
                    element
                }
                _ => panic!("Expected text"),
            },
            None => panic!("Expected text"),
        }
    }

    fn do_parse(&mut self) -> Parsed {
        let mut element: Option<Html> = None;
        let mut token: Option<&Token> = None;
        let mut just_added_element = false;
        let mut multi_line_element = false;
        let mut element_start_position: Option<u32> = None;
        loop {
            match self.tokens.next() {
                Some(tok) => match &tok.value {
                    Tok::PercentSign() => {
                        if just_added_element {
                            if let Some(Html::Element(ref mut ele)) = element {
                                ele.add_child(Html::Element(self.next_text()));
                            }
                        } else {
                            element = Some(Html::Element(self.next_text()));
                            just_added_element = true;
                            element_start_position = Some(self.current_position);
                        }
                    }
                    Tok::Period() => {
                        element_start_position = Some(self.current_position);
                        let mut class = String::new();
                        let key = "class".to_string();
                        match self.tokens.next() {
                            Some(token) => match &token.value {
                                Tok::Text(txt) => {
                                    self.current_position += 1;
                                    class = txt.to_string();
                                }
                                _ => panic!("Expecting text value for class name"),
                            },
                            None => continue,
                        }
                        if let Some(Html::Element(ref mut el)) = element {
                            el.add_attribute(key, format!("{}", class));
                        } else {
                            let mut el = HtmlElement::new("div".to_string());
                            el.add_attribute(key, class);
                            element = Some(Html::Element(el));
                        }
                    }
                    Tok::Dash() => {
                        let mut temp = self.tokens.clone();
                        let next_token = temp.peek();
                        if let Some(next_token) = next_token {
                            match &next_token.value {
                                Tok::Hashtag() => element = Some(self.parse_silent_comment(&tok)),
                                _ => continue,
                            }
                        }
                    }
                    Tok::Hashtag() => {
                        let mut id = String::new();
                        let key = "id".to_string();
                        self.current_position += 1;
                        element_start_position = Some(self.current_position);
                        match self.tokens.next() {
                            Some(token) => match &token.value {
                                Tok::Text(txt) => {
                                    self.current_position += 1;
                                    id = txt.to_string();
                                }
                                _ => panic!("Expecting text value for id"),
                            },
                            None => panic!("Expecting text value for id"),
                        }
                        if let Some(Html::Element(ref mut el)) = element {
                            el.add_attribute(key, id);
                        } else {
                            let mut el = HtmlElement::new("div".to_string());
                            el.add_attribute(key, id);
                            element = Some(Html::Element(el));
                        }
                    }
                    Tok::OpenParen() => {
                        self.current_position += 1;
                        if let Some(Html::Element(ref mut el)) = element {
                            self.parse_attributes(el);
                        } else {
                            panic!("Unexpected \"(\" while parsing");
                        }
                    }
                    Tok::ForwardSlash() => {
                        let comment = self.parse_comment();
                        element = Some(comment);
                        break;
                    }
                    Tok::EndLine() => match element {
                        Some(Html::Element(ref mut el)) => {
                            if requires_newline(&el.tag()) {
                                el.body.push('\n');
                            }
                        }
                        _ => (),
                    },
                    Tok::DocType() => loop {
                        match self.tokens.next() {
                            Some(token) => match &token.value {
                                Tok::Text(ref text) => {
                                    element = Some(Html::Doctype(text.to_string()));
                                    break;
                                }
                                Tok::Whitespace() => continue,
                                Tok::EndLine() => break,
                                tok => panic!(format!("Expecting Text but found {:?}", tok)),
                            },
                            None => break,
                        }
                    },
                    Tok::Colon() => {
                        let mut temp = self.tokens.clone();
                        let token = temp.next();
                        self.tokens.next();
                        if let Some(token) = token {
                            match token.value {
                                Tok::Text(ref text) => {
                                    if text == "css" {
                                        element = Some(Html::Css(self.parse_css_filter(tok)));
                                        break;
                                    } else {
                                        self.current_token = Some(token);
                                    }
                                }
                                _ => self.current_token = Some(token),
                            }
                        }
                    }
                    Tok::Whitespace() => continue,
                    Tok::Text(txt) => {
                        let mut text_builder = txt.clone();
                        let mut state: Option<&Token> = Some(tok);
                        let mut has_newline = false;
                        loop {
                            match self.tokens.next() {
                                Some(token) => match &token.value {
                                    Tok::Whitespace() => {
                                        if let Some(tok) = state {
                                            // We want to avoid putting beginning white space on text
                                            // that is on a separate line but still part of the same element
                                            if token.line_number <= tok.line_number && !has_newline
                                            {
                                                text_builder.push(' ');
                                            }
                                        }
                                        state = Some(token);
                                    }
                                    Tok::Text(ref text) => {
                                        let mut already_pushed = false;
                                        if let Some(tok) = self.current_token {
                                            if token.line_number < tok.line_number
                                                && token.position <= tok.position
                                            {
                                                text_builder.push_str(&text.trim());
                                                already_pushed = true;
                                            }
                                        }
                                        if !already_pushed {
                                            text_builder.push_str(&text);
                                        }
                                        has_newline = false;
                                        state = Some(token);
                                    }
                                    Tok::EndLine() => {
                                        text_builder.push('\n');
                                        state = Some(token);
                                        has_newline = true;
                                    }
                                    tok => break,
                                },
                                None => break,
                            }
                        }
                        if let Some(Html::Element(ref mut ele)) = element {
                            ele.body.push_str(&text_builder);
                        } else {
                            element = Some(Html::Text(text_builder));
                        }
                    }
                    Tok::OpenCurlyBrace() => {
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

    fn parse_css_filter(&mut self, previous_token: &Token) -> CssElement {
        let mut css_builder = String::new();
        loop {
            match self.tokens.next() {
                Some(token) => {
                    if token.position == previous_token.position && token.value != Tok::Whitespace()
                    {
                        self.current_token = Some(token);
                        break;
                    } else {
                        css_builder.push_str(&token.value.to_string());
                    }
                }
                None => break,
            }
        }
        CssElement::new(css_builder)
    }

    fn parse_ruby_attributes(&mut self, element: &mut HtmlElement) {
        let mut id = "";
        loop {
            match self.tokens.next() {
                Some(token) => match token.value {
                    Tok::ClosedCurlyBrace() => break,
                    Tok::Colon() => {
                        match self.tokens.next() {
                            Some(token) => {
                                match &token.value {
                            Tok::Text(ref text) => {
                                self.current_position += 1;
                             id = text;
                        }
                            tok => panic!(format!("Expected an identifier after a colon when parsing attributes but found {:?}", tok)),
                            }
                        },
                            None => panic!("Unexpected end of file when parsing attributes"),
                        }
                    }
                    Tok::Arrow() => {
                        loop {
                            self.current_position += 1;
                            match self.tokens.next() {
                                Some(token) => {
                                    match token.value {
                                Tok::Whitespace() => continue,
                                Tok::Text(ref value) => {
                                    match id {
                                        "" => panic!("Found a value for an attribute but no attribute id."),
                                        i => element.add_attribute(i.to_string(), value.to_string()),
                                    }
                                    break;
                                },
                                Tok::OpenBrace() => {
                                    loop {
                                        self.current_position += 1;
                                        match self.tokens.next() {
                                            Some(token) => {
                                                match &token.value {
                                            Tok::Text(ref text) => element.add_attribute(id.to_string(), text.to_string()),
                                            Tok::Whitespace() => continue,
                                            Tok::Comma() => continue,
                                            Tok::ClosedBrace() => break,
                                            tok => panic!(format!("Unexpected token {:?} in attribute array.", tok )),
                                        }
                                    },
                                        None => break,
                                    }
                                    break;
                                }
                                },
                                _ =>  panic!("Expecting value after \"=>\""),
                                    }
                                }
                                    ,None => panic!("Expecting value after \"=>\""),
                                }
                            }
                    }
                    Tok::Comma() => id = "",
                    Tok::Text(ref text) => id = text,
                    _ => continue,
                },
                None => break,
            }
        }
    }

    fn parse_silent_comment(&mut self, current_token: &Token) -> Html {
        let starting_position = current_token.position;
        let starting_line = current_token.line_number;
        let mut text_builder = String::new();
        self.tokens.next(); // advance past hashtag
        let mut current = self.tokens.next();
        let mut i = 0;
        loop {
            match current {
                Some(token) => {
                    if token.position >= starting_position {
                        text_builder.push_str(&token.value.to_string());
                    } else if token.value != Tok::Whitespace()
                        && token.position == starting_position
                    {
                        self.current_token = current;
                        break;
                    }
                }
                None => break,
            }
            current = self.tokens.next();
        }
        Html::SilentComment(text_builder.to_string())
    }

    fn parse_attributes(&mut self, element: &mut HtmlElement) {
        let mut at_id = true;
        let mut id = "";
        while let Some(token) = self.tokens.next() {
            self.current_position += 1;
            match token.value {
                Tok::CloseParen() => break,
                Tok::Text(ref txt) => {
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
                Tok::Equal() => {
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
                Some(token) => match token.value {
                    Tok::EndLine() => {
                        self.current_position = 0;
                        has_newline = true;
                        last_token_newline = true;
                        comment_builder.push('\n');
                    }
                    Tok::Text(ref txt) => {
                        last_token_newline = false;
                        comment_builder.push_str(txt);
                    }
                    Tok::Whitespace() => {
                        if !last_token_newline {
                            comment_builder.push(' ');
                        }
                    }
                    _ => last_token_newline = false,
                },
                None => break,
            }
        }
        if has_newline {
            comment_builder.push('\n');
        }
        Html::Comment(comment_builder.to_string())
    }
}

fn requires_newline(tag: &str) -> bool {
    match tag {
        "p" => true,
        _ => false,
    }
}
