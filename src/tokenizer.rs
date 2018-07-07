use std::str::Chars;
use values::{Token, TokenValue};

pub struct Tokenizer<'a> {
    haml: &'a str,
    chars: Chars<'a>,
    tokens: Vec<TokenValue>,
    current_line: u32,
    current_position: u32,
}

impl<'a> Tokenizer<'a> {
    pub fn new(raw_haml: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            haml: &raw_haml,
            chars: raw_haml.chars(),
            tokens: Vec::new(),
            current_line: 1,
            current_position: 0,
        }
    }

    pub fn get_tokens(&mut self) -> &Vec<TokenValue> {
        loop {
            if let Some(token) = self.next() {
                self.tokens.push(token);
            } else {
                break;
            }
        }
        &self.tokens
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = TokenValue;

    fn next(&mut self) -> Option<TokenValue> {
        if let Some(ch) = self.chars.next() {
            self.current_position += 1;
            match ch {
                '\\' => Some(TokenValue::new(
                    Token::Backslash(),
                    self.current_line,
                    self.current_position,
                )),
                '.' => Some(TokenValue::new(
                    Token::Period(),
                    self.current_line,
                    self.current_position,
                )),
                '=' => Some(TokenValue::new(
                    Token::Equal(),
                    self.current_line,
                    self.current_position,
                )),
                '"' => Some(TokenValue::new(
                    Token::DoubleQuote(),
                    self.current_line,
                    self.current_position,
                )),
                '(' => Some(TokenValue::new(
                    Token::OpenParen(),
                    self.current_line,
                    self.current_position,
                )),
                ')' => Some(TokenValue::new(
                    Token::CloseParen(),
                    self.current_line,
                    self.current_position,
                )),
                ' ' => Some(TokenValue::new(
                    Token::Whitespace(),
                    self.current_line,
                    self.current_position,
                )),
                '\n' => {
                    self.current_line += 1;
                    Some(TokenValue::new(
                        Token::EndLine(),
                        self.current_line - 1,
                        self.current_position,
                    ))
                }
                '@' => Some(TokenValue::new(
                    Token::AtSymbol(),
                    self.current_line,
                    self.current_position,
                )),
                '%' => Some(TokenValue::new(
                    Token::PercentSign(),
                    self.current_line,
                    self.current_position,
                )),
                c => Some(TokenValue::new(
                    Token::Char(c),
                    self.current_line,
                    self.current_position,
                )),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_at_symbol() {
        test_helper("@", &Token::AtSymbol(), 1, 1);
    }

    #[test]
    fn test_period() {
        test_helper(".", &Token::Period(), 1, 1);
    }

    #[test]
    fn test_equal() {
        test_helper("=", &Token::Equal(), 1, 1);
    }

    #[test]
    fn test_open_paren() {
        test_helper("(", &Token::OpenParen(), 1, 1);
    }

    #[test]
    fn test_close_paren() {
        test_helper(")", &Token::CloseParen(), 1, 1);
    }
    #[test]
    fn test_whitespace() {
        test_helper(" ", &Token::Whitespace(), 1, 1);
    }
    #[test]
    fn test_endline() {
        test_helper("\n", &Token::EndLine(), 1, 1);
    }
    #[test]
    fn test_percent_sign() {
        test_helper("%", &Token::PercentSign(), 1, 1);
    }

    #[test]
    fn test_char() {
        test_helper("a", &Token::Char('a'), 1, 1);
    }

    #[test]
    fn test_backslash() {
        test_helper("\\", &Token::Backslash(), 1, 1);
    }

    #[test]
    fn test_multiple() {
        let haml = "%a";
        let mut tokenizer = Tokenizer::new(haml);
        let actual_first_token = tokenizer.next();
        let actual_second_token = tokenizer.next();

        if let Some(first_token) = actual_first_token {
            assert_eq!(1, first_token.get_line_number());
            assert_eq!(1, first_token.get_position());
            assert_eq!(&Token::PercentSign(), first_token.get_token());
        } else {
            panic!("Expected at least one token but found none.");
        }

        if let Some(second_token) = actual_second_token {
            assert_eq!(1, second_token.get_line_number());
            assert_eq!(2, second_token.get_position());
            assert_eq!(&Token::Char('a'), second_token.get_token());
        } else {
            panic!("Expected at least two tokens but found only one");
        }
    }

    fn test_helper(
        haml: &str,
        expected_token: &Token,
        expected_line_number: u32,
        expected_position: u32,
    ) {
        let mut tokenizer = Tokenizer::new(haml);
        let token_value = tokenizer.next();
        match token_value {
            Some(tv) => {
                assert_eq!(expected_line_number, tv.get_line_number());
                assert_eq!(expected_position, tv.get_position());
                assert_eq!(expected_token, tv.get_token());
            }
            None => panic!("Expecting a token value but found none."),
        }
    }
}
