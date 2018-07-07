pub mod span;

pub type Attributes = Vec<Attribute>;

#[derive(Debug, PartialEq)]
pub struct Attribute {
    value: String,
    key: String,
}

impl Attribute {
    pub fn new(key: String, value: String) -> Attribute {
        Attribute {
            key,
            value,
        }
    }
}

pub trait Element : Html {
    fn add_child(&mut self, Box<dyn Html>);
    fn add_attribute(&mut self, attribute: Attribute);
}

pub trait Html {
    fn to_html(&self) -> String;
}

pub struct Text {
    text: String,
}

impl Html for Text {
  
    fn to_html(&self) -> String {
        self.text.clone()
    }
}

impl Text {
    pub fn new(text: String) -> Text {
        Text {
            text,
        }
    }
}