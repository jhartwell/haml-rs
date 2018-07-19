use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    value: String,
    key: String,
}

pub trait ToHtml {
    fn to_html(&self) -> String;
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
    attributes: Vec<Attribute>,
}

impl ToHtml for HtmlElement {
    fn to_html(&self) -> String {
        let mut html_builder = String::new();
        html_builder.push_str(&format!("<{}", self.tag));
        for attribute in self.attributes() {
            html_builder.push_str(&format!(" {}=\"{}\"", attribute.key(), attribute.value()));
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
            attributes: vec![],
        }
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }

    pub fn children(&self) -> &Vec<Html> {
        &self.children
    }

    pub fn add_attribute(&mut self, attribute: Attribute) {
        self.attributes.push(attribute);
    }

    pub fn add_attributes(&mut self, attributes: &mut Vec<Attribute>) {
        self.attributes.append(attributes);
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