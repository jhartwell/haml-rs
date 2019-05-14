// use https://regexr.com/ to test regex
use super::{STRING, WHITESPACE};
use regex::{Captures, Regex};

#[derive(Debug, PartialEq, Clone)]
pub enum ElementType {
    Div(),
    Other(),
}

#[derive(Clone, Debug)]
pub struct Element {
    pub whitespace: usize,
    pub name: Option<String>,
    pub element_type: ElementType,
    pub class_and_ids: Option<String>,
    pub inline_text: Option<String>,
    pub attributes: Option<String>,
}

impl Element {
    fn create_div<'a>(caps: &'a Captures) -> Element {
        Element {
            whitespace: Element::handle_whitespace(caps),
            name: Element::handle_name(caps),
            element_type: ElementType::Div(),
            class_and_ids: Element::handle_class_and_ids(caps),
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
            Some(class_id) => match class_id.as_str() {
                "" => None,
                s => Some(s.to_owned()),
            },
            None => None,
        }
    }

    fn handle_attributes(caps: &Captures) -> Option<String> {
        match caps.name("attributes") {
            Some(attributes) => match attributes.as_str() {
                "" => None,
                s => Some(s.to_owned()),
            },
            None => None,
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
        let div_regex = Regex::new(&div()).unwrap();

        match element_regex.is_match(haml) {
            true => {
                let caps = element_regex.captures(haml).unwrap();
                Some(Element::create_element(&caps))
            }
            false => {
                let caps = div_regex.captures(haml).unwrap();
                Some(Element::create_div(&caps))
            }
        }
    }
}

fn element_name() -> String {
    format!("[%]{{1}}{}", STRING)
}

fn element_class_id() -> String {
    format!("[#|.]{{1}}\\w+")
}

fn element_text() -> String {
    r"\s+.+".to_owned()
}
fn element() -> String {
    format!(
        "(?P<ws>{})*(?P<name>{}){{1}}(?P<classid>({})*)(?P<attributes>({}){{0,1}})(?P<text>{})*",
        WHITESPACE,
        element_name(),
        element_class_id(),
        attributes(),
        element_text()
    )
}

fn div() -> String {
    format!(
        "(?P<ws>{})*(?P<name>{}){{1}}(?P<classid>({})*)(?P<attributes>({}){{0,1}})(?P<text>{})*",
        WHITESPACE,
        element_class_id(),
        element_class_id(),
        attributes(),
        element_text()
    )
}

fn attributes() -> String {
    "[{{]((\\s*[:]\\w+){1}\\s*[=]\\s*[']\\w*[']\\s*)+[}}]".to_owned()
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
        assert_eq!(Some("{:id = 'me'}".to_owned()), element.attributes);
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
        assert_eq!(
            Some("{:id = 'no' :class = 'box'}".to_owned()),
            element.attributes
        );
        assert_eq!(None, element.inline_text);
    }

    #[test]
    fn test_basic_id_div() {
        let haml = "#hi";
        let element = Element::from_string(haml).unwrap();;
        println!("test_basic_id_div");
        assert_eq!(0, element.whitespace);
        assert_eq!(Some("#hi".to_owned()), element.name);
        assert_eq!(ElementType::Div(), element.element_type);
        assert_eq!(None, element.class_and_ids);
        assert_eq!(None, element.attributes);
        assert_eq!(None, element.inline_text);
    }
}
