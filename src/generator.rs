use ast::{Arena, ToHtml, Html};

pub struct Generator {}

impl Generator {
    pub fn to_html(arena: &Arena) -> String {
        let mut html_builder = String::new();
        if arena.len() > 0 {
            let current_index: usize = 0;
            loop {
                let node = arena.node_at(current_index);
                html_builder.push_str(&node.data().to_html());
                println!("PUSHED");
                break;
            }
        }
        html_builder
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use scanner::Scanner;
    use parser::Parser;

    #[test]
    fn test_comment() {
        let haml = "/ test";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let arena = parser.parse();
        let html = Generator::to_html(arena);
        assert_eq!("<!-- test -->", html);
    }

    #[test]
    fn test_text() {
        let haml = "test";
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let arena = parser.parse();
        let html = Generator::to_html(arena);
        assert_eq!("test", html);
    }
}
