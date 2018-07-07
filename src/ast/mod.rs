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

pub struct Comment {
    text: String,
}

impl Comment {
    pub fn new(text: String) -> Comment {
        Comment {
            text,
        }
    }

    pub fn boxed(text: String) -> Box<Comment> {
        Box::new(
            Comment {
                text,
            }
        )
    }
}

impl Html for Comment {
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

pub struct Element {
    tag: Option<String>,
    children: Option<Vec<Box<dyn Html>>>,
    attributes: Option<Attributes>,
}

impl Element {
    pub fn new(tag: String) -> Element {
        Element {
            tag: Some(tag),
            children: None,
            attributes: None,
        }
    }

    pub fn boxed(tag: String) -> Box<Element> {
        Box::new(
            Element {
                tag: Some(tag),
                children: None,
                attributes: None,
            }
        )
    }
}

impl Html for Element {
    fn tag(&self) -> &Option<String> {
        &self.tag
    }

    fn children(&self) -> &Option<Vec<Box<dyn Html>>> {
        &self.children
    }

    fn attributes(&self) -> &Option<Attributes> {
        &self.attributes
    }

    fn add_attribute(&mut self, attribute: Attribute) {
        if let Some(ref mut attributes) = self.attributes {
            attributes.push(attribute);
        } else {
            self.attributes = Some(
                vec![attribute]
            );
        }
    }

    fn add_child(&mut self, child: Box<dyn Html>) {
        if let Some(ref mut children) = self.children {
            children.push(child);
        } else {
            self.children = Some(
                vec![
                    child
                ]
            );
        }
    }  
}
