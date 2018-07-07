mod ast;
mod tokenizer;
mod values;
mod parser;

use tokenizer::Tokenizer;

pub struct Haml {}

impl Haml {

}

#[cfg(test)]
mod test {
    use tokenizer::Tokenizer;
    use parser::Parser;

    #[test]
    fn test_haml() {
        let haml = "%test
  %span
    this is a test";
        let mut tokenizer = Tokenizer::new(&haml);
        let tokens = tokenizer.get_tokens();
        let mut parser = Parser::new(tokens);
        // let parsed = parser.parse();
    }
}