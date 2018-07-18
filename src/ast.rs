use std::fmt;
use std::marker::PhantomData;
use std::slice;

#[derive(Debug, PartialEq)]
pub struct Attribute {
    value: String,
    key: String,
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
    type Impl;

    fn to_owned(&self) -> Impl;
    fn tag(&self) -> &Option<String>;
    fn children(&self) -> &Vec<Box<dyn Html>>;
    fn attributes(&self) -> &Vec<Attribute>;

    fn add_child(&mut self, child: Box<dyn Html>);
    fn add_children(&mut self, children: &mut Vec<Box<dyn Html>>);
    fn add_attribute(&mut self, attribute: Attribute);
    fn add_attributes(&mut self, attributes: &mut Vec<Attribute>);

    fn to_html(&self) -> String {
        let mut html_builder = String::new();
        if let Some(tag) = self.tag() {
            html_builder.push_str(&format!("<{}", tag));

            for attr in self.attributes() {
                html_builder.push_str(&format!(" {}=\"{}\"", attr.key, attr.value));
            }

            html_builder.push('>');

            for child in self.children() {
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
    children: Vec<Box<dyn Html>>,
    attributes: Vec<Attribute>,
}

impl<'a> fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Html for Text {
    type Impl = Text;

    fn to_owned(&self) -> Text {
        Text {
            text: String::from(self.text),
            children: vec![],
            attributes: vec![],
        }
    }

    fn tag(&self) -> &Option<String> {
        &None
    }

    fn to_html(&self) -> String {
        self.text.clone()
    }

    fn children(&self) -> &Vec<Box<dyn Html>> {
        &self.children
    }

    fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
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
            children: vec![],
            attributes: vec![],
        })
    }
}

#[derive(Debug)]
pub struct Comment {
    text: String,
    attributes: Vec<Attribute>,
    children: Vec<Box<dyn Html>>,
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
            children: vec![],
            attributes: vec![],
        })
    }
}

impl Html for Comment {
    type Impl = Comment;

    fn to_owned(&self) -> Comment {
        Comment {
            text: self.text.clone(),
            children: vec![],
            attributes: vec![],
        }
    }

    fn tag(&self) -> &Option<String> {
        &None
    }

    fn to_html(&self) -> String {
        format!("<!-- {} -->", self.text)
    }

    fn children(&self) -> &Vec<Box<dyn Html>> {
        &self.children
    }

    fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
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
    children: Vec<Box<dyn Html>>,
    attributes: Vec<Attribute>,
}

impl Element {
    pub fn boxed(tag: String) -> Box<Element> {
        Box::new(Element {
            tag: Some(tag),
            children: vec![],
            attributes: vec![],
        })
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_html())
    }
}

impl Html for Element {
    type Impl = Element;

    fn to_owned(&self) -> Element {
        Element {
            tag: self.tag.clone(),
            children: self.children.iter().map(|x| x.to_owned()).collect(),
            attributes: self.attributes.clone(),

        }
    }
    fn tag(&self) -> &Option<String> {
        &self.tag
    }

    fn children(&self) -> &Vec<Box<dyn Html>> {
        &self.children
    }

    fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
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
