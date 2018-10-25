use std::iter::Peekable;
use std::str::Chars;
use values::{Tok, Token};

pub struct Scanner<'a> {
    chars: Chars<'a>,
    current_char: Option<char>,
    tokens: Vec<Token>,
    is_quoted: bool,
    previous_token: Option<Token>,
    current_position: u32,
    current_line: u32,
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
            current_line: 1,
            current_position: 0,
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

    fn update(&mut self) -> (u32, u32) {
        let current_position = self.current_position;
        self.current_position += 1;
        (current_position, self.current_line)
    }

    fn update_newline(&mut self) -> (u32, u32) {
        let current_line = self.current_line;
        let current_position = self.current_position;
        self.current_line += 1;
        self.current_position = 0;
        (current_position, current_line)
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
            '\n' => {
                let (current_position, current_line) = self.update_newline();
                Some(Token::new(Tok::EndLine(), current_position, current_line))
            }
            '\r' => match self.chars.next() {
                Some('\n') => {
                    let (current_position, current_line) = self.update_newline();
                    Some(Token::new(Tok::EndLine(), current_position, current_line))
                }
                Some(ch) => {
                    self.update();
                    self.current_char = Some(ch);
                    self.next()
                }
                None => None,
            },
            '(' => {
                let (current_position, current_line) = self.update();
                Some(Token::new(Tok::OpenParen(), current_position, current_line))
            }
            ')' => {
                let (current_position, current_line) = self.update();
                Some(Token::new(
                    Tok::CloseParen(),
                    self.current_position,
                    self.current_line,
                ))
            }

            '"' => {
                self.update();
                let mut text_builder = String::new();
                let mut value: Option<Token> = None;
                loop {
                    match self.chars.next() {
                        Some('"') => {
                            let (current_position, current_line) = self.update();
                            value = Some(Token::new(
                                Tok::Text(text_builder.to_string()),
                                self.current_position,
                                self.current_line,
                            ));
                            break;
                        }
                        Some(ch) => {
                            self.update();
                            text_builder.push(ch);
                        }
                        None => {
                            value = Some(Token::new(
                                Tok::Text(text_builder.to_string()),
                                self.current_position,
                                self.current_line,
                            ));
                            break;
                        }
                    }
                }
                value
            }
            '=' => {
                let (current_position, current_line) = self.update();
                if let Some(ch) = self.chars.next() {
                    match ch {
                        '>' => {
                            let (current_position, current_line) = self.update();
                            Some(Token::new(Tok::Arrow(), current_position, current_line))
                        }
                        _ => {
                            let (current_position, current_line) = self.update();
                            self.current_char = Some(ch);
                            Some(Token::new(Tok::Equal(), current_position, current_line))
                        }
                    }
                } else {
                    Some(Token::new(Tok::Equal(), current_position, current_line))
                }
            }
            '/' => {
                let (current_position, current_line) = self.update();
                Some(Token::new(
                    Tok::ForwardSlash(),
                    current_position,
                    current_line,
                ))
            }
            '%' => {
                let (current_position, current_line) = self.update();
                Some(Token::new(
                    Tok::PercentSign(),
                    current_position,
                    current_line,
                ))
            }
            '.' => {
                let (current_position, current_line) = self.update();
                Some(Token::new(Tok::Period(), current_position, current_line))
            }
            ' ' => {
                let (current_position, current_line) = self.update();
                Some(Token::new(
                    Tok::Whitespace(),
                    current_position,
                    current_line,
                ))
            }
            '@' => {
                let (current_position, current_line) = self.update();
                Some(Token::new(Tok::AtSymbol(), current_position, current_line))
            }
            '#' => {
                let (current_position, current_line) = self.update();
                Some(Token::new(Tok::Hashtag(), current_position, current_line))
            }
            '{' => {
                let (current_position, current_line) = self.update();
                Some(Token::new(
                    Tok::OpenCurlyBrace(),
                    current_position,
                    current_line,
                ))
            }
            '}' => {
                let (current_position, current_line) = self.update();
                Some(Token::new(
                    Tok::ClosedCurlyBrace(),
                    current_position,
                    current_line,
                ))
            }
            '[' => {
                let (current_position, current_line) = self.update();
                Some(Token::new(Tok::OpenBrace(), current_position, current_line))
            }
            ']' => {
                let (current_position, current_line) = self.update();
                Some(Token::new(
                    Tok::ClosedBrace(),
                    current_position,
                    current_line,
                ))
            }
            ':' => {
                let (current_position, current_line) = self.update();
                Some(Token::new(Tok::Colon(), current_position, current_line))
            }
            ',' => {
                let (current_position, current_line) = self.update();
                Some(Token::new(Tok::Comma(), current_position, current_line))
            }
            '-' => {
                let (current_position, current_line) = self.update();
                Some(Token::new(Tok::Dash(), current_position, current_line))
            }
            c => {
                let mut text_builder = String::new();
                text_builder.push(c);

                let (mut current_position, current_line) = self.update();
                while let Some(next_char) = self.chars.next() {
                    current_position += 1;
                    self.update();
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
                    Some(Token::new(
                        Tok::Text(text_builder),
                        current_position,
                        current_line,
                    ))
                } else {
                    if &text_builder == "!!!" {
                        Some(Token::new(Tok::DocType(), current_position, current_line))
                    } else {
                        Some(Token::new(
                            Tok::Text(text_builder),
                            current_position,
                            current_line,
                        ))
                    }
                }
            }
        };
        self.previous_token = return_value.clone();
        return_value
    }
}
