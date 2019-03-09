use std::str::Chars;
use std::collections::HashMap;
use std::ops::Index;

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Str(String),
    Period(),
    PercentSign(),
    SingleQuote(),
    DoubleQuote(),
    Hashtag(),
    Backslash(),
    At(),
    OpenBrace(),
    CloseBrace(),
    OpenParen(),
    CloseParen(),
    Whitespace(),
    Newline(),
    Colon(),
    Equal(),
    GreaterThan(),
}

pub struct Tokenizer<'a> {
    pub haml: &'a str,
    tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(haml: &'a str) -> Tokenizer {
        Tokenizer {
            haml,
            tokens: vec![],
        }
    }

    fn add_and_clear(&mut self, buffer: &mut String, token: Token) {
        if !buffer.is_empty() {
            self.tokens.push(Token::Str(buffer.to_string()));
            buffer.clear();
        }
        self.tokens.push(token);
    }

    pub fn consume(&mut self) -> &Vec<Token> {
        let mut current_char = ' ';
        let mut stream: Chars = self.haml.chars();
        let mut tokens: Vec<Token> = vec![];
        let mut buffer: String = String::new();
        while let Some(current_char) = stream.next() {
            match current_char {
                '.' => self.add_and_clear(&mut buffer, Token::Period()),
                '%' => self.add_and_clear(&mut buffer, Token::PercentSign()),
                '\'' => self.add_and_clear(&mut buffer, Token::SingleQuote()),
                '"' => self.add_and_clear(&mut buffer, Token::DoubleQuote()),
                '#' => self.add_and_clear(&mut buffer, Token::Hashtag()),
                '\\' => self.add_and_clear(&mut buffer, Token::Backslash()),
                '@' => self.add_and_clear(&mut buffer, Token::At()),
                '{' => self.add_and_clear(&mut buffer, Token::OpenBrace()),
                '}' => self.add_and_clear(&mut buffer, Token::CloseBrace()),
                '(' => self.add_and_clear(&mut buffer, Token::OpenParen()),
                ')' => self.add_and_clear(&mut buffer, Token::CloseParen()),
                ' ' => self.add_and_clear(&mut buffer, Token::Whitespace()),
                '\n' => self.add_and_clear(&mut buffer, Token::Newline()),
                ':' => self.add_and_clear(&mut buffer, Token::Colon()),
                '=' => self.add_and_clear(&mut buffer, Token::Equal()),
                '>' => self.add_and_clear(&mut buffer, Token::GreaterThan()),
                c => buffer.push(c),
            }
        }
        if !buffer.is_empty() {
            self.tokens.push(Token::Str(buffer.to_string()));
        }
        &self.tokens
    }
}


pub trait ToHtml : std::fmt::Debug {
    fn html(&self) -> String;
}

#[derive(Debug)]
pub struct Element<'a> {
    pub children: Vec<Box<ToHtml>>,
    pub tag: String,
    pub attributes: HashMap<&'a str, Vec<&'a str>>,
}

#[derive(Debug)]
pub struct Text {
    pub value: String,
}

impl ToHtml for Text {
    fn html(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Attributes {
    pub values: Vec<Item>,
}


impl Attributes {
    pub fn new() -> Attributes {
        Attributes {
            values: vec![]
        }
    }

    pub fn add(&mut self, name: String, value: String) {
        let mut existing_item = &self.values.into_iter().filter(|x| x.name == name);
        if let Some(ref mut item) = existing_item.next() {
            item.value.push_str(&format!(" {}", value));
            return;
        } else {
            self.values.push(Item { name: name.to_string(),  value: value.to_string()});
        }
    }
}


impl ToHtml for Attributes {
    fn html(&self) -> String {
        let mut buffer = String::new();
        for ref val in &self.values {
            buffer.push_str(&format!(" {}='{}'", val.name, val.value));
        }
        buffer
    }
}

impl<'a> ToHtml for Element<'a> {
    fn html(&self) -> String {
        let mut html = String::new();
        html.push_str(&format!("<{}", self.tag));
        let mut keys = self.attributes.iter().collect::<HashMap<&&str, &Vec<&str>>>();
        for (attribute, value) in &keys {
            html.push_str(&format!(" {}=\"{}\"", attribute, value.join(" ")));
        }
        html.push('>');
        for ref child in &self.children {
            html.push_str(&child.html());
        }
        html.push_str(&format!("</{}>", self.tag));
        html
    }
}

pub struct Parser<'a> {
    pub tokens: &'a Vec<Token>,
    index: usize,
    length: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser {
        Parser {
            tokens,
            index: 0,
            length: tokens.len(),
        }
    }

    fn safe_index(&self, index: usize) -> Option<&Token> {
        match index < self.length {
            true => Some(&self.tokens[index]),
            false => None,
        }
    }

    fn match_percent_sign(&mut self) {
        let start_index = self.index;
        self.index = self.index + 1;

        while self.index < self.length {
            match self.tokens[self.index] {
                Token::Str(ref text) => println!("Text: {}", text),
                _ => println!("uhhhh"),
            }
            self.index = self.index + 1;
        }
    }

    pub fn parse(&mut self) -> Vec<Box<ToHtml>> {
        let parsed: Vec<Box<ToHtml>> = vec![];
        self.index = 0;    
        while self.index < self.length {
            match self.tokens[self.index] {
                Token::PercentSign() => self.match_percent_sign(),
                _ => println!("cowboy")
            }
            self.index = self.index + 1;
        }
        parsed
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn period() {
        let mut tokenizer = Tokenizer::new(".");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::Period(), output.first().unwrap());
    }

    #[test]
    fn percent_sign() {
        let mut tokenizer = Tokenizer::new("%");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::PercentSign(), output.first().unwrap());
    }

    #[test]
    fn colon() {
        let mut tokenizer = Tokenizer::new(":");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::Colon(), output.first().unwrap());
    }

    #[test]
    fn equal() {
        let mut tokenizer = Tokenizer::new("=");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::Equal(), output.first().unwrap());
    }

    #[test]
    fn newline() {
        let mut tokenizer = Tokenizer::new("\n");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::Newline(), output.first().unwrap());
    }

    #[test]
    fn whitespace() {
        let mut tokenizer = Tokenizer::new(" ");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::Whitespace(), output.first().unwrap());
    }

    #[test]
    fn close_paren() {
        let mut tokenizer = Tokenizer::new(")");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::CloseParen(), output.first().unwrap());
    }

    #[test]
    fn open_paren() {
        let mut tokenizer = Tokenizer::new("(");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::OpenParen(), output.first().unwrap());
    }

    #[test]
    fn close_brace() {
        let mut tokenizer = Tokenizer::new("}");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::CloseBrace(), output.first().unwrap());
    }

    #[test]
    fn open_brace() {
        let mut tokenizer = Tokenizer::new("{");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::OpenBrace(), output.first().unwrap());
    }

    #[test]
    fn at() {
        let mut tokenizer = Tokenizer::new("@");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::At(), output.first().unwrap());
    }

    #[test]
    fn backslash() {
        let mut tokenizer = Tokenizer::new("\\");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::Backslash(), output.first().unwrap());
    }

    #[test]
    fn hashtag() {
        let mut tokenizer = Tokenizer::new("#");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::Hashtag(), output.first().unwrap());
    }

    #[test]
    fn double_quote() {
        let mut tokenizer = Tokenizer::new("\"");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::DoubleQuote(), output.first().unwrap());
    }

    #[test]
    fn single_quote() {
        let mut tokenizer = Tokenizer::new("'");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::SingleQuote(), output.first().unwrap());
    }

    #[test]
    fn greater_than() {
        let mut tokenizer = Tokenizer::new(">");
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        assert_eq!(&Token::GreaterThan(), output.first().unwrap());
    }

    #[test]
    fn string() {
        let expected = "test";
        let mut tokenizer = Tokenizer::new(&expected);
        let output = tokenizer.consume();
        assert_eq!(1, output.len());
        
        let actual = output.first().unwrap();
        assert_eq!(&Token::Str(expected.to_string()), actual);
    }

    #[test]
    fn multiple_tokens() {
        let haml = "%test";
        let mut tokenizer = Tokenizer::new(&haml);
        let output = tokenizer.consume();

        assert_eq!(2, output.len());
        let mut iter = output.iter();
        assert_eq!(Some(&Token::PercentSign()), iter.next());
        assert_eq!(Some(&Token::Str("test".to_string())), iter.next());
        assert_eq!(None, iter.next());    
    }

    #[test]
    fn element() {
        let mut attributes :  HashMap<&str, Vec<&str>> = HashMap::new();
        attributes.insert("href", vec!["www.google.com"]);
        let expected_html = "<a href=\"www.google.com\"></a>";
        let ele = Element {
            tag: "a".to_string(),
            children: vec![],
            attributes: attributes
        };

        assert_eq!(expected_html.to_string(), ele.html());
    }

    #[test]
    fn element_class() {
        let mut attributes :  HashMap<&str, Vec<&str>> = HashMap::new();
        attributes.insert("class", vec!["button", "sm"]);
        let expected_html = "<a class=\"button sm\"></a>";
        let ele = Element {
            tag: "a".to_string(),
            children: vec![],
            attributes: attributes
        };

        assert_eq!(expected_html.to_string(), ele.html());
    }

    #[test]
    fn element_text() {
        let mut attributes :  HashMap<&str, Vec<&str>> = HashMap::new();
        attributes.insert("class", vec!["button", "sm"]);
        let expected_html = "<a class=\"button sm\">click me</a>";
        let text = Text { value: "click me".to_string() };
        let ele = Element {
            tag: "a".to_string(),
            children: vec![Box::new(text)],
            attributes: attributes
        };

        assert_eq!(expected_html.to_string(), ele.html());
    }

    #[test]
    fn element_multiple_children() {
        let mut attributes :  HashMap<&str, Vec<&str>> = HashMap::new();
        attributes.insert("class", vec!["button", "sm"]);
        let expected_html = "<div>here<a class=\"button sm\">click me</a></div>";
        let text = Text { value: "click me".to_string() };
        let link = Element {
            tag: "a".to_string(),
            children: vec![Box::new(text)],
            attributes: attributes
        };
        let div = Element {
            tag: "div".to_string(),
            children: vec![
                Box::new(Text { value: "here".to_string()}),
                Box::new(link)
            ],
            attributes: HashMap::new()
        };
    
        assert_eq!(expected_html.to_string(), div.html());
    }

    #[test]
    fn basic_parser() {
        let tokens = vec![Token::PercentSign(), Token::Str("Test".to_string())];
        let mut parser = Parser::new(&tokens);
        parser.parse();
        assert_eq!(true, false);
    }


    mod attributes {
        use super::*;
    #[test]
    fn attributes_add() {
        let mut attr = Attributes::new();
        attr.add("test".to_string(), "value".to_string());
        assert_eq!(attr.values.len(), 1);
    }

    #[test]
    fn attributes_add_same() {
        let mut attr = Attributes::new();
        attr.add("test".to_string(), "value".to_string());
        attr.add("test".to_string(), "it".to_string());
        assert_eq!(attr.values.len(), 1);
    }

    #[test]
    fn single_attribute_single_value_to_html() {
        let mut attr = Attributes::new();
        let key = "test".to_string();
        let value = "value".to_string();
        let format = format!(" {}='{}'", key, value);
        attr.add(key.to_string(), value.to_string());
        assert_eq!(format, attr.html());
    }

    #[test]
    fn single_attribute_multiple_values_to_html() {
        let mut attr = Attributes::new();
        let key = "test".to_string();
        let value = "value".to_string();
        let value2 = "it".to_string();

        let format = format!(" {}='{} {}'", key, value, value2);
        attr.add(key.to_string(), value.to_string());
        attr.add(key.to_string(), value2.to_string());
        assert_eq!(format, attr.html());
    }

    #[test]
    fn multiple_attributes_to_html() {
        let mut attr = Attributes::new();
        let key = "test".to_string();
        let key2 = "prod".to_string();
        let value = "value".to_string();
        let value2 = "it".to_string();

        let format = format!(" {}='{}' {}='{}'", key, value, key2, value2);
        attr.add(key.to_string(), value.to_string());
        attr.add(key2.to_string(), value2.to_string());
        assert_eq!(format, attr.html());

    }
    }
}
