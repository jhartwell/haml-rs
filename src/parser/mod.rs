pub mod element;
use element::{Element, ElementType};
use regex::Regex;
use std::collections::HashMap;

pub const WHITESPACE: &str = r"\s*";
pub const STRING: &str = r"\w+";

pub const TEXT_REGEX: &str = r"^(\s*)[\\]";


fn text_from_string(line: &str) -> Option<String> {
    let r = Regex::new(TEXT_REGEX).unwrap();
    match r.is_match(line) {
        true => Some(line[1..].to_owned()),
        false => None,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Haml {
    Root(),
    Element(Element),
    Text(String),
    Temp(String, u32, u32),

}
pub struct Parser {
    arena: Arena,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            arena: Arena::new(),
        }
    }

    pub fn parse(&mut self, haml: &str) -> &Arena {
        let mut previous_id = 0;
        let mut first_line = true;
        for line in haml.lines() {
            println!("{:?}", text_from_string(line));
            if let Some(el) = Element::from_string(line) {
                let ws = el.whitespace;
                let element = Haml::Element(el);
                if !first_line {
                    let p_id = self.arena.from_whitespace(previous_id, ws);
                    previous_id = self.arena.insert(element, p_id);
                } else {
                    previous_id = self.arena.insert(element, 0);
                    first_line = false;
                }
            } else if let Some(text_line) = text_from_string(line) {
                println!("hit");
                self.arena.insert(Haml::Text(text_line), previous_id);
            }
        }
        &self.arena
    }
}

#[derive(Debug)]
pub struct Arena {
    items: Vec<ArenaItem>,
}

#[derive(Debug)]
pub struct ArenaItem {
    pub value: Haml,
    pub parent: usize,
    pub children: Vec<usize>,
}

impl ArenaItem {
    pub fn new(value: Haml, parent: usize) -> ArenaItem {
        ArenaItem {
            value,
            parent,
            children: vec![],
        }
    }
}

impl Arena {
    pub fn new() -> Arena {
        Arena { items: vec![ ArenaItem::new(Haml::Root(), 0)] }
    }

    pub fn insert(&mut self, haml: Haml, parent: usize) -> usize {
        self.items.push(ArenaItem::new(haml, parent));
        let idx: usize = self.items.len() - 1;
        if idx > 0 {
            self.items[parent].children.push(idx);
        }
        idx
    }

    pub fn parent(&self, i: usize) -> usize {
        self.items[i].parent
    }

    pub fn children_of(&self, i: usize) -> &Vec<usize> {
        &self.items[i].children
    }

    pub fn item(&self, i: usize) -> &ArenaItem {
        &self.items[i]
    }

    pub fn root(&self) -> &ArenaItem {
        &self.items[0]
    }

    pub fn from_whitespace(&self, start_index: usize, ws: usize) -> usize {
        let mut idx = start_index;
        let mut parent = start_index;
        loop {
            let i = &self.items[idx];
            if let Haml::Element(el) = &i.value {
                if el.whitespace < ws {
                    parent = idx;
                    break;
                }
            }
            idx = i.parent;
        }
        parent
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_text() {
        let haml = r"\= test";
        let mut p = Parser::new();
        let e = p.parse(haml);
        let id = e.root().children[0];
        let item = e.item(id);
        match &item.value {
            Haml::Text(ref text) => assert_eq!("= test".to_owned(), *text),
            _ => panic!("failed"),
        }
    }

    #[test]
    fn parse_element_text() {
        let haml = "%hi\n\\value";
        let mut p = Parser::new();
        let e = p.parse(haml);
        let id = e.root().children[0];
        let item = e.item(id);
        if let Haml::Element(el) = &item.value {
            let mut it = item.children.iter();
            match it.next() {
                Some(child_id) => {
                    let child = e.item(*child_id);
                    match &child.value {
                        Haml::Text(ref txt) => assert_eq!("value".to_owned(), *txt),
                        _ => panic!("Failed"),
                    }
                },
                None => panic!("Failed"),
            }
        }
    }

    #[test]
    fn parse_element() {
        let haml = "%hi\n  .box\n    #b\n  %span";
        let mut p = Parser::new();
        let e = p.parse(haml);
        let id = e.item(0).children[0];
        let item = e.item(id);
        let el = match &item.value {
            Haml::Element(el) => el,
            _ => panic!("failed"),
        };

        assert_eq!(Some("%hi".to_owned()), el.name);
        assert_eq!(ElementType::Other(), el.element_type);
        assert_eq!(0, el.whitespace);

        let mut it = item.children.iter();
        let b = it.next().unwrap();
        let bel = e.item(*b);
        let el2 = match &bel.value {
            Haml::Element(el) => el,
            _ => panic!("failed")
        };
        assert_eq!(Some(".box".to_owned()), el2.name);
        assert_eq!(ElementType::Div(), el2.element_type);
        assert_eq!(2, el2.whitespace);

        let mut it2 = bel.children.iter();
        let c = it2.next().unwrap();
        let cel = e.item(*c);
        let el3 = match &cel.value {
            Haml::Element(el) => el,
            _ => panic!("failed")
        };
        assert_eq!(Some("#b".to_owned()), el3.name);
        assert_eq!(ElementType::Div(), el3.element_type);
        assert_eq!(4, el3.whitespace);

        let mut d = it.next().unwrap();
        let del = e.item(*d);
        let el4 = match &del.value {
            Haml::Element(el) => el,
            _ => panic!("failed")
        };
        assert_eq!(Some("%span".to_owned()), el4.name);
        assert_eq!(ElementType::Other(), el4.element_type);
        assert_eq!(2, el4.whitespace);

    }
}
