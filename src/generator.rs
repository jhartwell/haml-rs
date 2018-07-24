use ast::{Arena, ToHtml, Html};

pub struct Generator {}

impl Generator {
    pub fn to_html(arena: &Arena) -> String {
        arena.to_html()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use scanner::Scanner;
    use parser::Parser;

    fn get_html(haml: &str) -> String {
        let mut scanner = Scanner::new(haml);
        let tokens = scanner.get_tokens();
        let mut parser = Parser::new(tokens);
        let arena = parser.parse();
        Generator::to_html(arena)
    }

    #[test]
    fn test_comment() {
        let haml = "/ test";
        let html = get_html(haml);
        assert_eq!("<!-- test -->", html);
    }

    #[test]
    fn test_text() {
        let haml = "test";
        let html = get_html(haml);
        assert_eq!("test", html);
    }

    #[test]
    fn test_basic_span() {
        let haml = "%span";
        let html = get_html(haml);
        assert_eq!("<span></span>", html);
    }

    #[test]
    fn test_nested_span() {
        let haml = "%span\n  test";
        let html = get_html(haml);
        assert_eq!("<span>test</span>", html);
    }

    #[test]
    fn test_attributes_div() {
        let haml = "%div(id=\"test\")";
        let html = get_html(haml);
        assert_eq!("<div id=\"test\"></div>", html);
    }
}
