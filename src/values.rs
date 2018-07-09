#[derive(Debug, PartialEq)]
pub struct TokenValue {
    token: Token,
    line_number: u32,
    position: u32,
}

impl TokenValue {
    pub fn new(token: Token, line_number: u32, position: u32) -> TokenValue {
        TokenValue {
            token,
            line_number,
            position,
        }
    }

    pub fn get_token(&self) -> &Token {
        &self.token
    }

    pub fn get_line_number(&self) -> u32 {
        self.line_number
    }

    pub fn get_position(&self) -> u32 {
        self.position
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Backslash(),
    Period(),
    Equal(),
    DoubleQuote(),
    OpenParen(),
    CloseParen(),
    Whitespace(),
    EndLine(),
    PercentSign(),
    AtSymbol(),
    Hashtag(),
    Text(String),
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Backslash() => "\\".to_string(),
            Token::Period() => ".".to_string(),
            Token::Equal() => "=".to_string(),
            Token::DoubleQuote() => "\"".to_string(),
            Token::OpenParen() => "(".to_string(),
            Token::CloseParen() => ")".to_string(),
            Token::Whitespace() => " ".to_string(),
            Token::PercentSign() => "%".to_string(),
            Token::AtSymbol() => "@".to_string(),
            Token::Hashtag() => "#".to_string(),
            Token::Text(text) => text.to_string(),
            Token::EndLine() => "\n".to_string(),
        }
    }
}

impl Token {
    pub fn is_delim(ch: &char) -> bool {
        match ch {
            '\\' => true,
            '.' => true,
            '=' => true,
            '"' => true,
            '(' => true,
            ')' => true,
            ' ' => true,
            '%' => true,
            '@' => true,
            '#' => true,
            '\n' => true,
            _ => false,
        }
    }
}
// #[cfg(test)]
// mod test {

//     use super::*;

//     #[test]
//     fn test_token_value() {
//         let line_number = 1;
//         let position = 1;
//         let token = Token::`Symbol();
//         let expected_token = token.clone();
//         let token_value = TokenValue::new(token, position, line_number);
//         assert_eq!(line_number, token_value.get_line_number());
//         assert_eq!(position, token_value.get_position());
//         assert_eq!(&expected_token, token_value.get_token());
//     }
// }
