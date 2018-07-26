use std::str::Chars;
use values::Token;

pub struct Scanner<'a> {
    chars: Chars<'a>,
    current_char: Option<char>,
    tokens: Vec<Token>,
    is_quoted: bool,
    previous_token: Option<Token>,
}

impl<'a> Scanner<'a> {
    /// Constructs a new Scanner<'a>
    pub fn new(raw_haml: &'a str) -> Scanner<'a> {
        Scanner {
            chars: raw_haml.chars(),
            tokens: Vec::new(),
            is_quoted: false,
            current_char: None,
            previous_token: None,
        }
    }

    /// Gets the tokens from the Haml that was passed in when the instance
    /// of Scanner was created. This will contain a reference to the tokens
    /// stored in the Scanner struct.
    pub fn get_tokens(&mut self) -> &Vec<Token> {
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

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let current_char;
        if let Some(current) = self.current_char {
            current_char = current;
            self.current_char = None;
        } else {
            if let Some(ch) = self.chars.next() {
                current_char = ch;
            } else {
                return None;
            }
        }
        let return_value = match current_char {
            '\n' => Some(Token::EndLine()),
            '\r' => match self.chars.next() {
                Some('\n') => Some(Token::EndLine()),
                Some(ch) => {
                    self.current_char = Some(ch);
                    self.next()
                }
                None => None,
            },
            '(' => Some(Token::OpenParen()),
            ')' => Some(Token::CloseParen()),
            '"' => {
                let mut text_builder = String::new();
                let mut value: Option<Token> = None;
                loop {
                    match self.chars.next() {
                        Some('"') => {
                            value = Some(Token::Text(text_builder.to_string()));
                            break;
                        },
                        Some(ch) => text_builder.push(ch),
                        None => {
                            value = Some(Token::Text(text_builder.to_string()));
                            break;
                        }
                    }
                }
                value
                // if self.is_quoted == false {
                //     let mut text_builder = String::new();
                //     if let Some(c) = self.chars.next() {
                //         text_builder.push(c);
                //         loop {
                //             if let Some(next_char) = self.chars.next() {
                //                 if self.is_quoted {
                //                     // This is quoted which means as long as we don't get another double quote we are
                //                     // to take every character and append as text
                //                     if next_char != '"' {
                //                         text_builder.push(next_char);
                //                     } else {
                //                         self.current_char = Some(next_char);
                //                         // We found the end of the text
                //                         break;
                //                     }
                //                 } else {
                //                     if Token::is_delim(&next_char) {
                //                         self.current_char = if next_char != '"' {
                //                             Some(next_char)
                //                         } else {
                //                             None
                //                         };
                //                         break;
                //                     } else {
                //                         text_builder.push(next_char);
                //                     }
                //                 }
                //             } else {
                //                 break;
                //             }
                //         }
                //         Some(Token::Text(text_builder))
                //     } else {
                //         Some(Token::DoubleQuote())
                //     }
                // } else {
                //     self.is_quoted = true;
                //     Some(Token::DoubleQuote())
                // }
            }
            '=' => {
                if let Some(ch) = self.chars.next() {
                    match ch {
                        '>' => Some(Token::Arrow()),
                        _ => {
                            self.current_char = Some(ch);
                            Some(Token::Equal())
                        }
                    }
                } else {
                    Some(Token::Equal())
                }
            }
            '/' => Some(Token::ForwardSlash()),
            '%' => Some(Token::PercentSign()),
            '.' => Some(Token::Period()),
            ' ' => {
                let mut return_value = Some(Token::Whitespace());
                if Some(Token::EndLine()) == self.previous_token {
                    let mut count = 1;
                    loop {
                        match self.chars.next() {
                            Some(' ') => {
                                count += 1;
                            }
                            Some(c) => {
                                self.current_char = Some(c);
                                break;
                            }
                            None => break,
                        }
                    }
                    return_value = Some(Token::Indentation(count / 2));
                }
                return_value
            }
            '@' => Some(Token::AtSymbol()),
            '#' => Some(Token::Hashtag()),
            '{' => Some(Token::OpenCurlyBrace()),
            '}' => Some(Token::ClosedCurlyBrace()),
            '[' => Some(Token::OpenBrace()),
            ']' => Some(Token::ClosedBrace()),
            ':' => Some(Token::Colon()),
            ',' => Some(Token::Comma()),
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
                if self.is_quoted {
                    Some(Token::Text(text_builder))
                } else {
                    if &text_builder == "!!!" {
                        Some(Token::DocType())
                    } else {
                        Some(Token::Text(text_builder))
                    }
                }
            }
        };
        self.previous_token = return_value.clone();
        return_value
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_at_symbol() {
        let haml = "@";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::AtSymbol()), scanner.next());
    }

    #[test]
    fn test_period() {
        let haml = ".";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::Period()), scanner.next());
    }

    #[test]
    fn test_equal() {
        let haml = "=";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::Equal()), scanner.next());
    }

    #[test]
    fn test_open_paren() {
        let haml = "(";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::OpenParen()), scanner.next());
    }

    #[test]
    fn test_close_paren() {
        let haml = ")";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::CloseParen()), scanner.next());
    }
    #[test]
    fn test_whitespace() {
        let haml = " ";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::Whitespace()), scanner.next());
    }

    #[test]
    fn test_indentation() {
        let haml = "\n  ";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::EndLine()), scanner.next());
        assert_eq!(Some(Token::Indentation(1)), scanner.next());
    }

    #[test]
    fn test_multiple_indentation() {
        let haml = "\n      ";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::EndLine()), scanner.next());
        assert_eq!(Some(Token::Indentation(3)), scanner.next());
    }

    #[test]
    fn test_endline() {
        let haml = "\n";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::EndLine()), scanner.next());
    }

    #[test]
    fn test_windows_endline() {
        let haml = "\r\n";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::EndLine()), scanner.next());
    }

    #[test]
    fn test_windows_endline_with_text() {
        let haml = "%span\r\n";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::PercentSign()), scanner.next());
        assert_eq!(Some(Token::Text("span".to_string())), scanner.next());
        assert_eq!(Some(Token::EndLine()), scanner.next());
    }

    #[test]
    fn test_percent_sign() {
        let haml = "%";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::PercentSign()), scanner.next());
    }

    #[test]
    fn test_basic_text() {
        let haml = "a";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::Text("a".to_string())), scanner.next());
    }

    #[test]
    fn test_ruby_attribute() {
        let haml = "{:id => \"Test\"}";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::OpenCurlyBrace()), scanner.next());
        assert_eq!(Some(Token::Colon()), scanner.next());
        assert_eq!(Some(Token::Text("id".to_string())), scanner.next());
        assert_eq!(Some(Token::Whitespace()), scanner.next());
        assert_eq!(Some(Token::Arrow()), scanner.next());
        assert_eq!(Some(Token::Whitespace()), scanner.next());
        assert_eq!(Some(Token::Text("Test".to_string())), scanner.next());
        assert_eq!(Some(Token::ClosedCurlyBrace()), scanner.next());
        assert_eq!(None, scanner.next());
    }

    #[test]
    fn test_ruby_attributes() {
        let haml = "{:id => \"Test\", :class => \"container\"}";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::OpenCurlyBrace()), scanner.next());
        assert_eq!(Some(Token::Colon()), scanner.next());
        assert_eq!(Some(Token::Text("id".to_string())), scanner.next());
        assert_eq!(Some(Token::Whitespace()), scanner.next());
        assert_eq!(Some(Token::Arrow()), scanner.next());
        assert_eq!(Some(Token::Whitespace()), scanner.next());
        assert_eq!(Some(Token::Text("Test".to_string())), scanner.next());
        assert_eq!(Some(Token::Comma()), scanner.next());
        assert_eq!(Some(Token::Whitespace()), scanner.next());
        assert_eq!(Some(Token::Colon()), scanner.next());
        assert_eq!(Some(Token::Text("class".to_string())), scanner.next());
        assert_eq!(Some(Token::Whitespace()), scanner.next());
        assert_eq!(Some(Token::Arrow()), scanner.next());
        assert_eq!(Some(Token::Whitespace()), scanner.next());
        assert_eq!(Some(Token::Text("container".to_string())), scanner.next());
        assert_eq!(Some(Token::ClosedCurlyBrace()), scanner.next());
        assert_eq!(None, scanner.next());
    }

    #[test]
    fn test_ruby_attributes_with_dictionary() {
        let haml = "{:id => [\"Test\", \"It\"]}";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::OpenCurlyBrace()), scanner.next());
        assert_eq!(Some(Token::Colon()), scanner.next());
        assert_eq!(Some(Token::Text("id".to_string())), scanner.next());
        assert_eq!(Some(Token::Whitespace()), scanner.next());
        assert_eq!(Some(Token::Arrow()), scanner.next());
        assert_eq!(Some(Token::Whitespace()), scanner.next());
        assert_eq!(Some(Token::OpenBrace()), scanner.next());
        assert_eq!(Some(Token::Text("Test".to_string())), scanner.next());
        assert_eq!(Some(Token::Comma()), scanner.next());
        assert_eq!(Some(Token::Whitespace()), scanner.next());
        assert_eq!(Some(Token::Text("It".to_string())), scanner.next());
        assert_eq!(Some(Token::ClosedBrace()), scanner.next());
        assert_eq!(Some(Token::ClosedCurlyBrace()), scanner.next());
        assert_eq!(None, scanner.next());
    }

    #[test]
    fn test_quoted_text() {
        let haml = "\"a\"";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::Text("a".to_string())), scanner.next());
    }

    #[test]
    fn test_forwardslash() {
        let haml = "/";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::ForwardSlash()), scanner.next());
    }

    #[test]
    fn test_hashtag() {
        let haml = "#";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::Hashtag()), scanner.next());
    }

    #[test]
    fn test_multiple() {
        let haml = "%a";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::PercentSign()), scanner.next());
        assert_eq!(Some(Token::Text("a".to_string())), scanner.next());
    }

    #[test]
    fn test_element() {
        let haml = "%span";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::PercentSign()), scanner.next());
        assert_eq!(Some(Token::Text("span".to_string())), scanner.next());
    }

    #[test]
    fn test_element_with_attributes() {
        let haml = "%span(id=\"test\")";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::PercentSign()), scanner.next());
        assert_eq!(Some(Token::Text("span".to_string())), scanner.next());
        assert_eq!(Some(Token::OpenParen()), scanner.next());
        assert_eq!(Some(Token::Text("id".to_string())), scanner.next());
        assert_eq!(Some(Token::Equal()), scanner.next());
        assert_eq!(Some(Token::Text("test".to_string())), scanner.next());
        assert_eq!(Some(Token::CloseParen()), scanner.next());
    }

    #[test]
    fn test_element_with_multiple_attributes() {
        let haml = "%span(id=\"test\" data=\"target\")";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::PercentSign()), scanner.next());
        assert_eq!(Some(Token::Text("span".to_string())), scanner.next());
        assert_eq!(Some(Token::OpenParen()), scanner.next());
        assert_eq!(Some(Token::Text("id".to_string())), scanner.next());
        assert_eq!(Some(Token::Equal()), scanner.next());
        assert_eq!(Some(Token::Text("test".to_string())), scanner.next());
        assert_eq!(Some(Token::Whitespace()), scanner.next());
        assert_eq!(Some(Token::Text("data".to_string())), scanner.next());
        assert_eq!(Some(Token::Equal()), scanner.next());
        assert_eq!(Some(Token::Text("target".to_string())), scanner.next());
        assert_eq!(Some(Token::CloseParen()), scanner.next());
    }

    #[test]
    fn test_element_indentation() {
        let haml = "\n  %span";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::EndLine()), scanner.next());
        assert_eq!(Some(Token::Indentation(1)), scanner.next());
        assert_eq!(Some(Token::PercentSign()), scanner.next());
        assert_eq!(Some(Token::Text("span".to_string())), scanner.next());
    }

    #[test]
    fn test_element_multiple_indentation() {
        let haml = "\n    %span";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::EndLine()), scanner.next());
        assert_eq!(Some(Token::Indentation(2)), scanner.next());
        assert_eq!(Some(Token::PercentSign()), scanner.next());
        assert_eq!(Some(Token::Text("span".to_string())), scanner.next());
    }

    #[test]
    fn test_single_indentation_with_space_later() {
        let haml = "\n  %span(  id=\"test\")";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::EndLine()), scanner.next());
        assert_eq!(Some(Token::Indentation(1)), scanner.next());
        assert_eq!(Some(Token::PercentSign()), scanner.next());
        assert_eq!(Some(Token::Text("span".to_string())), scanner.next());
        assert_eq!(Some(Token::OpenParen()), scanner.next());
        assert_eq!(Some(Token::Whitespace()), scanner.next());
        assert_eq!(Some(Token::Whitespace()), scanner.next());
        assert_eq!(Some(Token::Text("id".to_string())), scanner.next());
        assert_eq!(Some(Token::Equal()), scanner.next());
        assert_eq!(Some(Token::Text("test".to_string())), scanner.next());
        assert_eq!(Some(Token::CloseParen()), scanner.next());
        assert_eq!(None, scanner.next());
    }

    #[test]
    fn test_multiple_lines() {
        let haml = "%span\n%div";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::PercentSign()), scanner.next());
        assert_eq!(Some(Token::Text("span".to_string())), scanner.next());
        assert_eq!(Some(Token::EndLine()), scanner.next());
        assert_eq!(Some(Token::PercentSign()), scanner.next());
        assert_eq!(Some(Token::Text("div".to_string())), scanner.next());
        assert_eq!(None, scanner.next());
    }

    #[test]
    fn test_multiple_lines_with_indentation() {
        let haml = "%span\n  %div";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::PercentSign()), scanner.next());
        assert_eq!(Some(Token::Text("span".to_string())), scanner.next());
        assert_eq!(Some(Token::EndLine()), scanner.next());
        assert_eq!(Some(Token::Indentation(1)), scanner.next());
        assert_eq!(Some(Token::PercentSign()), scanner.next());
        assert_eq!(Some(Token::Text("div".to_string())), scanner.next());
        assert_eq!(None, scanner.next());
    }

}
