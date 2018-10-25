#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
