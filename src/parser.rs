use crate::common::{Element, Html, Text, Token};
use std::any::Any;
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

<<<<<<< HEAD
=======
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
>>>>>>> 694d25c913ed882ba8b7c769ef9bdebed4a63ac8

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            tokens: tokens.iter(),
            state: State::new(),
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
<<<<<<< HEAD
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
=======
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
>>>>>>> 694d25c913ed882ba8b7c769ef9bdebed4a63ac8
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
<<<<<<< HEAD
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
=======
        Box::new(t)
>>>>>>> 694d25c913ed882ba8b7c769ef9bdebed4a63ac8
    }
}
