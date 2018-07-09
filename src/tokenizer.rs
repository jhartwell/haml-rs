use std::str::Chars;
use values::{Token, TokenValue};

pub struct Tokenizer<'a> {
    haml: &'a str,
    chars: Chars<'a>,
    current_char: Option<char>,
    tokens: Vec<TokenValue>,
    current_line: u32,
    current_position: u32,
    is_quoted: bool,
}

impl<'a> Tokenizer<'a> {
    pub fn new(raw_haml: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            haml: &raw_haml,
            chars: raw_haml.chars(),
            tokens: Vec::new(),
            current_line: 1,
            current_position: 0,
            is_quoted: false,
            current_char: None,
        }
    }

    pub fn get_tokens(&mut self) -> &Vec<TokenValue> {
        loop {
            if let Some(mut token) = self.next() {
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
        let current_char;
        if let Some(current) = self.current_char {
            current_char = current;
            self.current_char = None;
        } else {
            if let Some(ch) = self.chars.next() {
                current_char = ch;
                self.current_position += 1;
            } else {
                return None;
            }
        }
        let return_value = match current_char {
            '\n' => {
                let current_line = self.current_line;
                self.current_line = current_line + 1;
                Some(TokenValue::new(
                    Token::EndLine(),
                    current_line,
                    self.current_position,
                ))
            }
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
            '"' => {
                if self.is_quoted == false {
                    let mut text_builder = String::new();
                    if let Some(c) = self.chars.next() {
                        text_builder.push(c);

                        loop {
                            if let Some(next_char) = self.chars.next() {
                                if self.is_quoted {
                                    // This is quoted which means as long as we don't get another double quote we are
                                    // to take every character and append as text
                                    if next_char != '"' {
                                        text_builder.push(next_char);
                                        self.current_position += 1;
                                        if next_char == '\n' {
                                            self.current_line += 1;
                                            self.current_position = 0;
                                        }
                                    } else {
                                        self.is_quoted = false;
                                        self.current_char = Some(next_char);
                                        // We found the end of the text
                                        break;
                                    }
                                } else {
                                    if Token::is_delim(&next_char) {
                                        self.current_char = Some(next_char);
                                        break;
                                    } else {
                                        text_builder.push(next_char);
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                        Some(TokenValue::new(
                            Token::Text(text_builder),
                            self.current_line,
                            self.current_position,
                        ))
                    } else {
                        Some(TokenValue::new(
                            Token::DoubleQuote(),
                            self.current_line,
                            self.current_position,
                        ))
                    }
                } else {
                    Some(TokenValue::new(
                        Token::DoubleQuote(),
                        self.current_line,
                        self.current_position,
                    ))
                }
            }
            '=' => Some(TokenValue::new(
                Token::Equal(),
                self.current_line,
                self.current_position,
            )),
            '\\' => Some(TokenValue::new(
                Token::Backslash(),
                self.current_line,
                self.current_position,
            )),
            '%' => Some(TokenValue::new(
                Token::PercentSign(),
                self.current_line,
                self.current_position,
            )),
            '.' => Some(TokenValue::new(
                Token::Period(),
                self.current_line,
                self.current_position,
            )),
            ' ' => Some(TokenValue::new(
                Token::Whitespace(),
                self.current_line,
                self.current_position,
            )),
            '@' => Some(TokenValue::new(
                Token::AtSymbol(),
                self.current_line,
                self.current_position,
            )),
            '#' => Some(TokenValue::new(
                Token::Hashtag(),
                self.current_line,
                self.current_position,
            )),
            c => {
                let mut text_builder = String::new();
                text_builder.push(c);
                loop {
                    if let Some(next_char) = self.chars.next() {
                        if self.is_quoted {
                            // This is quoted which means as long as we don't get another double quote we are
                            // to take every character and append as text
                            if next_char != '"' {
                                text_builder.push(next_char);
                                self.current_position += 1;
                                if next_char == '\n' {
                                    self.current_line += 1;
                                    self.current_position = 0;
                                }
                            } else {
                                self.is_quoted = false;
                                self.current_char = Some(next_char);
                                // We found the end of the text
                                break;
                            }
                        } else {
                            if Token::is_delim(&next_char) {
                                self.current_char = Some(next_char);
                                break;
                            } else {
                                text_builder.push(next_char);
                            }
                        }
                    } else {
                        break;
                    }
                }
                Some(TokenValue::new(
                    Token::Text(text_builder),
                    self.current_line,
                    self.current_position,
                ))
            }
        };
        return_value
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
    fn test_basic_text() {
        test_helper("a", &Token::Text("a".to_string()), 1, 1);
    }

    #[test]
    fn test_quoted_text() {
        test_helper("\"a\"", &Token::Text("a".to_string()), 1, 1);
    }

    #[test]
    fn test_backslash() {
        test_helper("\\", &Token::Backslash(), 1, 1);
    }

    #[test]
    fn test_hashtag() {
        test_helper("#", &Token::Hashtag(), 1, 1);
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
            assert_eq!(&Token::Text("a".to_string()), second_token.get_token());
        } else {
            panic!("Expected at least two tokens but found only one");
        }
    }

    #[test]
    fn test_element() {
        let haml = "%span";
        let mut tokenizer = Tokenizer::new(haml);
        let actual_first_token = tokenizer.next();
        let actual_second_token = tokenizer.next();

        if let Some(first_token) = actual_first_token {
            assert_eq!(1, first_token.get_line_number());
            assert_eq!(1, first_token.get_position());
            assert_eq!(&Token::PercentSign(), first_token.get_token());
        } else {
            panic!("Expecting at least two tokens but found none");
        }

        // TODO add second part of test


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
