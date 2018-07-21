use ast::Html;

pub struct Generator {}

impl Generator {
    pub fn to_html(nodes: &Vec<>) -> String {
        let mut html_builder = String::new();
        for node in nodes.iter() {
            html_builder.push_str(&node.to_html());
        }
        html_builder
    }
}
