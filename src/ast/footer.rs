use ast::{Attribute, Attributes, Html};

pub struct Footer {
    children: Option<Vec<Box<dyn Html>>>,
    attributes: Option<Attributes>,
    tag: Option<String>,
}

impl Html for Footer {
    fn tag(&self) -> &Option<String> {
        &self.tag
    }

    fn add_child(&mut self, child: Box<dyn Html>) {
        if let Some(ref mut children) = self.children {
            children.push(child);
        } else {
            self.children = Some(
                vec![child]
            );
        }
    }

    fn add_attribute(&mut self, attribute: Attribute) {
        if let Some(ref mut attributes) = self.attributes {
            attributes.push(attribute);
        } else {
            self.attributes = Some(
                vec![ attribute]
            );
        }

    }

    fn children(&self) -> &Option<Vec<Box<dyn Html>>> {
        &self.children
    }

    fn attributes(&self) -> &Option<Attributes> {
        &self.attributes
    }
}

impl Footer {
    pub fn new() -> Footer {
        Footer {
            attributes: None,
            children: None,
            tag: Some("footer".to_string()),
        }
    }

    pub fn boxed() -> Box<Self> {
        Box::new(
            Footer {
                attributes: None,
                children: None,
                tag: Some("footer".to_string()),
            }
        )
    }
}