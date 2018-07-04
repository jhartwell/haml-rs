use std::collections::HashMap;

enum HtmlElement {
    Element(Element),
    Comment(Comment),
    Declaration(),
}

impl HtmlElement {
    fn to_html(&self) -> &str {
        match self {
            HtmlElement::Element(element) => element.to_html(),
            HtmlElement::Comment(comment) => comment.to_html(),
            HtmlElement::Declaration() => "",
        }
    }
}

type NodeList = Vec<HtmlElement>;

pub struct Element {
    tag: String,
    children: Option<NodeList>,
    attributes: Option<HashMap<String, String>>,
    html: String,
    is_dirty: bool,
}

pub struct Comment {
    text: String,
    is_dirty: bool,
    html: String,
}

impl Comment {
    fn new(text: String) -> Comment {
        Comment {
            text,
            is_dirty: true,
            html: String::new(),
        }
    }

    fn udpate_text(&mut self, text: String) {
        self.text = text;
        self.is_dirty = true
    }

    fn to_html(&mut self) -> &str {
        if self.is_dirty {
            self.html = format!("<!-- {} -->", self.text);
            self.is_dirty = false;
        }
        &self.html
    }
}

impl Element {
    pub fn new(tag: String) -> Element {
        Element {
            tag,
            children: None,
            attributes: None,
            is_dirty: true,
            html: String::new(),
        }
    }

    pub fn add_child(&mut self, child: HtmlElement) {
        if let Some(ref mut children) = self.children {
            children.push(child);
        } else {
            self.children = Some(vec![child]);
        }
    }

    pub fn add_attributes(&mut self, attributes: HashMap<String, String>) {
        self.attributes = Some(attributes);
    }

    pub fn attributes(&self) -> &Option<HashMap<String, String>> {
        &self.attributes
    }

    pub fn children(&self) -> &Option<NodeList> {
        &self.children
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    fn to_html(&self) -> &str {
        if self.is_dirty {
            let mut html = String::new();
            html.push_str(&format!("<{}", self.tag));
            if let Some(ref attributes) = self.attributes {
                for (key, value) in attributes {
                    html.push_str(&format!(" {}={}", key, value));
                }
            }
            html.push('>');

            if let Some(ref children) = self.children {
                for child in children {
                    html.push_str(&child.to_html());
                }
            }
            html.push_str(&format!("</{}>", self.tag));
            self.is_dirty = false;
            self.html = html;
        }
        &self.html
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_comment_with_text() {
        let text = "Test it out".to_string();
        let expected_text = text.clone();
        let comment = Comment::new(text);
        assert_eq!(format!("<!-- {} -->", expected_text), comment.to_html());
    }

    #[test]
    fn test_empty_comment() {
        let text = "".to_string();
        let expected_text = text.clone();
        let comment = Comment::new(text);
        assert_eq!(format!("<!--  -->"), comment.to_html());
    }

    #[test]
    fn test_element_with_empty_div() {
        let tag = "div".to_string();
        let expected_tag = tag.clone();
        let div = Element::new(tag);
        assert_eq!(
            format!("<{}></{}>", expected_tag, expected_tag),
            div.to_html()
        );
    }

    #[test]
    fn test_element_with_empty_div_and_attributes() {
        let tag = "div".to_string();
        let expected_tag = tag.clone();
        let mut div = Element::new(tag);

        // attributes
        let mut map = HashMap::new();
        let key = "id".to_string();
        let value = "test".to_string();
        let expected_key = key.clone();
        let expected_value = value.clone();
        map.insert(key, value);

        div.add_attributes(map);

        if let Some(attributes) = div.attributes() {
            if attributes.contains_key(&expected_key) {
                if let Some(actual_value) = attributes.get(&expected_key) {
                    assert_eq!(&expected_value, actual_value);
                }
            } else {
                panic!(format!(
                    "Expected to find key {} but key was not found.",
                    expected_key
                ));
            }
        } else {
            panic!("Expect to find attributes but found none.");
        }
    }

    #[test]
    #[should_panic]
    fn test_element_with_bad_attribute_key() {
        let tag = "div".to_string();
        let expected_tag = tag.clone();
        let mut div = Element::new(tag);

        // attributes
        let mut map = HashMap::new();
        let key = "id".to_string();
        let value = "test".to_string();
        let expected_key = "span".to_string();
        let expected_value = value.clone();
        map.insert(key, value);

        div.add_attributes(map);

        if let Some(attributes) = div.attributes() {
            if attributes.contains_key(&expected_key) {
                if let Some(actual_value) = attributes.get(&expected_key) {
                    assert_eq!(&expected_value, actual_value);
                }
            } else {
                panic!(format!(
                    "Expected to find key {} but key was not found.",
                    expected_key
                ));
            }
        } else {
            panic!("Expect to find attributes but found none.");
        }
    }

    #[test]
    fn element_add_child() {
        let expected_parent_tag = "div";
        let expected_child_tag = "span";

        let parent_tag = expected_parent_tag.to_string();
        let child_tag = expected_child_tag.to_string();
        let mut parent = Element::new(parent_tag);
        let child = Element::new(child_tag);

        parent.add_child(HtmlElement::Element(child));

        if let Some(children) = parent.children() {
            if let Some(child) = children.get(0) {
                match child {
                    HtmlElement::Element(child_element) => {
                        assert_eq!(expected_child_tag, child_element);
                        assert_eq!(expected_parent_tag, parent.tag());
                    }
                    _ => panic!("Expecting element but found something else."),
                }
            } else {
                panic!("Children is been empty.");
            }
        } else {
            panic!("Expected at least one child but found none");
        }
    }
}
