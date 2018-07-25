use ast::{Arena, ToHtml};

pub struct Generator {}

impl Generator {
    pub fn to_html(arena: &Arena) -> String {
        arena.to_html()
    }
}
