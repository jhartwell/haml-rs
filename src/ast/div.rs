/// TODO: Need to find a way to make sure that we keep the order of inner text and children that are added
/// if we add children first and then text but display text first that will mess up display
/// Thoughts: Use a vec and push each element onto the "stack" and then pull those to figure it out
/// This may require a new trait to keep the elements the same
use ast::Element;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Div {
    children: Option<Vec<Box<Element>>>,
    attributes: Option<HashMap<String, String>>,
    inner_text: Option<String>,
    dirty: bool,
    html: String,
}

impl Div {
    pub fn new() -> Div {
        Div {
            children: None,
            attributes: None,
            inner_text: None,
            dirty: true,
            html: String::new(),
        }
    }
}

impl Element for Div {

    fn add_attributes(&mut self, attributes: HashMap<String, String>) {
        if let Some(ref mut attrs) = self.attributes {
            attrs.extend(attributes);
        } else {
             self.attributes = Some(attributes);
         }
        self.dirty = true;
    }

    fn attributes(&self) -> &Option<HashMap<String, String>> {
        &self.attributes
    }

    fn add_child(&mut self, child: Box<Element>) {
        if let Some(ref mut children) = self.children {
            children.push(child);
        } else {
            self.children = Some(
                vec![
                    child,
                ]
            );
        }
    }

    fn children(&self) -> &Option<Vec<Box<Element>>> {
        &self.children
    }

    fn set_inner_text(&mut self, text: &str) {
        if let Some(ref mut inner_text) = self.inner_text {
            inner_text.push_str(text);
        } else {
            self.inner_text = Some(text.to_string());
        }
    }

    fn inner_text(&self) -> &Option<String> {
        &self.inner_text
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn to_html(&mut self) -> &str {
        if self.dirty {
            self.html.clear();
            self.html.push_str("<div");
            if let Some(ref attributes) = self.attributes {
                for (key, value) in attributes.iter() {
                    self.html.push_str(&format!(" {}=\"{}\"", key, value));
                }
            }
            self.html.push('>');
            if let Some(ref inner_text) = self.inner_text {
                self.html.push_str(&format!("\n  {}\n", inner_text));
            }
            if let Some(ref mut children) = self.children {
                self.html.push('\n');
                for child in children {
                    self.html.push_str(&format!("  {}", child.to_html()));
                    self.html.push('\n');
                }
            }

            self.html.push_str("</div>");
            self.dirty = false;
            &self.html
        }
        else {
            &self.html
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn create_basic_div() {
        let div = Div::new();
        assert_eq!(&None, div.attributes());
        assert_eq!(&None, div.children());
    }

    #[test]
    fn create_basic_div_with_child() {
        let mut div = Div::new();
        let child = Div::new();
        let expected_child_count: usize = 1;
        div.add_child(Box::new(child));
        assert_eq!(&None, div.attributes());
        if let Some(ref children) = div.children() {
            assert_eq!(expected_child_count, children.len());
        } else {
            panic!("Expected at least 1 child but found none");
        }
    }

    #[test]
    fn create_div_with_attributes() {
        let mut div = Div::new();
        let mut map = HashMap::new();
        let key = "id".to_string();
        let value = "test".to_string();
        let expected_key = key.clone();
        let expected_value = value.clone();
        map.insert(key, value);
        let expected_key_count = map.keys().len();
        div.add_attributes(map);
        if let Some(ref attributes) = div.attributes() {
            assert_eq!(expected_key_count, attributes.len());
            assert_eq!(true, attributes.contains_key(&expected_key));
            // TODO: Check that value == expected_value
        } else {
            panic!("Expected attributes to be set but there are no attributes.");
        }
        
    }

    #[test]
    fn create_div_with_attributes_and_empty_child() {
        let mut div = Div::new();
        // set up attributes
        let mut map = HashMap::new();
        let key = "id".to_string();
        let value = "test".to_string();
        let expected_key = key.clone();
        let expected_value = value.clone();
        map.insert(key, value);
        let expected_key_count = map.keys().len();
        div.add_attributes(map);

        // set up child
        let child = Div::new();
        let expected_child_count: usize = 1;
        div.add_child(Box::new(child));
        
        // assert for child
        if let Some(ref children) = div.children() {
            assert_eq!(expected_child_count, children.len());
        } else {
            panic!("Expected at least 1 child but found none");
        }

        // assert for attribues
        if let Some(ref attributes) = div.attributes() {
            assert_eq!(expected_key_count, attributes.len());
            assert_eq!(true, attributes.contains_key(&expected_key));
            // TODO: Check that value == expected_value
        } else {
            panic!("Expected attributes to be set but there are no attributes.");
        }
    }

    #[test]
    fn create_div_child_has_attributes() {
        let mut div = Div::new();
        let mut child = Div::new();
        let mut map = HashMap::new();
        let key = "id".to_string();
        let value = "test".to_string();

        let expected_size : usize = 1;
        let expected_key = key.clone();
        let expected_value = value.clone();
        map.insert(key, value);

        child.add_attributes(map);
        div.add_child(Box::new(child));

        if let Some(ref children) = div.children() {
            assert_eq!(expected_size, children.len());

            if let Some(ref actual_child) = children.get(0) {
                if let Some(ref attributes) = actual_child.attributes() {
                    assert_eq!(true, attributes.contains_key(&expected_key));
                    assert_eq!(&expected_value, attributes.get(&expected_key).unwrap());
                } else {
                    panic!("Expected attributes on child element but found none.");
                }
            } else {
                panic!("Could not get the child from the Vec of child elements.");
            }
        } else {
            panic!("Expect at least one child but found none.");
        }
    }
    
    #[test]
    fn create_div_with_nested_children() {
        let mut div = Div::new();
        let mut first_child = Div::new();
        let second_child = Div::new();

        first_child.add_child(Box::new(second_child));
        div.add_child(Box::new(first_child));

        if let Some(ref div_children) = div.children() {
            if let Some(ref first_child) = div_children.get(0) {
                if let Some(ref first_children) = first_child.children() {
                    if let Some(ref second_child) = first_children.get(0) {
                        assert_eq!(&None, second_child.attributes());
                        assert_eq!(&None, second_child.children());
                    } else {
                        panic!("Expected a second child but found none.");
                    }
                } else {
                    panic!("Expected there to be at least one child but found none");
                }
            } else {
                panic!("Expected to find a child but found none.");
            }
        } else {
            panic!("Expected children but found none.");
        }
    }

    #[test]
    fn div_html() {
        let mut div = Div::new();
        assert_eq!("<div></div>", div.to_html());
    }

    #[test]
    fn div_html_with_child() {
        let mut div = Div::new();
        let child = Div::new();
        div.add_child(Box::new(child));
        assert_eq!("<div>\n  <div></div>\n</div>", div.to_html());
    }

    #[test]
    fn div_html_with_nested_children() {
        let mut div = Div::new();
        let mut child = Div::new();
        let grandchild = Div::new();

        child.add_child(Box::new(grandchild));
        div.add_child(Box::new(child));
        assert_eq!("<div>\n  <div>\n  <div></div>\n</div>\n</div>", div.to_html());

    }

    #[test]
    fn div_html_with_attributes() {
        let mut div = Div::new();
        let mut map = HashMap::new();
        let key = "id".to_string();
        let value = "test".to_string();
        let expected_key = key.clone();
        let expected_value = value.clone();
        map.insert(key, value);
        div.add_attributes(map);

        assert_eq!(format!("<div {}=\"{}\"></div>", expected_key, expected_value), div.to_html());
    }

    #[test]
    fn div_html_no_div_attributes_child_with_attributes() {
        let mut div = Div::new();
        let mut child = Div::new();
        let mut map = HashMap::new();
        let key = "id".to_string();
        let value = "test".to_string();
        let expected_key = key.clone();
        let expected_value = value.clone();
        map.insert(key, value);
        child.add_attributes(map);
        div.add_child(Box::new(child));

        assert_eq!(format!("<div>\n  <div {}=\"{}\"></div>\n</div>", expected_key, expected_value), div.to_html());
    }

    #[test]
    fn div_inner_text() {
        let mut div = Div::new();
        let expected_text = "This is a test";
        div.set_inner_text(expected_text);
        if let Some(ref text) = div.inner_text() {
            assert_eq!(expected_text, text);
        } else {
            panic!("Expected inner text but found none.");
        }
    }

    #[test]
    fn div_inner_text_to_html() {
        let mut div = Div::new();
        let expected_text = "This is a test";
        div.set_inner_text(expected_text);
        assert_eq!(format!("<div>\n  {}\n</div>", expected_text), div.to_html());        
    }
}