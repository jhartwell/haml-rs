use ast::{Arena, ToHtml};

pub fn to_html(arena: &Arena) -> String {
    arena.to_html()
}