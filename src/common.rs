use std::collections::HashMap;

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

pub trait Html {
    fn html(&self) -> String;
}

pub struct Element<'a> {
    children: Vec<&'a Html>,
    attributes: String,
    attr_map: HashMap<String, Vec<String>>,
    tag: String,
}

impl<'a> Html for Element<'a> {
    fn html(&self) -> String {
        String::new()
    }
}

pub struct Text {
    text: String,
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
            attr_map: HashMap::new(),
        }
    }

    pub fn add_child(&mut self, element: &'a impl Html) {
        self.children.push(element);
    }

    pub fn add_attributes(&mut self, key: &str, attr: &str) {
        match self.attr_map.get(key) {
            Some(val) => {
                let new_value: Vec<String> = val.as_mut_slice().collect();
                new_value.push(attr);
                self.attr_map.insert(key, new_value);
            },
            None => self.attr_map.insert(key, val),
        }
    }
}
