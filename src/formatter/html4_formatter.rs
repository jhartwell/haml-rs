use crate::arena::{Arena, ArenaItem};
use crate::formatter::HtmlFormatter;
use crate::parser::Haml;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Html4Formatter {
    self_closing_tags: HashMap<String, bool>,
}

impl HtmlFormatter for Html4Formatter {
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
                Haml::Prolog(prolog) => html.push_str(&self.prolog_to_html(prolog)),
                Haml::ConditionalComment(ws, val) => {
                    html.push_str(&self.conditional_comment_to_html(item, arena))
                }
                _ => (),
            }
        }
        html.trim().to_owned()
    }
}

impl Html4Formatter {
    pub fn new() -> Html4Formatter {
        let mut self_closing_tags: HashMap<String, bool> = HashMap::new();
        self_closing_tags.insert("meta".to_string(), true);
        Html4Formatter { self_closing_tags }
    }

    fn prolog_to_html(&self, value: &Option<String>) -> &str {
        match value {
            None => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">"#,
            Some(v) => {
                match v.to_lowercase().as_str() {
"frameset" => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01 Frameset//EN" "http://www.w3.org/TR/html4/frameset.dtd">"#,
            "strict" => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">"#,
            _ => "",
                }
            }
        }
    }

    fn item_to_html(&self, item: &ArenaItem, arena: &Arena) -> String {
        match &item.value {
            Haml::Text(text) => format!("{}\n", text.to_owned()),
            Haml::Comment(comment) => self.comment_to_html(item, arena),
            Haml::Element(_) => self.element_to_html(item, arena),
            Haml::InnerText(text) => text.to_owned(),
            Haml::Prolog(Some(prolog)) => prolog.to_owned(),
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
                    // this needs to be separated eventuallyas this is html5 specific
                    if key.trim() == "checked" && value == "true" {
                        html.push_str(" checked");
                    } else {
                        html.push_str(&format!(" {}='{}'", key, value.to_owned()));
                    }
                }
            }

            html.push('>');
            if !el.self_close && !self.self_closing_tags.contains_key(&el.name().unwrap()) {
                if let Some(text) = &el.inline_text {
                    html.push_str(&format!("{}", text.trim()));
                }
                if item.children.len() > 0 {
                    let mut index = 0;
                    if Some("pre".to_owned()) != el.name()
                        && Some("textarea".to_owned()) != el.name()
                    {
                        html.push('\n');
                    }
                    for c in item.children.iter() {
                        let i = arena.item(*c);
                        html.push_str(&self.item_to_html(i, arena));
                    }
                }
                if Some("pre".to_owned()) == el.name() || Some("textarea".to_owned()) == el.name() {
                    html = html.trim_end().to_owned();
                }
                if Some("input".to_owned()) != el.name() {
                    html.push_str(&format!("</{}>\n", el.name().unwrap()));
                }
            }
        }
        html
    }
}
