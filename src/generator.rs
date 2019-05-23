use crate::parser::element::Element;
use crate::parser::{Arena, ArenaItem, Haml};
use std::collections::HashMap;

pub struct Generator<'a> {
    arena: &'a Arena,
}

fn arena_item_to_html(item: &ArenaItem) -> String {
    let mut html = String::new();
    let mut d: HashMap<String, String> = HashMap::new();
    if let Haml::Element(el) = &item.value {
        let name = el.name().unwrap();
        // let class_and_ids = el.class_and_ids.unwrap();
    }

    html
}

impl<'a> Generator<'a> {
    pub fn new(arena: &'a Arena) -> Generator {
        Generator { arena }
    }

    pub fn to_html(&self) -> String {
        let mut html = String::new();
        let root = self.arena.root();
        for child in root.children.iter() {
            let item = &self.arena.item(*child);
            if let Haml::Element(ref el) = item.value {
                if let Some(name) = &el.name() {
                    html.push_str(&format!("<{}", name));

                    for (k, v) in el.attributes().iter() {
                        html.push_str(&format!(" {}='{}'", k.trim(), v.join(" ").trim()));
                    }

                    html.push_str(">");

                    if let Some(text) = &el.inline_text {
                        html.push_str(&format!("{} ", text));
                    }
                    for c in item.children.iter() {
                        let i = &self.arena.item(*c);
                        match &i.value {
                            Haml::Text(s) => html.push_str(&format!("{}\n",s.trim())),
                            _ => (),
                        }
                    }
                    html = html[0..html.len() - 1].to_owned(); // lazy way to get rid of the last newline for the last child
                    html.push_str(&format!("</{}>", name));
                }
            }
        }
        html
    }
}
