#[derive(Debug)]
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
    Char(char),
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_token_value() {
        let line_number = 1;
        let position = 1;
        let token = Token::AtSymbol();
        let expected_token = token.clone();
        let token_value = TokenValue::new(token, position, line_number);
        assert_eq!(line_number, token_value.get_line_number());
        assert_eq!(position, token_value.get_position());
        assert_eq!(&expected_token, token_value.get_token());
    }
}
