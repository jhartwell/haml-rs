use crate::arena::{Arena, ArenaItem};
use crate::formatter::HtmlFormatter;
use crate::parser::Haml;

#[derive(Debug)]
pub struct XHtmlFormatter;

impl HtmlFormatter for XHtmlFormatter {
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
                _ => (),
            }
        }
        html.trim().to_owned()
    }
}

impl XHtmlFormatter {
    pub fn new() -> XHtmlFormatter {
        XHtmlFormatter {}
    }

    fn prolog_to_html(&self, value: &Option<String>) -> &str {
        match value {
            None =>r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">"#,
            Some(val) => {
        
        match val.as_ref() {
            "strict" => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd">"#,
            "frameset" => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Frameset//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd">"#,
            "5" => "<!DOCTYPE html>",
            "1.1" => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">"#,
            "basic" => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML Basic 1.1//EN" "http://www.w3.org/TR/xhtml-basic/xhtml-basic11.dtd">"#,
            "mobile" => r#"<!DOCTYPE html PUBLIC "-//WAPFORUM//DTD XHTML Mobile 1.2//EN" "http://www.openmobilealliance.org/tech/DTD/xhtml-mobile12.dtd">"#,
            "rdfa" => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML+RDFa 1.0//EN" "http://www.w3.org/MarkUp/DTD/xhtml-rdfa-1.dtd">"#,
            _ => ""
        }}}
    }

    fn item_to_html(&self, item: &ArenaItem, arena: &Arena) -> String {
        match &item.value {
            Haml::Text(text) => format!("{}\n", text.to_owned()),
            Haml::Comment(comment) => self.comment_to_html(item, arena),
            Haml::Element(_) => self.element_to_html(item, arena),
            Haml::InnerText(text) => text.to_owned(),
            Haml::Prolog(prolog) => self.prolog_to_html(prolog).to_string(),
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

    fn element_to_html(&self, item: &ArenaItem, arena: &Arena) -> String {
        let mut html = String::new();
        if let Haml::Element(el) = &item.value {
            html.push_str(&format!("<{}", el.name().unwrap()));
            for key in el.attributes().iter() {
                if let Some(value) = el.get_attribute(key) {
                    if key.trim() == "checked" && value == "true" {
                        html.push_str(&format!(" checked='checked'"));
                    } else {
                        html.push_str(&format!(" {}='{}'", key, value.to_owned()));
                    }
                }
            }

            if el.name() == Some("input".to_owned()) {
                html.push_str(" />");
            } else {
                html.push('>');
            }
            if !el.self_close {
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
