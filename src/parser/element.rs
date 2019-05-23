// use https://regexr.com/ to test regex
use regex::{Captures, Regex};
use crate::regex::{element, div, break_attributes};
use std::collections::HashMap;

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
    pub class_and_ids: Option<String>,
    pub inline_text: Option<String>,
    pub attributes: HashMap<String, Vec<String>>,
}

impl Element {
    pub fn name(&self) -> Option<String> {
        if let Some(name) = &self.name {
            Some(name[1..].to_owned())
        } else {
            None
        }
    }

    pub fn attributes(&self) -> &HashMap<String, Vec<String>> {
        &self.attributes
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
        Element {
            whitespace: Element::handle_whitespace(caps),
            name: Some("div".to_string()),
            element_type: ElementType::Div(),
            class_and_ids: Element::div_id_class(caps),
            inline_text: Element::handle_inline_text(caps),
            attributes: Element::handle_attributes(caps),
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
            Some(name) => Some(name.as_str().to_owned()),
            None => None,
        }
    }

    fn handle_class_and_ids(caps: &Captures) -> Option<String> {
        match caps.name("classid") {
            Some(class_id) => {println!("{}", class_id.as_str()); match class_id.as_str() {
                "" => None,
                s => Some(s.to_owned()),
            }},
            None => None,
        }
    }

    fn format_value(val: &str) -> String {
        let trimmed = val.trim();
        trimmed.trim()[1..trimmed.len()-1].to_owned()
    }
    fn handle_attributes(caps: &Captures) -> HashMap<String, Vec<String>> {
        let mut map : HashMap<String, Vec<String>> = HashMap::new();
        println!("{:?}", caps.name("attributes"));
        match caps.name("attributes") {
            Some(attributes) => match attributes.as_str() {
                "" => map,
                s => {
                    let r = Regex::new(&break_attributes()).unwrap();
                    let m: HashMap<String, String> = HashMap::new();
                    if r.is_match(s) {
                        for attr in r.find_iter(s) {
                            if attr.as_str().len() != s.len() {
                                let mut attr_it = attr.as_str().split("=>");
                                let id = attr_it.next();
                                let val = attr_it.next();
                                match (id, val) {
                                    (_, None) => continue,
                                    (None, _) => continue,
                                    (Some(i),Some(v)) => {
                                        if let Some(current_val) = map.get_mut(&i[1..].trim().to_owned()) {
                                            (*current_val).push(Element::format_value(v));
                                        } else {
                                            map.insert(i[1..].to_owned(), vec![Element::format_value(v)]);
                                        }
                                    },
                                }
                            }
                        }
                    }
                    map
                }
            },
            None => map,
        }
    }

    fn handle_inline_text(caps: &Captures) -> Option<String> {
        match caps.name("text") {
            Some(text) => Some(text.as_str().trim().to_owned()),
            None => None,
        }
    }

    fn create_element<'a>(caps: &'a Captures) -> Element {
        Element {
            whitespace: Element::handle_whitespace(caps),
            name: Element::handle_name(caps),
            element_type: ElementType::Other(),
            class_and_ids: Element::handle_class_and_ids(caps),
            inline_text: Element::handle_inline_text(caps),
            attributes: Element::handle_attributes(caps),
        }
    }

    pub fn from_string(haml: &str) -> Option<Element> {
        let element_regex = Regex::new(&element()).unwrap();
        let div_regex = Regex::new(&div());
    
        let element: Option<Element> = match Regex::new(&element()) {
            Ok(el) => {
                match el.is_match(haml) {
                    true => {
                        let caps = el.captures(haml).unwrap();
                        Some(Element::create_element(&caps))
                    },
                    false => None
                }
            },
            _ => None
        };

        match element {
            Some(el) => Some(el),
            None => {
                match div_regex {
                    Ok(el) => {
                        match el.is_match(haml) {
                            true => {
                                let caps = el.captures(haml).unwrap();
                                Some(Element::create_div(&caps))
                            },
                            false => None
                        }
                    },
                    _ => None
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn whitespace_counts() {
        let haml = "    %hi";
        let el = Element::from_string(haml).unwrap();
        assert_eq!(4, el.whitespace);
    }
    #[test]
    fn basic_element() {
        let t = "%hi";
        let el = Element::from_string(t).unwrap();
        println!("basic_element");
        assert_eq!(0, el.whitespace);
        assert_eq!(Some("%hi".to_owned()), el.name);
    }

    #[test]
    fn element_with_single_class() {
        let t = "%hi.box";
        let el = Element::from_string(t).unwrap();
        println!("element_with_single_class");
        assert_eq!(0, el.whitespace);
        assert_eq!(Some("%hi".to_owned()), el.name);
        assert_eq!(Some(".box".to_owned()), el.class_and_ids);
    }

    #[test]
    fn element_with_two_classes() {
        let t = "%hi.box.top";
        let el = Element::from_string(t).unwrap();
        println!("element_with_two_classes");
        assert_eq!(Some("%hi".to_owned()), el.name);
        assert_eq!(Some(".box.top".to_owned()), el.class_and_ids);
    }

    #[test]
    fn element_with_id() {
        let t = "%hi#there";
        let el = Element::from_string(t).unwrap();
        println!("element_with_id");
        assert_eq!(Some("%hi".to_owned()), el.name);
        assert_eq!(Some("#there".to_owned()), el.class_and_ids);
    }

    #[test]
    fn element_with_id_and_class() {
        let id_then_class = "%hi#there.box";
        let id_then_class_el = Element::from_string(id_then_class).unwrap();
        assert_eq!(0, id_then_class_el.whitespace);
        assert_eq!(Some("%hi".to_owned()), id_then_class_el.name);
        assert_eq!(
            Some("#there.box".to_owned()),
            id_then_class_el.class_and_ids
        );

        let class_then_id = "%hi.box#there";
        let class_then_id_element = Element::from_string(class_then_id).unwrap();;
        assert_eq!(0, class_then_id_element.whitespace);
        assert_eq!(Some("%hi".to_owned()), class_then_id_element.name);
        assert_eq!(ElementType::Other(), class_then_id_element.element_type);
        assert_eq!(
            Some(".box#there".to_owned()),
            class_then_id_element.class_and_ids
        );

        let class_then_id_then_class = "%hi.box#there.modal";
        let class_then_id_then_class_element =
            Element::from_string(class_then_id_then_class).unwrap();;
        assert_eq!(0, class_then_id_then_class_element.whitespace);
        assert_eq!(
            Some("%hi".to_owned()),
            class_then_id_then_class_element.name
        );
        assert_eq!(
            ElementType::Other(),
            class_then_id_then_class_element.element_type
        );
        assert_eq!(
            Some(".box#there.modal".to_owned()),
            class_then_id_then_class_element.class_and_ids
        );

        let id_then_class_class = "%hi#there.box.modal";
        let id_then_class_class_element = Element::from_string(id_then_class_class).unwrap();;
        assert_eq!(0, id_then_class_class_element.whitespace);
        assert_eq!(Some("%hi".to_owned()), id_then_class_class_element.name);
        assert_eq!(
            ElementType::Other(),
            id_then_class_class_element.element_type
        );
        assert_eq!(
            Some("#there.box.modal".to_owned()),
            id_then_class_class_element.class_and_ids
        );
    }

    #[test]
    fn element_with_text_after() {
        let basic_element = "%hi value";
        let element = Element::from_string(basic_element).unwrap();;
        println!("element_with_text_after");
        assert_eq!(0, element.whitespace);
        assert_eq!(Some("%hi".to_owned()), element.name);
        assert_eq!(ElementType::Other(), element.element_type);
        assert_eq!(None, element.class_and_ids);
        assert_eq!(Some("value".to_owned()), element.inline_text);
    }

    #[test]
    fn element_with_single_attribute() {
        let haml = "%hi{:id = 'me'}";
        let element = Element::from_string(haml).unwrap();;
        println!("element_with_single_attribute");
        assert_eq!(0, element.whitespace);
        assert_eq!(Some("%hi".to_owned()), element.name);
        assert_eq!(ElementType::Other(), element.element_type);
        assert_eq!(None, element.class_and_ids);
        // assert_eq!(Some("{:id = 'me'}".to_owned()), element.attributes);
        assert_eq!(None, element.inline_text);
    }

    #[test]
    fn element_with_multiple_attributes() {
        let haml = "%hi{:id = 'no' :class = 'box'}";
        let element = Element::from_string(haml).unwrap();;
        println!("element_with_multiple_attributes");
        assert_eq!(0, element.whitespace);
        assert_eq!(Some("%hi".to_owned()), element.name);
        assert_eq!(ElementType::Other(), element.element_type);
        assert_eq!(None, element.class_and_ids);
        // assert_eq!(
        //     Some("{:id = 'no' :class = 'box'}".to_owned()),
        //     element.attributes
        // );
        assert_eq!(None, element.inline_text);
    }

    #[test]
    fn test_basic_id_div() {
        let haml = "#hi";
        let element = Element::from_string(haml).unwrap();;
        println!("test_basic_id_div");
        assert_eq!(0, element.whitespace);
        assert_eq!(Some("div".to_owned()), element.name);
        assert_eq!(ElementType::Div(), element.element_type);
        assert_eq!(Some("#hi".to_string()), element.class_and_ids);
        // assert_eq!(None, element.attributes);
        assert_eq!(None, element.inline_text);
    }

    #[test]
    fn test_not_beginning_of_line() {
        let haml = "ab   %hi";
        let element = Element::from_string(haml);
        assert_eq!(None, element);
    }

    #[test]
    fn test_id_and_classes() {
        let haml = "#i.b.a";
        let element = Element::from_string(haml).unwrap();
        assert_eq!(Some("div".to_string()), element.name);
        assert_eq!(Some("#i.b.a".to_string()), element.class_and_ids);
    }
}
