use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Attributes {
    attributes: HashMap<String, Vec<String>>,
}

impl Attributes {
    pub fn new() -> Attributes {
        Attributes {
            attributes: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, value: String) {
        if let Some(attrs) = self.attributes.get_mut(&key) {
            (*attrs).push(value);
        } else {
            self.attributes.insert(key, vec![value]);
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.attributes.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        if let Some(attr) = self.attributes.get(key) {
            Some(attr)
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        self.attributes.len()
    }
}

impl ToHtml for Attributes {
    fn to_html(&self) -> String {
        let mut html_builder = String::new();
        for key in self.attributes.keys() {
            let values = self.attributes.get(key).unwrap().join(" ");
            html_builder.push_str(&format!(" {}=\"{}\"", key, values));
        }
        html_builder
    }
}

pub trait ToHtml {
    fn to_html(&self) -> String;
}

pub struct HtmlDocument {
    nodes: Vec<Html>,
}

impl HtmlDocument {
    pub fn new() -> HtmlDocument {
        HtmlDocument {
            nodes: vec![]
        }
    }

    pub fn nodes(&self) -> &Vec<Html> {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut Vec<Html> {
        &mut self.nodes
    }
}

#[derive(Clone, Debug)]
pub enum Html {
    Comment(String),
    Text(String),
    Doctype(String),
    Element(HtmlElement),
}

impl ToHtml for Html {
    fn to_html(&self) -> String {
        match self {
            Html::Comment(text) => format!("<!-- {} -->", text),
            Html::Text(text) => text.to_string(),
            Html::Element(el) => el.to_html(),
            Html::Doctype(text) => {
                match text.to_lowercase().as_ref() {
                    "strict" => 
                    "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Strict//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd\">".to_string(),
                    "frameset" =>
                    "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Frameset//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd\">".to_string(),
                    "5" =>
                    "<!DOCTYPE html>".to_string(),
                    "1.1" =>
                    "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.1//EN\" \"http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd\">".to_string(),
                    "basic" =>
                    "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML Basic 1.1//EN\" \"http://www.w3.org/TR/xhtml-basic/xhtml-basic11.dtd\">".to_string(),
                    "mobile" =>
                    "<!DOCTYPE html PUBLIC \"-//WAPFORUM//DTD XHTML Mobile 1.2//EN\" \"http://www.openmobilealliance.org/tech/DTD/xhtml-mobile12.dtd\">".to_string(),
                    _ =>
                    "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">".to_string(),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct HtmlElement {
    tag: String,
    children: Vec<Html>,
    attributes: Attributes,
}

impl ToHtml for HtmlElement {
    fn to_html(&self) -> String {
        let mut html_builder = String::new();
        html_builder.push_str(&format!("<{}", self.tag));
        if self.attributes.size() > 0 {
            html_builder.push_str(&self.attributes.to_html());
        }
        html_builder.push('>');

        for child in self.children() {
            html_builder.push_str(&child.to_html());
        }

        html_builder.push_str(&format!("</{}>", self.tag));
        html_builder
    }
}

impl HtmlElement {
    pub fn new(tag: String) -> HtmlElement {
        HtmlElement {
            tag,
            children: vec![],
            attributes: Attributes::new(),
        }
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    pub fn children(&self) -> &Vec<Html> {
        &self.children
    }

    pub fn add_attribute(&mut self, key: String, value: String) {
        self.attributes.add(key, value);
    }

    pub fn add_child(&mut self, child: Html) {
        self.children.push(child);
    }
}