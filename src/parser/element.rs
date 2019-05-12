// use https://regexr.com/ to test regex
use super::{STRING, WHITESPACE};
use regex::Regex;

pub struct Element {
    pub whitespace: usize,
    pub name: Option<String>,
    pub class_and_ids: Option<String>,
    pub inline_text: Option<String>,

}

impl Element {
    pub fn from_string(t: &str) -> Element {
        let r = Regex::new(&Element::element()).unwrap();
        let captures = r.captures(t).unwrap();
        let mut whitespace = 0;
        let mut name: Option<String> = None;
        let mut class_and_ids: Option<String> = None;
        let mut inline_text: Option<String> = None;
        if let Some(ws) = captures.get(1) {
            whitespace = ws.as_str().len();
        }

        if let Some(element_name) = captures.get(2) {
            name = Some(element_name.as_str().to_owned());
        }

        if let Some(class_ids) = captures.get(3) {
            class_and_ids = Some(class_ids.as_str().to_owned());
        }

        if let Some(text) = captures.get(4) {
            inline_text = Some(text.as_str().to_owned());
        }


        Element {
         whitespace,
         name,
         class_and_ids,
         inline_text,

        }
    }

 fn attribute_key() -> String {
    format!("[:]{{1}}{}", STRING)
}

 fn attribute_value() -> String {
    STRING.to_string()
}

 fn attribute() -> String {
    format!(
        "\\w*({})\\w*[=]\\w*({})",
        Element::attribute_key(),
        Element::attribute_value()
    )
}

fn attributes() -> String {
    format!("[{{]({})+[}}]", Element::attribute())
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
    format!("({})*({}){{1}}(({})*)({})*", WHITESPACE, Element::element_name(),
     Element::element_class_id(), 
     Element::element_text() )
}
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_element() {
        let t = "%hi";
        let el = Element::from_string(t);
        assert_eq!(0, el.whitespace);
        assert_eq!(Some("%hi".to_owned()), el.name);
    }


    #[test]
    fn element_with_single_class() {
        let t = "%hi.box";
        let el = Element::from_string(t);

        assert_eq!(0, el.whitespace);
        assert_eq!(Some("%hi".to_owned()), el.name);
        assert_eq!(Some(".box".to_owned()), el.class_and_ids);
    }

    #[test]
    fn element_with_two_classes() {
        let t = "%hi.box.top";
        let el = Element::from_string(t);
        assert_eq!(Some("%hi".to_owned()), el.name);
        assert_eq!(Some(".box.top".to_owned()), el.class_and_ids);
    }

    #[test]
    fn element_with_id() {
        let t = "%hi#there";
        let el = Element::from_string(t);
        assert_eq!(Some("%hi".to_owned()), el.name);
        assert_eq!(Some("#there".to_owned()), el.class_and_ids);
    }
    
    #[test]
    fn element_with_id_and_class() {
        let id_then_class = "%hi#there.box";
        let id_then_class_el = Element::from_string(id_then_class);
        assert_eq!(0, id_then_class_el.whitespace);
        assert_eq!(Some("%hi".to_owned()), id_then_class_el.name);
        assert_eq!(Some("#there.box".to_owned()), id_then_class_el.class_and_ids);

        let class_then_id = "%hi.box#there";
        let class_then_id_element = Element::from_string(class_then_id);
        assert_eq!(0, class_then_id_element.whitespace);
        assert_eq!(Some("%hi".to_owned()), class_then_id_element.name);
        assert_eq!(Some(".box#there".to_owned()), class_then_id_element.class_and_ids);

        let class_then_id_then_class = "%hi.box#there.modal";
        let class_then_id_then_class_element = Element::from_string(class_then_id_then_class);
        assert_eq!(0, class_then_id_then_class_element.whitespace);
        assert_eq!(Some("%hi".to_owned()), class_then_id_then_class_element.name);
        assert_eq!(Some(".box#there.modal".to_owned()), class_then_id_then_class_element.class_and_ids);

        let id_then_class_class = "%hi#there.box.modal";
        let id_then_class_class_element = Element::from_string(id_then_class_class);
        assert_eq!(0,id_then_class_class_element.whitespace);
        assert_eq!(Some("%hi".to_owned()), id_then_class_class_element.name);
        assert_eq!(Some("#there.box.modal".to_owned()), id_then_class_class_element.class_and_ids);
    }

    #[test]
    fn element_with_text_after() {
        let basic_element = "%hi value";
        let element = Element::from_string(basic_element);

        assert_eq!(0, element.whitespace);
        assert_eq!(Some("%hi".to_owned()), element.name);
        assert_eq!(None, element.class_and_ids);
        assert_eq!(Some("value".to_owned()), element.inline_text);
    }
}