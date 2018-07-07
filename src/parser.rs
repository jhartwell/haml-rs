use ast::{Comment, Element, Html, Text};
use std::slice::Iter;
use values::{Token, TokenValue};

pub struct Parser<'a> {
    tokens: &'a Vec<TokenValue>,
    iter: Iter<'a, TokenValue>,
    current_token: Option<&'a TokenValue>,
    indentation: u32,
}

impl<'a> Iterator for Parser<'a> {
    type Item = Box<dyn Html>;

    fn next(&mut self) -> Option<Box<dyn Html>> {
        loop {
            let next_token = self.iter.next();
            if let Some(tok) = next_token {
                match tok.get_token() {
                    Token::Backslash() => {
                        if self.indentation > 1 && self.indentation % 2 == 0 {
                            break;
                        } else {
                            break;
                        }
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        None
    }
}

// Backslash(),
//     Period(),
//     Equal(),
//     DoubleQuote(),
//     OpenParen(),
//     CloseParen(),
//     Whitespace(),
//     EndLine(),
//     PercentSign(),
//     AtSymbol(),
//     Char(char),

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<TokenValue>) -> Parser<'a> {
        Parser {
            tokens,
            iter: tokens.iter(),
            current_token: None,
            indentation: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Box<dyn Html>> {
        let mut values = vec![];
        loop {
            if let Some(val) = self.next() {
                values.push(val);
            } else {
                break;
            }
        }
        values
    }
}
