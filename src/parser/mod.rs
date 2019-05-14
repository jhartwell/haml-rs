mod element;
use element::{Element, ElementType};

use std::collections::HashMap;
use std::ops::Index;

pub const WHITESPACE: &str = r"\s*";
pub const STRING: &str = r"\w+";
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
            if let Some(el) = Element::from_string(line) {
                if !first_line {
                    let p_id = self.arena.from_whitespace(previous_id, el.whitespace);
                    previous_id = self.arena.insert(el.clone(), p_id);
                    // let parentItem = self.arena.item(parent);
                    // if parentItem.el.whitespace < el.whitespace {
                    //     println!("LT: {} - PWS: {} - EWS: {}", parent, parentItem.el.whitespace, el.whitespace);
                    //     self.arena.insert(el.clone(), parent);
                    // } else if parentItem.el.whitespace == el.whitespace {
                    //     let p_id = self.arena.parent(parent);
                    //     self.arena.insert(el.clone(), p_id);
                    //     parent = p_id;
                    //     println!("EQ: {}", parent);
                    // } else {
                    //     let p_id = self.arena.from_whitespace(parent, el.whitespace);
                    //     self.arena.insert(el.clone(), p_id);
                    //     parent = p_id;
                    //     println!("GT: {}", parent);
                    // }
                } else {
                    previous_id = self.arena.insert(el.clone(), 0);
                    first_line = false;
                }
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
struct ArenaItem {
    pub el: Element,
    pub parent: usize,
    pub children: Vec<usize>,
}

impl ArenaItem {
    pub fn new(el: Element, parent: usize) -> ArenaItem {
        ArenaItem {
            el,
            parent,
            children: vec![],
        }
    }
}
impl Arena {
    pub fn new() -> Arena {
        Arena { items: vec![] }
    }

    pub fn insert(&mut self, el: Element, parent: usize) -> usize {
        self.items.push(ArenaItem::new(el, parent));
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

    pub fn from_whitespace(&self, start_index: usize, ws: usize) -> usize {
        let mut idx = start_index;
        let mut parent = start_index;
        loop {
            let i = &self.items[idx];
            if i.el.whitespace < ws {
                parent = idx;
                break;
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
    fn parsed() {
        let haml = "%hi\n  .box\n    #b\n  %span";
        let mut p = Parser::new();
        let e = p.parse(haml);
        let root = e.item(0);
        let el = &root.el;
        assert_eq!(Some("%hi".to_owned()), el.name);
        assert_eq!(ElementType::Other(), el.element_type);
        assert_eq!(0, el.whitespace);

        let mut it = root.children.iter();
        let b = it.next().unwrap();
        let bel = e.item(*b);
        let el2 = &bel.el;
        assert_eq!(Some(".box".to_owned()), el2.name);
        assert_eq!(ElementType::Div(), el2.element_type);
        assert_eq!(2, el2.whitespace);

        let mut it2 = bel.children.iter();
        let c = it2.next().unwrap();
        let cel = e.item(*c);
        let el3 = &cel.el;
        assert_eq!(Some("#b".to_owned()), el3.name);
        assert_eq!(ElementType::Div(), el3.element_type);
        assert_eq!(4, el3.whitespace);

        let mut d = it.next().unwrap();
        let del = e.item(*d);
        let el4 = &del.el;
        assert_eq!(Some("%span".to_owned()), el4.name);
        assert_eq!(ElementType::Other(), el4.element_type);
        assert_eq!(2, el4.whitespace);

    }
}
