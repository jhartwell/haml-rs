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
                        }
                        Some(ch) => text_builder.push(ch),
                        None => {
                            value = Some(Token::Text(text_builder.to_string()));
                            break;
                        }
                    }
                }
                value
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
            ' ' => Some(Token::Whitespace()),
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
                while let Some(next_char) = self.chars.next() {
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
                        if Token::is_delim(&next_char) && next_char != '.' {
                            self.current_char = Some(next_char);
                            break;
                        } else {
                            text_builder.push(next_char);
                        }
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