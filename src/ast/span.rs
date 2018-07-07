use ast::{Attribute, Attributes, Element, Html};

pub struct Span {
    children: Option<Vec<Box<Html>>>,
    attributes: Option<Attributes>,
}

impl Html for Span {
    fn to_html(&self) -> String {
        let mut html_builder = String::new();
        html_builder.push_str("<span");

        if let Some(ref attributes) = self.attributes {
            for attr in attributes {
                html_builder.push_str(&format!(" {}=\"{}\"", attr.key, attr.value));
            }
        }
        html_builder.push('>');

        if let Some(ref children) = self.children {
            for child in children {
                html_builder.push_str(
                    &child.to_html()
                );
            }
        }
        html_builder.push_str("</span>");
        html_builder
    }
}

impl Element for Span {
    fn add_child(&mut self, child: Box<dyn Html>) {
        if let Some(ref mut children) = self.children {
            children.push(child);
        } else {
            self.children = Some(vec![child]);
        }
    }

    fn add_attribute(&mut self, attribute: Attribute) {
        if let Some(ref mut attributes) = self.attributes {
            attributes.push(attribute);
        } else {
            self.attributes = Some(
                vec![
                    attribute
                ]
            )
        }
    }
}

impl Span {
    pub fn new() -> Span {
        Span {
            attributes: None,
            children: None,
        }
    }

    pub fn boxed() -> Box<Span> {
        Box::new(
            Span {
                attributes: None,
                children: None,
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ::ast::Text;

    #[test]
    fn test_basic_span() {
        let span = Span::new();
        assert_eq!("<span></span>", span.to_html());
    }

    #[test]
    fn test_span_with_inner_text() {
        let expected_text = "This is a test";
        let text = Text::new(expected_text.to_string());
        let mut span = Span::new();
        span.add_child(Box::new(text));
        assert_eq!(format!("<span>{}</span>", expected_text), span.to_html());
    }

    #[test]
    fn test_span_with_child() {
        let mut parent = Span::new();
        let child = Span::boxed();
        parent.add_child(child);
        assert_eq!("<span><span></span></span>", parent.to_html());
    }
}