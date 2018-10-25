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

impl ToString for Tok {
    fn to_string(&self) -> String {
        match self {
            Tok::ForwardSlash() => "/".to_string(),
            Tok::Period() => ".".to_string(),
            Tok::Equal() => "=".to_string(),
            Tok::OpenParen() => "(".to_string(),
            Tok::CloseParen() => ")".to_string(),
            Tok::Whitespace() => " ".to_string(),
            Tok::EndLine() => "\n".to_string(),
            Tok::PercentSign() => "%".to_string(),
            Tok::AtSymbol() => "@".to_string(),
            Tok::Hashtag() => "#".to_string(),
            Tok::Text(ref txt) => txt.to_string(),
            Tok::DocType() => "".to_string(),
            Tok::OpenCurlyBrace() => "{".to_string(),
            Tok::ClosedCurlyBrace() => "}".to_string(),
            Tok::Colon() => ":".to_string(),
            Tok::Arrow() => "=>".to_string(),
            Tok::OpenBrace() => "[".to_string(),
            Tok::ClosedBrace() => "]".to_string(),
            Tok::Comma() => ",".to_string(),
            Tok::Dash() => "-".to_string(),
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
