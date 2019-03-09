use regex::Regex;
use std::collections::HashMap;

pub trait Html {
    fn html(&self) -> String;
}

pub trait Empty {
    fn empty() -> Self;
}

#[derive(Debug)]
pub struct Handler<'a> {
    pub haml: &'a str,
    last_line: &'a str,
}

#[derive(Debug)]
struct Header {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    order: Vec<String>,
}

impl Empty for Header {
        fn empty() -> Header {
        Header {
            tag: "".to_string(),
            attributes: HashMap::new(),
            order: vec![],
        }
    }
}

impl Header {
    pub fn new(tag: &str) -> Header {
        Header {
            tag: tag.to_string(),
            attributes: HashMap::new(),
            order: vec![],
        }
    }



    pub fn add_attribute(&mut self, attribute: &str, value: &str) {
        if let Some(attr) = self.attributes.get_mut(attribute) {
            *attr = format!("{} {}", attr, value);
            return;
        }
        self.attributes
            .insert(attribute.to_string(), value.to_string());
        self.order.push(attribute.to_string());
    }
}

impl Html for Header {
    fn html(&self) -> String {
        let mut html = String::new();
        html.push_str(&format!("<{}", self.tag));
        for attr in self.order.iter() {
            if let Some(val) = self.attributes.get(attr) {
                html.push_str(&format!(" {}='{}'", attr, val));
            }
        }
        html.push('>');
        html
    }
}

#[derive(Debug)]
struct Element {
    pub header: Header,
    pub children: Vec<Box<Element>>,
    pub position: u32,
}

impl Empty for Element {
    fn empty() -> Element {
        Element {
            header: Header::empty(),
            children: vec![],
            position: 0,
        }
    }
}

impl Element {
    pub fn new(tag: &str) -> Element {
        Element {
            header: Header::new(tag),
            children: vec![],
            position: 0,
        }
    }

}

impl Html for Element {
    fn html(&self) -> String {
        let mut html = String::new();
        html.push_str(&self.header.html());
        for child in &self.children {
            html.push_str(&child.html());
        }
        html.push_str(&format!("</{}>", self.header.tag));
        html
    }
}

#[derive(Clone,Copy, Debug)]
enum State {
    Element(),
    Class(),
    Id(),
    Attr(),
}

impl<'a> Handler<'a> {
    pub fn new(haml: &'a str) -> Handler {
        Handler {
            haml,
            last_line: "",
        }
    }

    pub fn parse(&mut self) -> String {
        let mut lines = self.haml.lines();
        let mut items: Vec<Element> = vec![];
        for line in lines {
            items.push(self.parse_line(line));
            self.last_line = line;
        }

        let mut html = String::new();
        for item in items {
            html.push_str(&item.html());
        }
        html
    }

    fn parse_line(&self, line: &str) -> Element {
        let mut index = 0;
        let mut whitespace_count = 0;
        let mut el: Element = Element::empty();
        for (idx, c) in line.char_indices() {
            match c {
                ' ' => whitespace_count = idx,
                c => {
                    el = self.parse_element(&line[idx..]);
                    break;
                }
            }
        }
        el
    }

    fn parse_element(&self, line: &str) -> Element {
        let first = &line[0..1];
        let mut rest = &line[1..];
        let el = match first {
            "%" => {
                let mut el = String::new();
                let mut other = String::new();
                let mut current_index = 0;
                let mut state = State::Element();
                for (idx, c) in rest.char_indices() {
                    current_index = idx;
                    match c {
                        '.' => {
                            state = State::Class();
                            break;
                        }
                        '#' => {
                            state = State::Id();
                            break;
                        }
                        '{' => {
                            state = State::Attr();
                            break;
                        }
                        '}' => {
                            state = State::Attr();
                            break;
                        }
                        '(' => {
                            state = State::Attr();
                            break;
                        }
                        ')' => {
                            state = State::Attr();
                            break;
                        }
                        ch => el.push(ch),
                    }
                }

                match state {
                    State::Element() => (),
                    _ => rest = &rest[current_index..],
                }
                let mut element = Element::new(&el);
                
                while let Some((new_state, idx)) = self.do_match(&mut element, state, rest) {
                    rest = &rest[idx..];
                    state = new_state;
                }
                element
            }
            "#" => panic!("Div-ID"),
            "." => panic!("Div-Class"),
            _ => panic!("other"),
        };
        el
    }

    fn do_match(&self, el: &mut Element, state: State, rest: &str) -> Option<(State,usize)> {
        let mut new_state = state;
        match state {
            State::Class() => {
                let mut class = String::new();
                let mut index = 0;
                for (idx, c) in rest.char_indices() {
                    match c {
                        '.' => {
                            index = idx;
                            if idx > 0 {
                                new_state = State::Class();
                                break;
                            }                    
                        },
                        '#' => {
                            index = idx;
                            new_state = State::Id();
                            break;
                        }
                        '{' => {
                            index = idx;
                            new_state = State::Attr();
                            break;
                        }
                        '}' => {
                            index = idx;
                            new_state = State::Attr();
                            break;
                        }
                        '(' => {
                            index = idx;
                            new_state = State::Attr();
                            break;
                        }
                        ')' => {
                            index = idx;
                            new_state = State::Attr();
                            break;
                        }
                        c => {
                            new_state = State::Element();
                            class.push(c);
                        }
                    }
                }
                if class.len() > 0 {
                    el.header.add_attribute("class", &class);
                }
                Some((new_state,index))
            }
            State::Id() => {
                let mut id = String::new();
                let mut index = 0;
                
                for (idx, c) in rest.char_indices() {
                    match c {
                        '.' => {
                            index = idx;
                            new_state = State::Class();
                            break;
                        }
                        '#' => {
                            index = idx;
                            if idx == 0 {
                                index = index + 1;
                            }
                            break;
                        }
                        '{' => {
                            index = idx;
                            new_state = State::Attr();
                            break;
                        }
                        '}' => {
                            index = idx;
                            new_state = State::Attr();
                            break;
                        }
                        '(' => {
                            index = idx;
                            new_state = State::Attr();
                            break;
                        }
                        ')' => {
                            index = idx;
                            new_state = State::Attr();
                            break;
                        },
                        ' ' => {
                            index = idx;
                            new_state = State::Element();
                            break;
                        }
                        c => {
                            index = idx;
                            new_state = State::Element();
                            id.push(c);
                        }
                    }
                }
                if id.len() > 0 {
                    el.header.add_attribute("id", &id);
                }
                Some((new_state,index))
            }
            _ => None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn element() {
        let tag = "div";
        let class = "class";
        let class_container = "container";
        let id = "id";
        let id_value = "part1";

        let mut el = Element::new(tag);
        el.header.add_attribute(class, class_container);
        el.header.add_attribute(id, id_value);

        let expected_html = format!(
            "<{} {}='{}' {}='{}'></{}>",
            tag, class, class_container, id, id_value, tag
        );
        assert_eq!(expected_html, el.html());
    }

    mod header {
        use super::*;

        #[test]
        fn header_attribute() {
            let mut header = Header::new("test");
            let key = "hi";
            let value = "there";

            header.add_attribute(key, value);
            assert_eq!(1, header.attributes.len());
            assert_eq!(Some(&value.to_string()), header.attributes.get(key));

            let value2 = "we";
            header.add_attribute(key, value2);
            let expected_combination = format!("{} {}", value, value2);
            assert_eq!(1, header.attributes.len());
            assert_eq!(
                Some(&expected_combination.to_string()),
                header.attributes.get(key)
            );
        }

        #[test]
        fn header_html() {
            let tag = "div";
            let key = "id";
            let value = "part";

            let mut header = Header::new(tag);
            header.add_attribute(key, value);

            assert_eq!(1, header.attributes.len());
            assert_eq!(Some(&value.to_string()), header.attributes.get(key));

            let html = format!("<{} {}='{}'>", tag, key, value);
            assert_eq!(html, header.html());
        }

        #[test]
        fn header_html_multiple_attributes() {
            let tag = "div";

            let id = "id";
            let id_value = "part";

            let class = "class";
            let class_value = "container";

            let mut header = Header::new(tag);
            header.add_attribute(id, id_value);

            assert_eq!(1, header.attributes.len());
            assert_eq!(Some(&id_value.to_string()), header.attributes.get(id));

            header.add_attribute(class, class_value);
            assert_eq!(2, header.attributes.len());
            assert_eq!(Some(&class_value.to_string()), header.attributes.get(class));

            let html = format!(
                "<{} {}='{}' {}='{}'>",
                tag, id, id_value, class, class_value
            );
            assert_eq!(html, header.html());
        }

        #[test]
        fn one_attribute_with_multiple_values() {
            let tag = "div";

            let class = "class";
            let class_container = "container";
            let class_box = "box";

            let mut header = Header::new(tag);
            header.add_attribute(class, class_container);

            assert_eq!(1, header.attributes.len());
            assert_eq!(
                Some(&class_container.to_string()),
                header.attributes.get(class)
            );

            header.add_attribute(class, class_box);
            assert_eq!(1, header.attributes.len());
            assert_eq!(
                Some(&format!("{} {}", class_container, class_box)),
                header.attributes.get(class)
            );
        }
    }

    mod testing {
        use super::*;

        #[test]
        fn test() {
            let haml = "%test";
            let mut handler = Handler::new(haml);
            let html = handler.parse();
            assert!(false);
        }
    }

    mod handler {
        use super::*;
        mod empty {
            use super::*;

            #[test]
            fn basic() {
                let haml = "%test";
                let html = "<test></test>";

                let mut handler = Handler::new(haml);
                assert_eq!(html, handler.parse());
            }

            #[test]
            fn basic_class() {
                let haml = "%test.box";
                let html = "<test class='box'></test>";
                let mut handler = Handler::new(haml);
                assert_eq!(html, handler.parse());
            }

            #[test]
            fn basic_multiple_classes() {
                let haml = "%test.box.container";
                let html = "<test class='box container'></test>";
                let  mut handler = Handler::new(haml);
                assert_eq!(html, handler.parse());
            }

            #[test]
            fn basic_id() {
                let haml = "%test#level";
                let html = "<test id='level'></test>";
                let  mut handler = Handler::new(haml);
                assert_eq!(html, handler.parse());
            }

            #[test]
            #[should_panic]
            fn double_id() {
                let haml = "%test#level#two";
                let mut handler = Handler::new(haml);
                handler.parse();
            }

            #[test]
            #[ignore]
            fn basic_id_class() {
                let haml = "%test.box.container";
                let html = "<test class='box container'></test>";
                let  mut handler = Handler::new(haml);
                assert_eq!(html, handler.parse());
            }

            #[test]
            #[ignore]
            fn div_id() {
                let haml = "#test";
                let html = "<div id='test'></div>";

                let mut handler = Handler::new(haml);
                assert_eq!(html, handler.parse());
            }
        }
    }
}
// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn xhtml_transitional() {
//         let re = Regex::new(super::xhtml_transitional).unwrap();
//         let valid_no_newline = "!!!";
//         let valid_with_space_no_newline = "!!! ";
//         let valid_with_newline = "!!!\n";
//         let valid_with_space_and_newline = "!!! \n";

//         let invalid_starting_space = " !!!";
//         let invalid_extra_characters = "!!! abc";

//         assert!(re.is_match(valid_no_newline));
//         assert!(re.is_match(valid_with_space_no_newline));
//         assert!(re.is_match(valid_with_newline));
//         assert!(re.is_match(valid_with_space_and_newline));

//         // Invalid
//         assert!(!re.is_match(invalid_starting_space));
//         assert!(!re.is_match(invalid_extra_characters));
//     }
//     #[test]
//     fn strict_doctype() {
//         let re = Regex::new(super::strict_doctype).unwrap();
//         let valid_no_newline = "!!! Strict";
//         let valid_with_newline = "!!! Strict\n";
//         let valid_with_space_and_newline = "!!! Strict  \n";

//         let invalid_no_space = "!!!Strict";
//         let invalid_starting_space = " !!! Strict";

//         // Valid
//         assert!(re.is_match(valid_no_newline));
//         assert!(re.is_match(valid_with_newline));
//         assert!(re.is_match(valid_with_space_and_newline));

//         // Invalid
//         assert!(!re.is_match(invalid_no_space));
//         assert!(!re.is_match(invalid_starting_space));
//     }
// }
