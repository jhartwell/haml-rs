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

pub trait ToAst {
    fn to_ast(&self) -> String;
}

#[derive(Clone, Debug)]
pub enum Html {
    Comment(String),
    Text(String),
    Doctype(String),
    Element(HtmlElement),
}

fn doctype_lookup(doctype: &str) -> String {
    match doctype {
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

impl ToAst for Html {
    fn to_ast(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct HtmlElement {
    tag: String,
    attributes: Attributes,
}

impl HtmlElement {
    pub fn new(tag: String) -> HtmlElement {
        HtmlElement {
            tag,
            attributes: Attributes::new(),
        }
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    pub fn add_attribute(&mut self, key: String, value: String) {
        self.attributes.add(key, value);
    }
}

#[derive(Debug, Clone)]
pub struct Arena {
    nodes: Vec<Node>,
    levels: HashMap<u32, usize>,
}

impl Arena {
    pub fn new() -> Arena {
        Arena {
            nodes: vec![],
            levels: HashMap::new(),
        }
    }

    pub fn add_child(&mut self, child_id: usize, parent_id: usize) {
        self.nodes[parent_id].children.push(child_id);
        self.nodes[child_id].parent = parent_id;
    }

    pub fn add_sibling(&mut self, current_id: usize, sibling_id: usize) {
        self.nodes[current_id].next_sibling = Some(sibling_id);
        self.nodes[sibling_id].parent = self.parent(current_id);
    }

    pub fn parent(&self, id: usize) -> usize {
        if self.nodes.len() > 0 {
            self.nodes[id].parent
        } else {
            0
        }
    }

    pub fn at_indentation(&self, indent: u32) -> Option<usize> {
        if self.levels.contains_key(&indent) {
            Some(self.levels[&indent])
        } else {
            None
        }
    }

    pub fn new_node(&mut self, data: Html, indentation: u32) -> usize {
        let next_index = self.nodes.len();
        self.nodes.push(Node {
            parent: 0,
            children: vec![],
            next_sibling: None,
            data,
            indentation,
        });
        self.levels.insert(indentation, next_index);

        next_index
    }

    pub fn node_at(&self, id: usize) -> &Node {
        &self.nodes[id]
    }

    fn node_to_ast(&self, id: usize, indent: &str) -> String {
        let mut ast_builder = String::new();
        let node = self.node_at(id);
        ast_builder.push_str(&format!("{:?}", node.data));
        for child in node.children() {
            ast_builder.push_str(&format!(
                "\n{}{}",
                indent,
                self.node_to_ast(*child, &format!("{}\t", indent))
            ));
        }
        ast_builder
    }

    fn node_to_html(&self, id: usize) -> String {
        let mut html_builder = String::new();
        let node = self.node_at(id);
        match node.data {
            Html::Element(ref ele) => {
                html_builder.push_str(&format!("<{}", ele.tag()));
                for key in sort(ele.attributes().raw()) {
                    if let Some(ref value) = ele.attributes().raw().get(&key) {
                        html_builder.push_str(&format!(" {}=\'{}\'", key, value.join(" ")));
                    }
                }
                html_builder.push_str(&format!(">{}", common::newline()));

                for child_id in node.children() {
                    html_builder.push_str(&format!("{}", self.node_to_html(*child_id)));
                }

                if common::does_tag_close(&ele.tag()) {
                    html_builder.push_str(&format!("</{}>{}", ele.tag(), common::newline()));
                }
            }
            Html::Doctype(ref doctype) => {
                html_builder.push_str(&format!("{}{}", doctype_lookup(doctype), common::newline()))
            }
            Html::Comment(ref comment) => {
                html_builder.push_str(&format!("<!-- {} -->{}", comment, common::newline()))
            }
            Html::Text(ref text) => {
                html_builder.push_str(&format!("{}{}", text, common::newline()))
            }
        }
        if id == 0 {
            if let Some(sibling_id) = node.next_sibling() {
                html_builder.push_str(&format!("{}", self.node_to_html(sibling_id)));
            }
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

impl ToAst for Arena {
    fn to_ast(&self) -> String {
        if self.nodes.len() > 0 {
            self.node_to_ast(0, "")
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    parent: usize,
    next_sibling: Option<usize>,
    children: Vec<usize>,
    data: Html,
    indentation: u32,
}

impl Node {
    pub fn next_sibling(&self) -> Option<usize> {
        self.next_sibling
    }

    pub fn children(&self) -> &Vec<usize> {
        &self.children
    }
}

fn sort(map: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut v = vec![];
    for key in map.keys() {
        v.push(key.clone());
    }
    v.sort();
    v
}
