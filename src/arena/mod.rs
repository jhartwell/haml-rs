use crate::formatter::HtmlFormatter;
use crate::parser::Haml;

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
        Arena {
            items: vec![ArenaItem::new(Haml::Root(), 0)],
        }
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
            match &i.value {
                Haml::Element(ref el) => {
                    if el.whitespace < ws {
                        parent = idx;
                        break;
                    }
                }
                Haml::SilentComment(whitespace) => {
                    if *whitespace < ws {
                        parent = idx;
                        break;
                    }
                }
                _ => idx = i.parent,
            }
            // if let Haml::Element(el) = &i.value {
            //     if el.whitespace < ws {
            //         parent = idx;
            //         break;
            //     }
            // }
            // idx = i.parent;
        }
        parent
    }

    // pub fn to_html(&self) -> String {
    //     self.formatter.generate(self)
    //     let mut html = String::new();
    //     let root = self.root();
    //     for child in root.children.iter() {
    //         let item = self.item(*child);
    //         match item.value {
    //             Haml::SilentComment(_) => (),
    //             _ => html.push_str(&self.formatter.generate(item))
    //         }
    //         // html.push_str(&self.item_to_html(self.item(*child)));
    //     }
    //     html.trim().to_owned()
    // }

    // fn item_to_html(&self, item: &ArenaItem) -> String {
    //     match &item.value {
    //         Haml::Text(text) => format!("{}\n",text.to_owned()),
    //         Haml::Comment(comment) => self.comment_to_html(&item),
    //         Haml::Element(_) => self.element_to_html(&item),
    //         Haml::InnerText(text) => text.to_owned(),
    //         Haml::Prolog(Some(prolog)) => prolog.to_owned(),
    //         _ => String::new(),
    //     }
    // }

    // fn comment_to_html(&self, item: &ArenaItem) -> String {
    //     let mut html = String::new();
    //     if let Haml::Comment(line) = &item.value {
    //         html.push_str(&format!("<!--{}", line));
    //     }
    //     if item.children.len() > 0 {
    //         html.push('\n');
    //     } else {
    //         html.push(' ');
    //     }
    //     for child in item.children.iter() {
    //         let item = self.item(*child);
    //         html.push_str(&self.item_to_html(item));
    //     }
    //     html.push_str("-->");
    //     html
    // }
    // fn element_to_html(&self, item: &ArenaItem) -> String {
    //     if let Haml::Element(el) = &item.value {

    //         html.push_str(&format!("<{}", el.name().unwrap()));
    //         for key in el.attributes().iter() {
    //             if let Some(value) = el.get_attribute(key) {
    //                 // this needs to be separated eventuallyas this is html5 specific
    //                 if key.trim() == "checked" {
    //                     match &self.format {
    //                         Format::Html5() => if value == "true" {
    //                             html.push_str(" checked");
    //                         },
    //                         _ => match value.as_ref() {
    //                             "true" => html.push_str(" checked='checked'"),
    //                             _ => html.push_str(&format!(" checked='{}'", value)),
    //                         }
    //                     }
    //                 } else {
    //                     html.push_str(&format!(" {}='{}'", key, value));
    //                 }
    //             }
    //         }

    //         if Some("input".to_owned()) == el.name() || Some("meta".to_owned()) == el.name() {
    //             match self.format {
    //                 Format::XHtml() => html.push_str(" />"),
    //                 _ => html.push('>'),
    //             }
    //         } else {
    //             html.push('>');
    //         }
    //         if !el.self_close {
    //         if let Some(text) = &el.inline_text {
    //             html.push_str(&format!("{}", text.trim()));
    //         }
    //         if item.children.len() > 0 {
    //             let mut index = 0;
    //             if Some("pre".to_owned()) != el.name() && Some("textarea".to_owned()) != el.name() {
    //                 html.push('\n');
    //             }
    //             for c in item.children.iter() {
    //                 let i = self.item(*c);
    //                 html.push_str(&self.item_to_html(i));
    //             }
    //         }
    //         if Some("pre".to_owned()) == el.name() || Some("textarea".to_owned()) == el.name() {
    //             html  =html.trim_end().to_owned();
    //         }
    //         if Some("input".to_owned()) != el.name() {
    //         html.push_str(&format!("</{}>\n", el.name().unwrap()));
    //         }
    //         }
    //     }
    //     html
    // }
}
