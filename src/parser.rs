use crate::common::{Element, Html, Text, Token};
use std::slice::Iter;

pub struct State<'a> {
    pub current_state: Option<&'a Token>,
    pub previous_states: Vec<&'a Token>,
}

impl<'a> State<'a> {
    pub fn new() -> State<'a> {
        State {
            current_state: None,
            previous_states: vec![],
        }
    }

    pub fn unitialized(&self) -> bool {
        self.current_state == None
    }

    pub fn last_state(&self) -> Option<&& Token> {
        self.previous_states.last()
    }

    pub fn update_state(&mut self, new_state: &'a Token) {
        if !self.unitialized() {
            self.previous_states.push(self.current_state.unwrap());
        }
        self.current_state = Some(new_state);
    }

    pub fn last_n_states(&self, n: usize) -> &[&'a Token] {
        &self.previous_states[n..]
    }

    pub fn can_transition(&self, new_state: &'a Token) -> bool {
        if let Some(state) = &self.current_state {
            match state {
                Token::Element(_) => match new_state {
                    Token::Class(_) => true,
                    Token::Id(_) => true,
                    Token::Whitespace(_) => true,
                    Token::StartAttributes() => true,
                    Token::Newline() => true,
                    _ => false,
                },
                Token::ImpliedDiv() => match new_state {
                    Token::Class(_) => true,
                    _ => false,
                },
                Token::StartAttributes() => match new_state {
                    Token::Whitespace(_) => true,
                    Token::Text(_) => true,
                    Token::EndAttributes() => true,
                    _ => false,
                },
                Token::EndAttributes() => match new_state {
                    Token::Whitespace(_) => true,
                    Token::Newline() => true,
                    _ => false,
                },
                Token::Class(_) => match new_state {
                    Token::Class(_) => true,
                    Token::Id(_) => true,
                    Token::Whitespace(_) => true,
                    Token::StartAttributes() => true,
                    Token::Newline() => true,
                    _ => false,
                },
                Token::Id(_) => match new_state {
                    Token::Class(_) => true,
                    Token::Whitespace(_) => true,
                    Token::StartAttributes() => true,
                    Token::Newline() => true,
                    _ => false,
                },
                Token::Whitespace(_) => true,
                Token::Text(_) => match new_state {
                    Token::Whitespace(_) => true,
                    Token::Newline() => true,
                    Token::Equal() => true,
                    Token::Quoted() => true,
                    Token::Colon() => true,
                    Token::Slash() => true,
                    Token::EndAttributes() => true,
                    _ => false,
                },
                Token::Newline() => true,
                Token::Equal() => match new_state {
                    Token::Whitespace(_) => true,
                    Token::Text(_) => true,
                    Token::Arrow() => true,
                    _ => false,
                },
                Token::Arrow() => match new_state {
                    Token::Whitespace(_) => true,
                    Token::Text(_) => true,
                    _ => false,
                },
                Token::Colon() => match new_state {
                    Token::Text(_) => true,
                    _ => false,
                },
                _ => false,
            }
        } else {
            match new_state {
                Token::Text(_) => true,
                Token::Element(_) => true,
                Token::ImpliedDiv() => true,
                Token::Class(_) => true,
                Token::Id(_) => true,
                _ => false,
            }
        }
    }
}

enum ImpliedDivType {
    Class(),
    Id(),
}

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
    state: State<'a>,
}


impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            tokens: tokens.iter(),
            state: State::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<Box<dyn Html>> {
        let a: Vec<Box<Html>> = vec![];
        while let Some(tok) = self.tokens.next() {
            if !self.state.can_transition(&tok) {
                panic!("Can't transition");
            }
            self.state.update_state(&tok);
            match self.state.current_state.unwrap() {
                Token::Whitespace(n) => {

                }
                Token::Element(el) => {self.parse_element(el); ()},
                _ => (),
            }
        }
        a
    }

    fn parse_attributes(&mut self) -> String {
        let mut current = self.tokens.next();
        let mut attr = String::new();
        loop {
            match current {
                Some(Token::Class(class)) => while let Some(t) = self.tokens.next() {},
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
                    while let Some(t) = self.tokens.next() {}
                    break;
                }
                Some(Token::EndAttributes()) => break,
                _ => panic!("Error"),
            }
        }
        attr
    }

    fn check(&self, token: &Token) {
        if !self.state.can_transition(token) {
            panic!("FAAILED");
        }
    }

    fn parse_implied_div(&mut self, val: &str, attr_type: ImpliedDivType) ->  impl Html {
        let mut t = Element::new("div");
        match attr_type {
            ImpliedDivType::Class() => t.add_attributes(&format!("class='{}'", val)),
            ImpliedDivType::Id() => t.add_attributes(&format!("id='{}'", val)),
        }
        t
    }
    fn parse_element(&mut self, tag: &str) -> impl Html {
        let mut t = Element::new(tag);
        let mut indent = 0;
        if let Some(&&Token::Whitespace(i)) = self.state.last_state() {
            indent = i;
        }
        while let Some(current) = self.tokens.next() {
            self.state.update_state(&current);
            match current {
                Token::Whitespace(i) => {
                    if *i <= indent {
                        break;
                    }else {
                        continue;
                    }
                },
                Token::Element(ref e) => {
                    t.add_child(&self.parse_element(e));
                },
                Token::ImpliedDiv() => {
                    if let Some(next) = self.tokens.next() {
                        self.check(next);
                        self.parse_implied_div()
                    }
                }
                _ => (),
            }
        }
        t
    }
}
