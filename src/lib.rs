mod ast;
mod generator;
mod parser;
mod tokenizer;
mod values;

use generator::Generator;
use parser::Parser;
use tokenizer::Tokenizer;

pub struct Haml {}

impl Haml {
    pub fn to_html(haml: &str) -> String {
        let mut tokenizer = Tokenizer::new(haml);
        let tokens = tokenizer.get_tokens();
        let mut parser = Parser::new(tokens);
        let parsed_values = parser.parse();
        Generator::to_html(parsed_values)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_haml() {
        let haml = "%test";
        let expected_html = "<test></test>".to_string();
        assert_eq!(expected_html, Haml::to_html(haml));
    }
    //     use tokenizer::Tokenizer;
    //     use parser::Parser;

    //     #[test]
    //     fn test_haml() {
    //         let haml = "%test
    //   %span
    //     this is a test";
    //         let mut tokenizer = Tokenizer::new(&haml);
    //         let tokens = tokenizer.get_tokens();
    //         let mut parser = Parser::new(tokens);
    //         // let parsed = parser.parse();
    //     }
}
