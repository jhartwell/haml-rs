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
            dirty: false,
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
                    self.html.push_str(&format!(" {}={}", key, value));
                }
            }

            if let Some(ref mut children) = self.children {
                for child in children {
                    self.html.push_str(&format!("  {}", child.to_html()));
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

    }
    
    #[test]
    fn create_div_with_nested_children() {

    }
}