use crate::Token;

pub(crate) fn lex(haml: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut buffer = String::new();
    for ch in haml.chars() {
        match ch {
            ' ' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::Whitespace());
            },
            '(' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::OpenParen());
            },
            ')' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::CloseParen());
            },
            '{' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::OpenBrace());
            },
            '}' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::CloseBrace());
            },
            '%' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::PercentageSign());
            },
            '.' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::Period());
            },
            '=' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::Equal());
            },
            '\"' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::SingleQuote());
            },
            '\'' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::DoubleQuote());
            },
            '\\' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::BackSlash());
            },
            '/' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::ForwardSlash());
            },
            '#' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::Hashtag());
            },
            '<' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::LessThan());
            },
            '>' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::GreaterThan());
            },
            '!' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::Exclamation());
            },
            '&' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::Ampersand());
            },
            '~' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::Tilde());
            },
            '\n' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.to_string()));
                    buffer.clear();
                }
                tokens.push(Token::Newline());
            },
            c => buffer.push(c),
        }

    }
    if !buffer.is_empty() {
        tokens.push(Token::Text(buffer.to_string()));
    }
    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t() {
        let haml = "%test";
        let tokens = lex(haml);
        let mut it = tokens.iter();
        assert_eq!(Some(&Token::PercentageSign()), it.next());
        assert_eq!(Some(&Token::Text("test".to_string())), it.next());
        assert_eq!(None, it.next());
    }
}