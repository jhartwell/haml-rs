use ast::{Comment, Element, Html, Text};
use std::slice::Iter;
use values::{Token};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    iter: Iter<'a, Token>,
}

impl<'a> Iterator for Parser<'a> {
    type Item = Box<dyn Html>;

    fn next(&mut self) -> Option<Box<dyn Html>> {
        if let Some(token) = self.iter.next() {
            match token {
                Token::PercentSign() => {
                   
                        if let Some(item) = self.iter.next() {
                            match item {
                                Token::Text(tag) => Some(Element::boxed(tag.to_string())),
                                _ => None
                            }
                        } else {
                            None
                        }
                    
                },
                _ => None
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
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            tokens,
            iter: tokens.iter(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use scanner::Scanner;
    use values::{Token};

    #[test]
    fn test_basic_element() {
        let haml = "%div";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        if let Some(element) = parser.next() {
            assert_eq!(&Some("div".to_string()), element.tag());
        } else {
            panic!("Expected element but found nothing.");
        }
    }

}
