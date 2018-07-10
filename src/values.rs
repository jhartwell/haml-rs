
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
