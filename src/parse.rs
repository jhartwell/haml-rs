use crate::{Class, Declaration, Element, Haml, Id, Token};

struct State<'a> {
    tokens: &'a Vec<Token>,
    length: usize,
    index: usize,
}

impl<'a> State<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> State {
        let length = tokens.len();
        State {
            tokens,
            length,
            index: 0,
        }
    }

    fn get_next(&mut self) -> Option<&Token> {
        if self.index + 1 < self.length {
            self.tokens.get(self.index + 1)
        } else {
            None
        }
    }

    fn declaration(&mut self) -> Option<Box<dyn Haml>> {
        if self.index + 1 < self.length {
            let temp = self.index;
            match self.get_next() {
                Some(Token::Exclamation()) => {
                    self.index += 1;
                    match self.get_next() {
                        Some(Token::Exclamation()) => {
                            self.index += 1;
                            match self.get_next() {
                                Some(Token::Text(txt)) => Some(Box::new(Declaration::new(txt))),
                                None => Some(Box::new(Declaration::new(""))),
                                _ => None,
                            }
                        },
                        _ => {
                            self.index = temp;
                            None
                        }
                    }
                },
                _ => {
                    self.index = temp;
                    None
                }
            }
        } else {
            None
        }
    }

    fn classes_and_ids(&mut self, items: &mut Vec<Box<dyn Haml>>) {
        match self.get_next() {
            Some(Token::Period()) => {
                match self.get_next() {
                    Some(Token::Text(txt)) => {
                        items.push(Box::new(Class::new(txt)));
                        self.classes_and_ids(items);
                    },
                    _ => (),
                }
            },
            Some(Token::Hashtag()) => {
                match self.get_next() {
                    Some(Token::Text(txt)) => {
                        items.push(Box::new(Id::new(txt)));
                        self.classes_and_ids(items);
                    },
                    _ => (),
                }
            },
            Some(Token::OpenParen()) => (),
            Some(Token::OpenBrace()) => (),
            _ => panic!("Invalid token"),
        }
    }

    fn element(&mut self) -> Option<Box<dyn Haml>> {
        let mut temp = self.index;
        if self.index + 1 < self.length {
            match self.get_next() {
                Some(Token::Text(txt)) => {
                    self.index = self.index + 1;
                    let inner_temp = self.index;
                    match self.get_next() {
                        Some(Token::Period()) => {
                            let mut items = vec![];
                            self.classes_and_ids(&mut items);
                        }
                    }
                    let element = Element::new(&txt.to_string());
                    let mut items = vec![];
                    self.classes_and_ids(&mut items);
                    element
                                },
                _ => {
                    self.index = temp;
                    None
                }
            }
        } else {
            None
        }
    }
    pub fn next(&mut self) -> Option<Box<dyn Haml>> {
        match self.tokens.get(self.index) {
            Some(Token::Exclamation()) => self.declaration(),
            Some(Token::PercentageSign()) => self.element(),
            _ => None,
        }
    }
}

pub(crate) fn parse(tokens: &Vec<Token>) -> String {
    // let mut arena = Arena::new();
    let mut index = 0;
    let len = tokens.len();
    loop {
        
    }
    for tok in tokens {
        match tok {
            Token::Text(txt) => (),
            Token::Whitespace() => (),
            Token::OpenParen() => (),
            Token::CloseParen() => (),
            Token::OpenBrace() => (),
            Token::CloseBrace() => (),
            Token::PercentageSign() => (),
            Token::Period() => (),
            Token::Equal() => (),            
            Token::SingleQuote() => (),
            Token::DoubleQuote() => (),
            Token::ForwardSlash() => (),
            Token::BackSlash() => (),
            Token::Hashtag() => (),
            Token::LessThan() => (),            
            Token::GreaterThan() => (),      
            Token::Exclamation() => (),      
            Token::Ampersand() => (),      
            Token::Tilde() => (),      
            Token::Newline() => (),      
        }
    }
    String::new()
}