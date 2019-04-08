use crate::common::Token;
use std::collections::HashMap;

pub struct Lexer<'a> {
    haml: &'a str,
    previous_state: Option<Token>,
    current_state: Option<Token>,
    buffer: String,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(haml: &str) -> Lexer {
        Lexer {
            haml,
            previous_state: None,
            current_state: None,
            buffer: String::new(),
            tokens: vec![],
        }
    }

    fn handle_whitespace(&mut self) {
        let current_state = &self.current_state;

        match current_state {
            Some(Token::Whitespace(idx)) => self.current_state = Some(Token::Whitespace(idx + 1)),
            Some(Token::Newline()) => self.push_state(Token::Whitespace(1)),
            None => self.current_state = Some(Token::Whitespace(1)),
            _ => {
                if let Some(token) = &self.previous_state {
                    self.tokens.push(token.clone());
                }
                self.previous_state = self.current_state.clone();
                self.current_state = Some(Token::Whitespace(1));
            }
        }
    }

    fn push_state(&mut self, new_state: Token) {
        if let Some(current) = &self.current_state {
            self.tokens.push(Token::update(&current, &self.buffer));
        }
        self.buffer.clear();
        self.previous_state = self.current_state.clone();
        self.current_state = Some(new_state.clone());
    }

    fn handle_percentage(&mut self) {
        let current_state = &self.current_state;
        match current_state {
            Some(Token::Whitespace(_)) => self.push_state(Token::Element(String::new())),
            Some(Token::Text(_)) => self.buffer.push('%'),
            None => self.current_state = Some(Token::Element(String::new())),
            _ => (),
        }
    }

    fn handle_period(&mut self) {
        let current_state = &self.current_state;
        match current_state {
            Some(Token::Element(_)) => self.push_state(Token::Class(String::new())),
            Some(Token::Text(_)) => self.buffer.push('.'),
            Some(Token::Id(_)) => self.push_state(Token::Class(String::new())),
            Some(Token::Whitespace(_)) => {
                self.current_state = Some(Token::ImpliedDiv());
                self.push_state(Token::Class(String::new()));
            }
            None => {
                self.current_state = Some(Token::ImpliedDiv());
                self.push_state(Token::Class(String::new()));
            }
            _ => (),
        }
    }

    fn handle_hashtag(&mut self) {
        let current_state = &self.current_state;
        match current_state {
            Some(Token::Element(_)) => self.push_state(Token::Id(String::new())),
            Some(Token::Text(_)) => self.buffer.push('#'),
            Some(Token::Class(_)) => self.push_state(Token::Id(String::new())),
            None => {
                self.current_state = Some(Token::ImpliedDiv());
                self.push_state(Token::Id(String::new()));
            }
            _ => (),
        }
    }

    fn handle_open(&mut self, open_type: char) {
        let current_state = &self.current_state;
        match current_state {
            Some(Token::Element(_)) => self.push_state(Token::StartAttributes()),
            Some(Token::Text(txt)) => self.buffer.push(open_type),
            _ => (),
        }
    }

    fn handle_close(&mut self, close_type: char) {
        self.push_state(Token::EndAttributes());
    }

    fn handle_char(&mut self, ch: char) {
        let current_state = &self.current_state;
        match current_state {
            None => {
                self.current_state = Some(Token::Text(String::new()));
                self.buffer.push(ch);
            }
            Some(Token::Whitespace(_)) => {
                self.push_state(Token::Text(String::new()));
                self.buffer.push(ch);
            }
            Some(Token::Text(_)) => self.buffer.push(ch),
            Some(Token::Element(_)) => self.buffer.push(ch),
            Some(Token::Class(_)) => self.buffer.push(ch),
            Some(Token::Id(_)) => self.buffer.push(ch),
            _ => (),
        }
    }

    fn handle_newline(&mut self) {
        self.push_state(Token::Newline());
    }

    fn handle_equal(&mut self) {
        self.push_state(Token::Equal());
    }

    fn handle_arrow(&mut self) {
        self.push_state(Token::Arrow());
    }

    fn handle_slash(&mut self) {
        let current_state = &self.current_state;
        match current_state {
            None => self.push_state(Token::Slash()),
            Some(Token::Text(_)) => self.buffer.push('\\'),
            _ => self.push_state(Token::Slash()),
        }
    }

    fn handle_colon(&mut self) {
        self.push_state(Token::Colon());
    }

    pub fn generate(&mut self) -> &Vec<Token> {
        for ch in self.haml.chars() {
            match ch {
                ' ' => self.handle_whitespace(),
                '%' => self.handle_percentage(),
                '.' => self.handle_period(),
                '#' => self.handle_hashtag(),
                '{' => self.handle_open('{'),
                '(' => self.handle_open('('),
                ')' => self.handle_close(')'),
                '}' => self.handle_close('}'),
                '\n' => self.handle_newline(),
                '=' => self.handle_equal(),
                '>' => self.handle_arrow(),
                '\\' => self.handle_slash(),
                ':' => self.handle_colon(),
                c => self.handle_char(c),
            }
        }

        if let Some(token) = &self.current_state {
            self.tokens.push(Token::update(&token, &self.buffer));
        }
        self.current_state = None;
        &self.tokens
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let haml = "%test";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(1, tokens.len());
        assert_eq!(Token::Element("test".to_string()), tokens[0]);
    }

    #[test]
    fn basic2() {
        let haml = "%test.box";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Element("test".to_string()), tokens[0]);
        assert_eq!(Token::Class("box".to_string()), tokens[1]);
    }

    #[test]
    fn basic3() {
        let haml = "%test#fun";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Element("test".to_string()), tokens[0]);
        assert_eq!(Token::Id("fun".to_string()), tokens[1]);
    }

    #[test]
    fn basic_together() {
        let haml = "%test.box#fun";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(3, tokens.len());
        assert_eq!(Token::Element("test".to_string()), tokens[0]);
        assert_eq!(Token::Class("box".to_string()), tokens[1]);
        assert_eq!(Token::Id("fun".to_string()), tokens[2]);
    }

    #[test]
    fn basic_div_id() {
        let haml = "#fun";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(2, tokens.len());
        assert_eq!(Token::ImpliedDiv(), tokens[0]);
        assert_eq!(Token::Id("fun".to_string()), tokens[1]);
    }

    #[test]
    fn basic_div_class() {
        let haml = ".box";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(2, tokens.len());
        assert_eq!(Token::ImpliedDiv(), tokens[0]);
        assert_eq!(Token::Class("box".to_string()), tokens[1]);
    }

    #[test]
    fn basic_div_class_with_id() {
        let haml = ".box#fun";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(3, tokens.len());
        assert_eq!(Token::ImpliedDiv(), tokens[0]);
        assert_eq!(Token::Class("box".to_string()), tokens[1]);
        assert_eq!(Token::Id("fun".to_string()), tokens[2]);
    }

    #[test]
    fn basic_div_id_with_class() {
        let haml = "#fun.box";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(3, tokens.len());
        assert_eq!(Token::ImpliedDiv(), tokens[0]);
        assert_eq!(Token::Id("fun".to_string()), tokens[1]);
        assert_eq!(Token::Class("box".to_string()), tokens[2]);
    }

    #[test]
    fn whitespace() {
        let haml = " ";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(1, tokens.len());
        assert_eq!(Token::Whitespace(1), tokens[0]);
    }

    #[test]
    fn multiple_whitespace() {
        let haml = "     ";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(1, tokens.len());
        assert_eq!(Token::Whitespace(5), tokens[0]);
    }

    #[test]
    fn whitespace_element() {
        let haml = "  %span";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Whitespace(2), tokens[0]);
        assert_eq!(Token::Element("span".to_string()), tokens[1]);
    }

    #[test]
    fn whitespace_text() {
        let haml = "  hi";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Whitespace(2), tokens[0]);
        assert_eq!(Token::Text("hi".to_string()), tokens[1]);
    }

    #[test]
    fn text() {
        let haml = "test";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(1, tokens.len());
        assert_eq!(Token::Text("test".to_string()), tokens[0]);
    }

    #[test]
    fn newline() {
        let haml = "\n";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(1, tokens.len());
        assert_eq!(Token::Newline(), tokens[0]);
    }

    #[test]
    fn nested() {
        let haml = "%test\n  .box";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(5, tokens.len());
        assert_eq!(Token::Element("test".to_string()), tokens[0]);
        assert_eq!(Token::Newline(), tokens[1]);
        assert_eq!(Token::Whitespace(2), tokens[2]);
        assert_eq!(Token::ImpliedDiv(), tokens[3]);
        assert_eq!(Token::Class("box".to_string()), tokens[4]);
    }

    #[test]
    fn equal() {
        let haml = "=";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(1, tokens.len());
        assert_eq!(Token::Equal(), tokens[0]);
    }

    #[test]
    fn assignment() {
        let haml = "a = b";
        let mut lex = Lexer::new(haml);
        let tokens = lex.generate();
        assert_eq!(5, tokens.len());
        assert_eq!(Token::Text("a".to_string()), tokens[0]);
        assert_eq!(Token::Whitespace(1), tokens[1]);
        assert_eq!(Token::Equal(), tokens[2]);
        assert_eq!(Token::Whitespace(1), tokens[3]);
        assert_eq!(Token::Text("b".to_string()), tokens[4]);
    }
}
