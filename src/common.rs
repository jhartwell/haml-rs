#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Element(String),
    ImpliedDiv(),
    StartAttributes(),
    EndAttributes(),
    Class(String),
    Id(String),
    Whitespace(u32),
    Text(String),
    Newline(),
    Equal(),
    Quoted(),
    Arrow(),
    Slash(),
    Colon(),
}

impl Token {
    pub fn update(token: &Token, data: &str) -> Self {
        match token {
            Token::Element(_) => Token::Element(data.to_string()),
            Token::Class(_) => Token::Class(data.to_string()),
            Token::Id(_) => Token::Id(data.to_string()),
            Token::Text(_) => Token::Text(data.to_string()),
            token => token.clone(),
        }
    }
}