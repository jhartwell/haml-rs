use crate::parser::element::Element;
use crate::parser::{Arena, ArenaItem, Haml};
use std::collections::HashMap;

pub struct Generator<'a> {
    arena: &'a Arena,
}

impl<'a> Generator<'a> {
    pub fn new(arena: &'a Arena) -> Generator {
        Generator { arena }
    }

    pub fn to_html(&self) -> String {
        self.arena.to_html()
    }
}
