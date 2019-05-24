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
                println!("{:?}", el);
                if let Some(name) = &el.name() {
                    html.push_str(&format!("<{}", name));

                    for key in el.attributes().iter() {
                        if let Some(value) = el.get_attribute(key) {
                            // this needs to be separated eventuallyas this is html5 specific
                            if key.trim() == "checked" && value == "true" {
                                html.push_str(" checked");
                            } else {
                                html.push_str(&format!(" {}='{}'", key.trim(), value.to_owned()));
                            }
                        }
                    }

                    html.push_str(">");
                    if name == "input" {
                        continue;
                    }
                    if let Some(text) = &el.inline_text {
                        html.push_str(&format!("{} ", text));
                    }

                    let count = item.children.len();
                    let mut index = 0;
                    for c in item.children.iter() {
                        let i = &self.arena.item(*c);
                        match &i.value {
                            Haml::Text(s) => html.push_str(&format!("{}", s.trim())),
                            Haml::Comment(c) => html.push_str(&format!("<!-- {} -->", c)),
                            _ => (),
                        }
                        index += 1;
                        if index < count {
                            html.push('\n');
                        }
                    }
                    html.push_str(&format!("</{}>", name));
                }
            }
            if let Haml::Comment(ref c) = item.value {
                html.push_str(&format!("<!-- {} -->", c));
            }
        }
        html
    }
}
