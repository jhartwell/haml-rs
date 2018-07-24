mod ast;
mod generator;
mod parser;
mod scanner;
mod values;

// use generator::Generator;
// use parser::Parser;
// use scanner::Scanner;

// pub struct Haml {}

// impl Haml {
//     pub fn to_html(haml: &str) -> String {
//         let mut scanner = Scanner::new(haml);
//         let tokens = scanner.get_tokens();
//         let mut parser = Parser::new(tokens);
//         let parsed_values = parser.parse();
//         Generator::to_html(&parsed_values)
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_haml() {
//         let haml = "%test";
//         let expected_html = "<test></test>".to_string();
//         assert_eq!(expected_html, Haml::to_html(haml));
//     }
// }
