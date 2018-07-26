use common;
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
        if self.attributes.get(&key) == None {
            self.attributes.insert(key.clone(), vec![]);
        }
        if let Some(attrs) = self.attributes.get_mut(&key) {
            (*attrs).push(value);
        }
    }

    pub fn raw(&self) -> &HashMap<String, Vec<String>> {
        &self.attributes
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
                    _ => {
                        "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">".to_string()
                    }
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
}

pub struct Arena {
    nodes: Vec<Node>,
}

impl Arena {
    pub fn new() -> Arena {
        Arena { nodes: vec![] }
    }

    pub fn add_child(&mut self, child_id: usize, parent_id: usize) {
        self.nodes[parent_id].children.push(child_id);
        self.nodes[child_id].parent = parent_id;
    }

    pub fn add_sibling(&mut self, current_id: usize, sibling_id: usize) {
        self.nodes[current_id].next_sibling = Some(sibling_id);
        self.nodes[sibling_id].previous_sibling = Some(current_id);
    }

    pub fn parent(&self, id: usize) -> usize {
        if self.nodes.len() > 0 {
            self.nodes[id].parent
        } else {
            0
        }
    }

    pub fn new_node(&mut self, data: Html) -> usize {
        let next_index = self.nodes.len();
        self.nodes.push(Node {
            parent: 0,
            children: vec![],
            previous_sibling: None,
            next_sibling: None,
            data,
        });

        next_index
    }

    pub fn node_at(&self, id: usize) -> &Node {
        &self.nodes[id]
    }

    fn node_to_html(&self, id: usize) -> String {
        let mut html_builder = String::new();

        let node = self.node_at(id);
        match node.data {
            Html::Element(ref ele) => {
                html_builder.push_str(&format!("<{}", ele.tag()));
                for (key, value) in ele.attributes().raw().iter() {
                    html_builder.push_str(&format!(" {}='{}'", key, value.join(" ")));
                }
                html_builder.push_str(&format!(">{}", common::newline()));

                for child_id in node.children() {
                    html_builder.push_str(&format!("{}", self.node_to_html(*child_id)));
                }

                html_builder.push_str(&format!("</{}>{}", ele.tag(), common::newline()));
            }
            ref data => html_builder.push_str(&format!("{}{}", data.to_html(), common::newline())),
        }
        if let Some(sibling_id) = node.next_sibling() {
            html_builder.push_str(&format!("{}", self.node_to_html(sibling_id)));
        }

        html_builder
    }
}

impl ToHtml for Arena {
    fn to_html(&self) -> String {
        if self.nodes.len() > 0 {
            self.node_to_html(0)
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug)]
pub struct Node {
    parent: usize,
    previous_sibling: Option<usize>,
    next_sibling: Option<usize>,
    children: Vec<usize>,
    data: Html,
}

impl Node {
    pub fn next_sibling(&self) -> Option<usize> {
        self.next_sibling
    }

    pub fn children(&self) -> &Vec<usize> {
        &self.children
    }
}
