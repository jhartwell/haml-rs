use ast::{Attribute, Attributes, Html};

pub struct Custom {
    tag: Option<String>,
    children: Option<Vec<Box<dyn Html>>>,
    attributes: Option<Attributes>,
}

impl Html for Custom {
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

    fn tag(&self) -> &Option<String> {
        &self.tag
    }
}

impl Custom {
    pub fn new(tag: String) -> Custom {
        Custom {
            tag: Some(tag),
            attributes: None,
            children: None,
        }
    }

    pub fn boxed(tag: String) -> Box<Self> {
        Box::new(
            Custom {
                tag: Some(tag),
                attributes: None,
                children: None,
            }
        )
    }
}