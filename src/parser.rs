use ast::{Attributes, Comment, Element, Html, Text};
use std::slice::Iter;
use values::{Token, TokenValue};

pub struct Parser<'a> {
    tokens: &'a Vec<TokenValue>,
    iter: Iter<'a, TokenValue>,
    current_value: Option<&'a TokenValue>,
    previous_token: &'a Token,
    indentation: u32,
    is_quoted: bool,
}

impl<'a> Iterator for Parser<'a> {
    type Item = Box<dyn Html>;

    fn next(&mut self) -> Option<Box<dyn Html>> {
        self.current_value = self.iter.next();
        if let Some(current_value) = self.current_value {
            if current_value.get_position() == 1 {
                let token = current_value.get_token();
                if token == &Token::Backslash() {
                    Some(self.parse_comment())
                } else if token == &Token::PercentSign() {
                    Some(self.parse_element())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

// If starts with backslash, that means all is comment
// if starts with % that means it is an element
// else inner text
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
            current_value: None,
            previous_token: &Token::None(),
            indentation: 0,
            is_quoted: false,
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

    fn parse_comment(&mut self) -> Box<Comment> {
        let mut string_builder = String::new();
        loop {
            if let Some(val) = self.iter.next() {
                if val.get_token() != &Token::EndLine() {
                    string_builder.push(val.get_token().to_char());
                }
            } else {
                break;
            }
        }
        Comment::boxed(string_builder.trim_left().to_string())
    }

    fn parse_element(&mut self) -> Box<Element> {
        let mut tag = String::new();
        loop {
            if let Some(val) = self.iter.next() {
                self.current_value = Some(&val);
                let token = val.get_token();
                if token != &Token::EndLine()
                    && token != &Token::OpenParen()
                    && token != &Token::Whitespace()
                {
                    tag.push(token.to_char());
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Element::boxed(tag)
    }

    // TODO: currently set up to only parse single attribute, 
    // need to check if there is space and then a character
    // after value is set, if so loop again, 
    // if not make sure there is a close parenthesis

    fn parse_attributes(&mut self) -> Attributes {
        let attributes = vec![];
        loop {
            if let Some(id) = self.iter.next() {
                let mut id_builder = String::new();
                loop {
                    let token = id.get_token();
                    match token {
                        Token::Char(ch) => id_builder.push(*ch),
                        _ => {
                            self.previous_token = &token;
                            break;
                        }
                    }
                }
                if self.previous_token == &Token::Whitespace() 
                   || self.previous_token == &Token::Equal()
                   || self.previous_token == &Token::DoubleQuote() {
                       if self.previous_token != &Token::DoubleQuote() {
                           loop {
                               if let Some(quote) = self.iter.next() {
                                   if quote.DoubleQuote() {
                                       break;
                                   }
                               }
                           }
                       }
                    loop {
                        if let Some(attr) = self.iter.next() {
                            let token = attr.get_token();
                            match token {
                                &Token::DoubleQuote() => break;
                            }
                        }
                    }
                } else {
                   panic!("Expected whitespace or equal for attribute at line {}, position {}", id.get_line_number(), id.get_position());
                }
            }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use values::{Token, TokenValue};

    #[test]
    fn test_comment_at_first_position() {
        let tokens = vec![
            TokenValue::new(Token::Backslash(), 1, 1),
            TokenValue::new(Token::Whitespace(), 1, 2),
            TokenValue::new(Token::Char('h'), 1, 3),
            TokenValue::new(Token::Char('i'), 1, 4),
            TokenValue::new(Token::EndLine(), 1, 5),
        ];
        let mut parser = Parser::new(&tokens);
        let html = parser.next().unwrap();
        assert_eq!("<!-- hi -->\n", html.to_html());
    }

    #[test]
    fn test_basic_element_with_no_children_or_attribute() {
        let tokens = vec![
            TokenValue::new(Token::PercentSign(), 1, 1),
            TokenValue::new(Token::Char('s'), 1, 2),
            TokenValue::new(Token::Char('p'), 1, 3),
            TokenValue::new(Token::Char('a'), 1, 4),
            TokenValue::new(Token::Char('n'), 1, 5),
            TokenValue::new(Token::EndLine(), 1, 6),
        ];

        let mut parser = Parser::new(&tokens);
        let html = parser.next().unwrap();
        assert_eq!("<span></span>", html.to_html());
    }

    #[test]
    fn test_basic_element_with_attributes_and_no_children() {
        let tokens = vec![
            TokenValue::new(Token::PercentSign(), 1, 1),
            TokenValue::new(Token::Char('s'), 1, 2),
            TokenValue::new(Token::Char('p'), 1, 3),
            TokenValue::new(Token::Char('a'), 1, 4),
            TokenValue::new(Token::Char('n'), 1, 5),
            TokenValue::new(Token::OpenParen(), 1, 6),
            TokenValue::new(Token::Whitespace(), 1, 7),  
            TokenValue::new(Token::Char('i'), 1, 8),
            TokenValue::new(Token::Char('d'), 1, 9),
            TokenValue::new(Token::Whitespace(), 1, 10),
            TokenValue::new(Token::Equal(), 1, 11),
            TokenValue::new(Token::DoubleQuote(), 1, 12),
            TokenValue::new(Token::Char('t'), 1, 13), 
            TokenValue::new(Token::Char('e'), 1, 14), 
            TokenValue::new(Token::Char('s'), 1, 15), 
            TokenValue::new(Token::Char('t'), 1, 16), 
            TokenValue::new(Token::DoubleQuote(), 1, 17), 
            TokenValue::new(Token::CloseParen(), 1, 18),
            TokenValue::new(Token::EndLine(), 1, 19),
        ];   

        let mut parser = Parser::new(&tokens);
        let html = parser.next().unwrap();
        assert_eq!("<span id=\"test\"></span>", html.to_html());
    }
}
