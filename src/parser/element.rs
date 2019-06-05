// use https://regexr.com/ to test regex
use crate::regex::{div, element, element_class_id, html_attribute, ruby_attribute};
use regex::{Captures, Regex};
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, PartialEq, Clone)]
pub enum ElementType {
    Div(),
    Other(),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub whitespace: usize,
    pub name: Option<String>,
    pub element_type: ElementType,
    pub inline_text: Option<String>,
    pub attributes: HashMap<String, Vec<String>>,
    pub attribute_order: BTreeSet<String>,
    pub self_close: bool,
    pub whitespace_removal: bool,
}

impl Element {
    pub fn name(&self) -> Option<String> {
        if let Some(name) = &self.name {
            Some(name.to_owned())
        } else {
            None
        }
    }

    pub fn attributes(&self) -> &BTreeSet<String> {
        &self.attribute_order
    }

    fn div_id_class(caps: &Captures) -> Option<String> {
        let mut value = String::new();
        let name = match caps.name("name") {
            Some(name) => name.as_str(),
            _ => "",
        };
        let other = match caps.name("classid") {
            Some(classid) => classid.as_str(),
            _ => "",
        };
        let output = format!("{}{}", name.trim(), other.trim());
        match output.is_empty() {
            true => None,
            _ => Some(output),
        }
    }

    fn create_div<'a>(caps: &'a Captures) -> Element {
        let (mut attributes, order) = Element::handle_attributes(caps);
        let whitespace_removal = match caps.get(0) {
            Some(s) => s.as_str().ends_with("<"),
            None => false,
        };
        Element {
            whitespace: Element::handle_whitespace(caps),
            name: Some("div".to_string()),
            element_type: ElementType::Div(),
            inline_text: Element::handle_inline_text(caps),
            attributes: attributes,
            attribute_order: order,
            self_close: Element::handle_self_close(caps),
            whitespace_removal,
        }
    }

    fn handle_self_close(caps: &Captures) -> bool {
        match caps.name("self_close") {
            Some(m) => match m.as_str() {
                "" => false,
                _ => true,
            },
            None => false,
        }
    }
    fn handle_whitespace(caps: &Captures) -> usize {
        match caps.name("ws") {
            Some(ws) => ws.as_str().len(),
            None => 0,
        }
    }

    fn handle_name(caps: &Captures) -> Option<String> {
        match caps.name("name") {
            Some(name) => {
                let mut val = name.as_str().to_owned();
                if val.starts_with("%") {
                    val = val[1..].to_owned();
                }
                Some(val.as_str().to_owned())
            }
            None => None,
        }
    }

    fn format_value(val: &str) -> String {
        match val.starts_with("\"") {
            true => val[1..val.len() - 1].to_owned(),
            false => val.to_owned(),
        }
    }

    fn add_to_map(map: &mut HashMap<String, Vec<String>>, key: &str, value: &str) {
        let mut val = value.to_string();
        if val.starts_with("'") {
            val = val[1..].to_string();
        }
        if val.ends_with("'") {
            val = val[0..val.len() - 1].to_string();
        }
        if let Some(values) = map.get_mut(key) {
            match key {
                "id" => {
                    (*values).clear();
                    (*values).push(format!("id_{}", val));
                }
                _ => (*values).push(val),
            }
        } else {
            map.insert(key.to_owned(), vec![val]);
        }
    }

    // parse the attributes by hand so that atomic values are covered
    fn parse_html_attributes(
        attributes: &str,
        map: &mut HashMap<String, Vec<String>>,
        order: &mut BTreeSet<String>,
    ) {
        if !attributes.is_empty() {
            let mut seen_key = false;
            let mut step = false;
            let mut attrs = String::new();
            if attributes.starts_with("(") {
                attrs = attributes[1..].to_owned();
            }

            if attrs.ends_with(")") {
                attrs = attrs[0..attrs.len() - 1].to_owned();
            }

            let words: Vec<&str> = attrs.split(" ").collect();
            let mut words_iter = words.iter();
            let mut buffer = words_iter.next();
            let mut temp = "";
            loop {
                if let Some(wrd) = buffer {
                    let split: Vec<&str> = wrd.split("=").collect();
                    if split.len() > 1 {
                        let key = split.get(0).unwrap();
                        let value = split.get(1).unwrap();
                        Element::add_to_map(map, key, value);
                        order.insert(key.to_string());
                        buffer = words_iter.next();
                    } else {
                        if let Some(next) = words_iter.next() {
                            match *next {
                                "=" => match words_iter.next() {
                                    Some(w) => {
                                        Element::add_to_map(map, *wrd, w);
                                        order.insert((*wrd).to_owned());
                                        buffer = words_iter.next();
                                    }
                                    None => break,
                                },
                                e => {
                                    map.insert((*wrd).to_owned(), vec![]);
                                    order.insert((*wrd).to_owned());
                                    temp = e;
                                    buffer = Some(&temp);
                                }
                            }
                        } else {
                            map.insert((*wrd).to_owned(), vec![]);
                            order.insert((*wrd).to_owned());
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }
    fn handle_attrs(
        attributes: &str,
        attribute_regex: &str,
        separator: &str,
        map: &mut HashMap<String, Vec<String>>,
        start_index: usize,
        order: &mut BTreeSet<String>,
    ) {
        if !attributes.is_empty() {
            let r = Regex::new(attribute_regex).unwrap();
            if r.is_match(attributes) {
                for attr in r.find_iter(attributes) {
                    let mut attr_it = attr.as_str().split(separator);
                    let id = attr_it.next();
                    let val = attr_it.next();
                    match (id, val) {
                        (_, None) => continue,
                        (None, _) => continue,
                        (Some(i), Some(v)) => {
                            if let Some(current_val) =
                                map.get_mut(&i[start_index..].trim().to_owned())
                            {
                                (*current_val).push(Element::format_value(v));
                            } else {
                                order.insert(i[start_index..].to_owned());
                                map.insert(
                                    i[start_index..].to_owned(),
                                    vec![Element::format_value(v)],
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    fn handle_attributes(caps: &Captures) -> (HashMap<String, Vec<String>>, BTreeSet<String>) {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut order: BTreeSet<String> = BTreeSet::new();
        if let Some(c) = caps.name("name") {
            let name = c.as_str();
            if name.starts_with(".") {
                map.insert("class".to_owned(), vec![name[1..].to_owned()]);
                order.insert("class".to_owned());
            } else if name.starts_with("#") {
                map.insert("id".to_owned(), vec![name[1..].to_owned()]);
                order.insert("id".to_owned());
            }
        }

        match caps.name("classid") {
            Some(c) => {
                let class_id_reg = Regex::new(&element_class_id()).unwrap();
                let classid_value = c.as_str();
                for ci in class_id_reg.find_iter(classid_value) {
                    let value = classid_value[ci.start()..ci.end()].to_string();
                    match &value[0..1] {
                        "#" => {
                            map.insert("id".to_owned(), vec![value[1..].to_owned()]);
                            order.insert("id".to_string());
                        }
                        "." => {
                            Element::add_to_map(&mut map, "class", &value[1..]);
                            order.insert("class".to_string());
                        }
                        _ => (),
                    }
                }
            }
            None => (),
        }
        match caps.name("ruby_attributes") {
            Some(attributes) => Element::handle_attrs(
                attributes.as_str(),
                &ruby_attribute(),
                "=>",
                &mut map,
                1,
                &mut order,
            ),
            None => (),
        }
        match caps.name("html_attributes") {
            Some(attributes) => {
                Element::parse_html_attributes(attributes.as_str(), &mut map, &mut order)
            }
            None => (),
        }

        (map, order)
    }

    fn handle_inline_text(caps: &Captures) -> Option<String> {
        match caps.name("text") {
            Some(text) => Some(text.as_str().trim().to_owned()),
            None => None,
        }
    }

    fn create_element<'a>(caps: &'a Captures) -> Element {
        let (attributes, order) = Element::handle_attributes(caps);
        let whitespace_removal = match caps.get(0) {
            Some(s) => s.as_str().ends_with("<"),
            None => false,
        };
        Element {
            whitespace: Element::handle_whitespace(caps),
            name: Element::handle_name(caps),
            element_type: ElementType::Other(),
            inline_text: Element::handle_inline_text(caps),
            attributes: attributes,
            attribute_order: order,
            self_close: Element::handle_self_close(caps),
            whitespace_removal,
        }
    }

    pub fn from_string(haml: &str) -> Option<Element> {
        let element_regex = Regex::new(&element()).unwrap();
        let div_regex = Regex::new(&div());
        let element: Option<Element> = match Regex::new(&element()) {
            Ok(el) => match el.is_match(haml) {
                true => {
                    let caps = el.captures(haml).unwrap();
                    Some(Element::create_element(&caps))
                }
                false => None,
            },
            _ => None,
        };

        match element {
            Some(el) => Some(el),
            None => match div_regex {
                Ok(el) => match el.is_match(haml) {
                    true => {
                        let caps = el.captures(haml).unwrap();
                        Some(Element::create_div(&caps))
                    }
                    false => None,
                },
                _ => None,
            },
        }
    }

    pub fn get_attribute(&self, name: &str) -> Option<String> {
        if let Some(attributes) = self.attributes.get(name) {
            Some(attributes.join(" ").trim().to_owned())
        } else {
            None
        }
    }
}
