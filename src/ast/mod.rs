pub mod a;
pub mod article;
pub mod aside;
pub mod custom;
pub mod details;
pub mod div;
pub mod figcaption;
pub mod figure;
pub mod footer;
pub mod header;
pub mod main;
pub mod mark;
pub mod nav;
pub mod section;
pub mod span;
pub mod summary;
pub mod time;

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

pub trait Html {
    fn tag(&self) -> &Option<String>;
    fn children(&self) -> &Option<Vec<Box<dyn Html>>>;
    fn attributes(&self) -> &Option<Attributes>;

    fn add_child(&mut self, child: Box<dyn Html>);
    fn add_attribute(&mut self, attribute: Attribute);

    fn to_html(&self) -> String {
        let mut html_builder = String::new();
        if let Some(tag) = self.tag() {
            html_builder.push_str(&format!("<{}", tag));

            if let Some(ref attributes) = self.attributes() {
                for attr in attributes {
                    html_builder.push_str(&format!(" {}=\"{}\"", attr.key, attr.value));
                }
            }
            html_builder.push('>');

            if let Some(ref children) = self.children() {
                for child in children {
                    html_builder.push_str(
                        &child.to_html()
                    );
                }
            }
            html_builder.push_str(&format!("</{}>", tag));
        }
        html_builder
    }
}

pub struct Text {
    text: String,
}

impl Html for Text {
    fn tag(&self) -> &Option<String> {
        &None
    }
  
    fn to_html(&self) -> String {
        self.text.clone()
    }

    fn children(&self) -> &Option<Vec<Box<dyn Html>>> {
        &None
    }

    fn attributes(&self) -> &Option<Attributes> {
        &None
    }

    fn add_attribute(&mut self, attribute: Attribute) {
        // do nothing as text does not allow attributes
    }

    fn add_child(&mut self, child: Box<dyn Html>) {
        // do nothing as text does not allow children
    }
}

impl Text {
    pub fn new(text: String) -> Text {
        Text {
            text,
        }
    }
}