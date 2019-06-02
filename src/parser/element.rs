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
        Element {
            whitespace: Element::handle_whitespace(caps),
            name: Some("div".to_string()),
            element_type: ElementType::Div(),
            inline_text: Element::handle_inline_text(caps),
            attributes: attributes,
            attribute_order: order,
            self_close: Element::handle_self_close(caps),
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
        if let Some(values) = map.get_mut(key) {
            (*values).push(value.to_owned());
        } else {
            map.insert(key.to_owned(), vec![value.to_owned()]);
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
            Some(attributes) => Element::handle_attrs(
                attributes.as_str(),
                &html_attribute(),
                "=",
                &mut map,
                0,
                &mut order,
            ),
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
        Element {
            whitespace: Element::handle_whitespace(caps),
            name: Element::handle_name(caps),
            element_type: ElementType::Other(),
            inline_text: Element::handle_inline_text(caps),
            attributes: attributes,
            attribute_order: order,
            self_close: Element::handle_self_close(caps),
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

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn whitespace_counts() {
//         let haml = "    %hi";
//         let el = Element::from_string(haml).unwrap();
//         assert_eq!(4, el.whitespace);
//     }
//     #[test]
//     fn basic_element() {
//         let t = "%hi";
//         let el = Element::from_string(t).unwrap();
//         println!("basic_element");
//         assert_eq!(0, el.whitespace);
//         assert_eq!(Some("%hi".to_owned()), el.name);
//     }

//     #[test]
//     fn element_with_single_class() {
//         let t = "%hi.box";
//         let el = Element::from_string(t).unwrap();
//         println!("element_with_single_class");
//         assert_eq!(0, el.whitespace);
//         assert_eq!(Some("%hi".to_owned()), el.name);
//         assert_eq!(Some(".box".to_owned()), el.class_and_ids);
//     }

//     #[test]
//     fn element_with_two_classes() {
//         let t = "%hi.box.top";
//         let el = Element::from_string(t).unwrap();
//         println!("element_with_two_classes");
//         assert_eq!(Some("%hi".to_owned()), el.name);
//         assert_eq!(Some(".box.top".to_owned()), el.class_and_ids);
//     }

//     #[test]
//     fn element_with_id() {
//         let t = "%hi#there";
//         let el = Element::from_string(t).unwrap();
//         println!("element_with_id");
//         assert_eq!(Some("%hi".to_owned()), el.name);
//         assert_eq!(Some("#there".to_owned()), el.class_and_ids);
//     }

//     #[test]
//     fn element_with_id_and_class() {
//         let id_then_class = "%hi#there.box";
//         let id_then_class_el = Element::from_string(id_then_class).unwrap();
//         assert_eq!(0, id_then_class_el.whitespace);
//         assert_eq!(Some("%hi".to_owned()), id_then_class_el.name);
//         assert_eq!(
//             Some("#there.box".to_owned()),
//             id_then_class_el.class_and_ids
//         );

//         let class_then_id = "%hi.box#there";
//         let class_then_id_element = Element::from_string(class_then_id).unwrap();;
//         assert_eq!(0, class_then_id_element.whitespace);
//         assert_eq!(Some("%hi".to_owned()), class_then_id_element.name);
//         assert_eq!(ElementType::Other(), class_then_id_element.element_type);
//         assert_eq!(
//             Some(".box#there".to_owned()),
//             class_then_id_element.class_and_ids
//         );

//         let class_then_id_then_class = "%hi.box#there.modal";
//         let class_then_id_then_class_element =
//             Element::from_string(class_then_id_then_class).unwrap();;
//         assert_eq!(0, class_then_id_then_class_element.whitespace);
//         assert_eq!(
//             Some("%hi".to_owned()),
//             class_then_id_then_class_element.name
//         );
//         assert_eq!(
//             ElementType::Other(),
//             class_then_id_then_class_element.element_type
//         );
//         assert_eq!(
//             Some(".box#there.modal".to_owned()),
//             class_then_id_then_class_element.class_and_ids
//         );

//         let id_then_class_class = "%hi#there.box.modal";
//         let id_then_class_class_element = Element::from_string(id_then_class_class).unwrap();;
//         assert_eq!(0, id_then_class_class_element.whitespace);
//         assert_eq!(Some("%hi".to_owned()), id_then_class_class_element.name);
//         assert_eq!(
//             ElementType::Other(),
//             id_then_class_class_element.element_type
//         );
//         assert_eq!(
//             Some("#there.box.modal".to_owned()),
//             id_then_class_class_element.class_and_ids
//         );
//     }

//     #[test]
//     fn element_with_text_after() {
//         let basic_element = "%hi value";
//         let element = Element::from_string(basic_element).unwrap();;
//         println!("element_with_text_after");
//         assert_eq!(0, element.whitespace);
//         assert_eq!(Some("%hi".to_owned()), element.name);
//         assert_eq!(ElementType::Other(), element.element_type);
//         assert_eq!(None, element.class_and_ids);
//         assert_eq!(Some("value".to_owned()), element.inline_text);
//     }

//     #[test]
//     fn element_with_single_attribute() {
//         let haml = "%hi{:id = 'me'}";
//         let element = Element::from_string(haml).unwrap();;
//         println!("element_with_single_attribute");
//         assert_eq!(0, element.whitespace);
//         assert_eq!(Some("%hi".to_owned()), element.name);
//         assert_eq!(ElementType::Other(), element.element_type);
//         assert_eq!(None, element.class_and_ids);
//         // assert_eq!(Some("{:id = 'me'}".to_owned()), element.attributes);
//         assert_eq!(None, element.inline_text);
//     }

//     #[test]
//     fn element_with_multiple_attributes() {
//         let haml = "%hi{:id = 'no' :class = 'box'}";
//         let element = Element::from_string(haml).unwrap();;
//         println!("element_with_multiple_attributes");
//         assert_eq!(0, element.whitespace);
//         assert_eq!(Some("%hi".to_owned()), element.name);
//         assert_eq!(ElementType::Other(), element.element_type);
//         assert_eq!(None, element.class_and_ids);
//         // assert_eq!(
//         //     Some("{:id = 'no' :class = 'box'}".to_owned()),
//         //     element.attributes
//         // );
//         assert_eq!(None, element.inline_text);
//     }

//     #[test]
//     fn test_basic_id_div() {
//         let haml = "#hi";
//         let element = Element::from_string(haml).unwrap();;
//         println!("test_basic_id_div");
//         assert_eq!(0, element.whitespace);
//         assert_eq!(Some("div".to_owned()), element.name);
//         assert_eq!(ElementType::Div(), element.element_type);
//         assert_eq!(Some("#hi".to_string()), element.class_and_ids);
//         // assert_eq!(None, element.attributes);
//         assert_eq!(None, element.inline_text);
//     }

//     #[test]
//     fn test_not_beginning_of_line() {
//         let haml = "ab   %hi";
//         let element = Element::from_string(haml);
//         assert_eq!(None, element);
//     }

//     #[test]
//     fn test_id_and_classes() {
//         let haml = "#i.b.a";
//         let element = Element::from_string(haml).unwrap();
//         assert_eq!(Some("div".to_string()), element.name);
//         assert_eq!(Some("#i.b.a".to_string()), element.class_and_ids);
//     }
// }
