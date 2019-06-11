use crate::arena::{Arena, ArenaItem};
use crate::formatter::HtmlFormatter;
use crate::parser::Haml;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Html5Formatter {
    self_closing_tags: HashMap<String, bool>,
}

impl HtmlFormatter for Html5Formatter {
    fn generate(&self, arena: &Arena) -> String {
        let root = arena.root();
        let mut html = String::new();
        for child in &root.children {
            let item = arena.item(*child);
            match &item.value {
                Haml::SilentComment(_) => (),
                Haml::Element(_) => html.push_str(&self.element_to_html(item, arena)),
                Haml::Comment(_) => html.push_str(&self.comment_to_html(item, arena)),
                Haml::Text(text) => html.push_str(&format!("{}\n", text.to_owned())),
                Haml::InnerText(text) => html.push_str(&text),
                Haml::Prolog(Some(_)) => (),
                Haml::Prolog(None) => html.push_str("<!DOCTYPE html>"),
                Haml::ConditionalComment(_, _) => {
                    html.push_str(&self.conditional_comment_to_html(item, arena))
                }
                _ => (),
            }
            if item.children.len() == 0 && item.parent != 0 {
                html.push('\n');
            }
        }
        html.trim().to_owned()
    }
}

impl Html5Formatter {
    pub fn new() -> Html5Formatter {
        let mut self_closing_tags: HashMap<String, bool> = HashMap::new();
        self_closing_tags.insert("meta".to_string(), true);
        Html5Formatter { self_closing_tags }
    }

    fn item_to_html(&self, item: &ArenaItem, arena: &Arena) -> String {
        match &item.value {
            Haml::Text(text) => format!("{}\n", text.to_owned()),
            Haml::Comment(comment) => self.comment_to_html(item, arena),
            Haml::Element(_) => self.element_to_html(item, arena),
            Haml::InnerText(text) => format!("{}\n", text),
            Haml::Prolog(Some(prolog)) => prolog.to_owned(),
            Haml::ConditionalComment(_, _) => self.conditional_comment_to_html(item, arena),
            _ => String::new(),
        }
    }
    fn comment_to_html(&self, item: &ArenaItem, arena: &Arena) -> String {
        let mut html = String::new();
        if let Haml::Comment(line) = &item.value {
            html.push_str(&format!("<!--{}", line));
        }
        if item.children.len() > 0 {
            html.push('\n');
        } else {
            html.push(' ');
        }
        for child in item.children.iter() {
            let item = arena.item(*child);
            html.push_str(&self.item_to_html(item, arena));
        }
        html.push_str("-->");
        html
    }

    fn conditional_comment_to_html(&self, item: &ArenaItem, arena: &Arena) -> String {
        let mut html = String::new();
        if let Haml::ConditionalComment(ws, value) = &item.value {
            html.push_str(&format!("<!--[{}]>\n", value));
            for child in item.children.iter() {
                let i = arena.item(*child);
                html.push_str(&self.item_to_html(i, arena));
            }
            html.push_str("<![endif]-->")
        }
        html
    }

    fn element_to_html(&self, item: &ArenaItem, arena: &Arena) -> String {
        let mut html = String::new();
        if let Haml::Element(el) = &item.value {
            html.push_str(&format!("<{}", el.name().unwrap()));
            
            for key in el.attributes().iter() {
                if let Some(value) = el.get_attribute(key) {
                    if key.trim() == "checked" && value == "true" {
                        html.push_str(" checked");
                    } else {
                        match value.is_empty() {
                            false => html.push_str(&format!(" {}='{}'", key.trim(), value)),
                            true => html.push_str(&format!(" {}", key.trim())),
                        }
                    }
                }
            }

            html.push('>');
            if !el.self_close && !self.self_closing_tags.contains_key(&el.name().unwrap()) {
                let mut has_inline_text = false;
                if let Some(text) = &el.inline_text {
                    html.push_str(&format!("{}", text));
                    has_inline_text = true;
                }
                    
                if item.children.len() > 0 {
                    let mut index = 0;
                    if Some("pre".to_owned()) != el.name()
                        && Some("textarea".to_owned()) != el.name() && !el.whitespace_removal_inside
                    {
                        html.push('\n');
                    }
                    for c in item.children.iter() {
                        let i = arena.item(*c);
                        html.push_str(&self.item_to_html(i, arena));
                    }
                }
                if el.whitespace_removal_inside {
                    html = html.trim_end().to_string();
                }
                if Some("pre".to_owned()) == el.name() || Some("textarea".to_owned()) == el.name() {
                    html = html.trim_end().to_owned();
                }
                if Some("input".to_owned()) != el.name() {
                    html.push_str(&format!("</{}>", el.name().unwrap()));
                    if item.children.len() > 0 { 
                        if !el.whitespace_removal_outside {
                            html.push('\n');
                        }
                    } else if has_inline_text && item.parent != 0 {
                        html.push('\n');
                    } else if item.parent != 0 {
                        html.push('\n');
                    }
                }
            }
        }
        html
    }
}
