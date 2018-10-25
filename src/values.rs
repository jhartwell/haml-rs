#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub value: Tok,
    pub position: u32,
    pub line_number: u32,
}

impl Token {
    pub fn new(tok: Tok, position: u32, line_number: u32) -> Token {
        Token {
            value: tok,
            position,
            line_number,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tok {
    ForwardSlash(),
    Period(),
    Equal(),
    OpenParen(),
    CloseParen(),
    Whitespace(),
    EndLine(),
    PercentSign(),
    AtSymbol(),
    Hashtag(),
    Text(String),
    DocType(),
    OpenCurlyBrace(),
    ClosedCurlyBrace(),
    Colon(),
    Arrow(),
    OpenBrace(),
    ClosedBrace(),
    Comma(),
    Dash(),
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
            '\r' => true,
            '{' => true,
            '}' => true,
            ':' => true,
            '[' => true,
            ']' => true,
            _ => false,
        }
    }
}
