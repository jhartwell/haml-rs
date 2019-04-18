use macros;
use std::convert::From;
use traits;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Element(String),
    ImpliedDiv(),
    StartAttributes(),
    EndAttributes(),
    Class(String),
    Id(String),
    Whitespace(u32),
    Text(String),
    Newline(),
    Equal(),
    Quoted(),
    Arrow(),
    Slash(),
    Colon(),
}

impl Token {
    pub fn update(token: &Token, data: &str) -> Self {
        match token {
            Token::Element(_) => Token::Element(data.to_string()),
            Token::Class(_) => Token::Class(data.to_string()),
            Token::Id(_) => Token::Id(data.to_string()),
            Token::Text(_) => Token::Text(data.to_string()),
            token => token.clone(),
        }
    }
}

pub trait Html: AsAny {
    fn html(&self) -> String;
}

#[derive(AsAny)]
pub struct Element<'a> {
    children: Vec<&'a Html>,
    attributes: String,
    tag: String,
}

impl<'a> Html for Element<'a> {
    fn html(&self) -> String {
        String::new()
    }
}

#[derive(AsAny)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn boxed(text: String) -> Box<Text> {
        Box::new(Text { text })
    }
}

impl Html for Text {
    fn html(&self) -> String {
        String::new()
    }
}

impl<'a> Element<'a> {
    pub fn new(tag: &str) -> Element<'a> {
        Element {
            children: vec![],
            attributes: String::new(),
            tag: tag.to_string(),
        }
    }

    pub fn add_child(&mut self, element: Box<dyn Html>) {
        self.children.push(&(*element));
    }

    pub fn add_attributes(&mut self, attr: &str) {
        self.attributes.push_str(attr);
    }
}

impl<'a> From<Box<dyn Html>> for Element<'a> {
    fn from(html: Box<dyn Html>) -> Self {
        html.downcast::<Element>().unwrap()
    }
}
