use std::fmt;
use std::vec::IntoIter;

#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    value: String,
    key: String,
}

#[derive(Debug)]
pub struct Attributes {
    attributes: Vec<Attribute>,
}


impl IntoIterator for Attributes {
    type Item = Attribute;
    type IntoIter = ::std::vec::IntoIter<Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.attributes.into_iter()
    }
}

impl Clone for Attributes {
    fn clone(&self) -> Attributes {
        Attributes {
            attributes: self.attributes.into_iter().collect(),
        }
    }
}


impl Attributes {
    pub fn new() -> Attributes {
        Attributes {
            attributes: vec![],
        }
    }

    pub fn push(&self, attribute: Attribute) {
        self.attributes.push(attribute);
    }

    pub fn append(&self, attributes: &mut Vec<Attribute>) {
        self.attributes.append(attributes);
    }

}

#[derive(Debug)]
pub struct Children {
    nodes: Vec<Box<dyn Html>>,
}

impl Children {
    pub fn new() -> Children {
        Children {
            nodes: vec![],
        }
    }
    
    pub fn push(&self, child: Box<dyn Html>) {
        self.nodes.push(child);
    }

    pub fn append(&self, children: &mut Vec<Box<dyn Html>>) {
        self.nodes.append(children);
    }
}

impl IntoIterator for Children {
    type Item = Box<dyn Html>;
    type IntoIter = IntoIter<Box<dyn Html>>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl Attribute {
    pub fn new(key: String, value: String) -> Attribute {
        Attribute { key, value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}

pub trait Html: fmt::Display + fmt::Debug {
    fn tag(&self) -> &Option<String>;
    fn children(&self) -> Children;
    fn attributes(&self) -> Attributes;

    fn add_child(&mut self, child: Box<dyn Html>);
    fn add_children(&mut self, children: &mut Vec<Box<dyn Html>>);
    fn add_attribute(&mut self, attribute: Attribute);
    fn add_attributes(&mut self, attributes: &mut Vec<Attribute>);

    fn to_html(&self) -> String {
        let mut html_builder = String::new();
        if let Some(tag) = self.tag() {
            html_builder.push_str(&format!("<{}", tag));

            for attr in self.attributes().into_iter() {
                html_builder.push_str(&format!(" {}=\"{}\"", attr.key, attr.value));
            }

            html_builder.push('>');

            for child in self.children().into_iter() {
                html_builder.push_str(&child.to_html());
            }

            html_builder.push_str(&format!("</{}>", tag));
        }
        html_builder
    }
}

#[derive(Debug)]
pub struct Text {
    text: String,
    children: Children,
    attributes: Attributes,
}

impl<'a> fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Html for Text {
    fn tag(&self) -> &Option<String> {
        &None
    }

    fn to_html(&self) -> String {
        self.text.clone()
    }

    fn children(&self) -> Children {
        self.children.clone()
    }

    fn attributes(&self) -> Attributes {
        self.attributes.clone()
    }

    fn add_attribute(&mut self, attribute: Attribute) {
        // do nothing as text does not allow attributes
    }

    fn add_child(&mut self, child: Box<dyn Html>) {
        // do nothing as text does not allow children
    }

    fn add_attributes(&mut self, attributes: &mut Vec<Attribute>) {
        // do nothing as text does not allow attributes
    }

    fn add_children(&mut self, children: &mut Vec<Box<dyn Html>>) {
        // do nothing as text does not allow children
    }
}

impl Text {
    pub fn boxed(text: String) -> Box<Text> {
        Box::new(Text {
            text,
            children: Children::new(),
            attributes: Attributes::new(),
        })
    }
}

#[derive(Debug)]
pub struct Comment {
    text: String,
    attributes: Attributes,
    children: Children,
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Comment {
    pub fn boxed(text: String) -> Box<Comment> {
        Box::new(Comment {
            text,
            children: Children::new(),
            attributes: Attributes::new(),
        })
    }
}

impl Html for Comment {
    fn tag(&self) -> &Option<String> {
        &None
    }

    fn to_html(&self) -> String {
        format!("<!-- {} -->", self.text)
    }

    fn children(&self) -> Children {
        self.children.clone()
    }

    fn attributes(&self) -> Attributes {
        self.attributes.clone()
    }

    fn add_attribute(&mut self, attribute: Attribute) {
        // do nothing as comment does not allow attributes
    }

    fn add_attributes(&mut self, attributes: &mut Vec<Attribute>) {
        // do nothing as comment does not allow attributes
    }

    fn add_child(&mut self, child: Box<dyn Html>) {
        // do nothing as comment does not allow children
    }

    fn add_children(&mut self, children: &mut Vec<Box<dyn Html>>) {
        // do nothing as comment does not allow children
    }
}

#[derive(Debug)]
pub struct Element {
    tag: Option<String>,
    children: Children,
    attributes: Attributes,
}

impl Element {
    pub fn boxed(tag: String) -> Box<Element> {
        Box::new(Element {
            tag: Some(tag),
            children: Children::new(),
            attributes: Attributes::new(),
        })
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_html())
    }
}

impl Html for Element {
    fn tag(&self) -> &Option<String> {
        &self.tag
    }

    fn children(&self) -> Children {
        self.children.clone()
    }

    fn attributes(&self) -> Attributes {
        self.attributes.clone()
    }

    fn add_attribute(&mut self, attribute: Attribute) {
        self.attributes.push(attribute);
    }

    fn add_attributes(&mut self, attributes: &mut Vec<Attribute>) {
        self.attributes.append(attributes);
    }

    fn add_child(&mut self, child: Box<dyn Html>) {
        self.children.push(child);
    }

    fn add_children(&mut self, children: &mut Vec<Box<dyn Html>>) {
        self.children.append(children);
    }
}
