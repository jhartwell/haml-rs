use std::str::Chars;
use values::Token;

pub struct Scanner<'a> {
    haml: &'a str,
    chars: Chars<'a>,
    current_char: Option<char>,
    tokens: Vec<Token>,
    is_quoted: bool,
    previous_token: Option<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(raw_haml: &'a str) -> Scanner<'a> {
        Scanner {
            haml: &raw_haml,
            chars: raw_haml.chars(),
            tokens: Vec::new(),
            is_quoted: false,
            current_char: None,
            previous_token: None,
        }
    }

    pub fn get_tokens(&mut self) -> &Vec<Token> {
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
            '(' => Some(Token::OpenParen()),
            ')' => Some(Token::CloseParen()),
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
                                    } else {
                                        self.current_char = Some(next_char);
                                        // We found the end of the text
                                        break;
                                    }
                                } else {
                                    if Token::is_delim(&next_char) {
                                        self.current_char = if next_char != '"' {
                                            Some(next_char)
                                        } else {
                                            None
                                        };
                                        break;
                                    } else {
                                        text_builder.push(next_char);
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                        Some(Token::Text(text_builder))
                    } else {
                        Some(Token::DoubleQuote())
                    }
                } else {
                    self.is_quoted = true;
                    Some(Token::DoubleQuote())
                }
            }
            '=' => Some(Token::Equal()),
            '\\' => Some(Token::Backslash()),
            '%' => Some(Token::PercentSign()),
            '.' => Some(Token::Period()),
            ' ' => {
                let mut return_value = Some(Token::Whitespace());
                if Some(Token::EndLine()) == self.previous_token
                    || Some(Token::Indentation()) == self.previous_token
                {
                    match self.chars.next() {
                        Some(' ') => {
                            return_value = Some(Token::Indentation());
                        }
                        Some(c) => self.current_char = Some(c),
                        None => (),
                    }
                }
                return_value
            }
            '@' => Some(Token::AtSymbol()),
            '#' => Some(Token::Hashtag()),
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
                Some(Token::Text(text_builder))
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
    fn test_endline() {
        let haml = "\n";
        let mut scanner = Scanner::new(haml);
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
    fn test_quoted_text() {
        let haml = "\"a\"";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::Text("a".to_string())), scanner.next());
    }

    #[test]
    fn test_backslash() {
        let haml = "\\";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::Backslash()), scanner.next());
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
    fn test_quoted_double_quote() {
        let haml = "\"\"\"";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::Text("\"".to_string())), scanner.next());
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
        assert_eq!(Some(Token::Indentation()), scanner.next());
        assert_eq!(Some(Token::PercentSign()), scanner.next());
        assert_eq!(Some(Token::Text("span".to_string())), scanner.next());
    }

    #[test]
    fn test_element_multiple_indentation() {
        let haml = "\n    %span";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::EndLine()), scanner.next());
        assert_eq!(Some(Token::Indentation()), scanner.next());
        assert_eq!(Some(Token::Indentation()), scanner.next());
        assert_eq!(Some(Token::PercentSign()), scanner.next());
        assert_eq!(Some(Token::Text("span".to_string())), scanner.next());
    }

    #[test]
    fn test_single_indentation_with_space_later() {
        let haml = "\n  %span(  id=\"test\")";
        let mut scanner = Scanner::new(haml);
        assert_eq!(Some(Token::EndLine()), scanner.next());
        assert_eq!(Some(Token::Indentation()), scanner.next());
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
        assert_eq!(Some(Token::Indentation()), scanner.next());
        assert_eq!(Some(Token::PercentSign()), scanner.next());
        assert_eq!(Some(Token::Text("div".to_string())), scanner.next());
        assert_eq!(None, scanner.next());
    }

}
