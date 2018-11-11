use super::HtmlFormat;
use ast::{Arena, Html, HtmlElement, Node};
use common;
use std::collections::HashMap;

type Handler = fn(&HtmlElement) -> String;
type Section = HashMap<String, SectionHandler>;

struct SectionHandler {
    pub section: String,
    pub cases: HashMap<String, Handler>,
}

impl SectionHandler {
    pub fn new(section: String) -> SectionHandler {
        SectionHandler {
            section,
            cases: HashMap::new()
        }
    }
}

pub fn to_html(arena: &Arena, format: HtmlFormat) -> String {
    match format {
        HtmlFormat::Html5() => generate_html5(arena),
        HtmlFormat::Html4() => generate_html4(arena),
        HtmlFormat::XHtml() => generate_xhtml(arena),
        _ => generate_html5(arena),
    }
}

fn generate_html5(arena: &Arena) -> String {
    let mut special_cases = HashMap::new();
    special_cases.insert("input".to_string(), html5_input as Handler);
    node_to_html(0, arena, &special_cases)
}

fn html5_input(element: &HtmlElement) -> String {
    String::new()
}


fn html5_attributes(ele: &HtmlElement) -> String {
    // let mut attribute_builder = String::new();
    // for key in sort(ele.attributes().raw()) {
    //     if let Some(ref value) = ele.attributes().raw().get(&key) {
    //         let attribute = match key {
    //             "checked" => " checked",
    //             _ => &format!(" {}=\'{}\'", key, value.join(" ")),
    //         };
    //         attribute_builder.push_str(&format!(" {}=\'{}\'", key, value.join(" ")));
    //     }
    // }
    // attribute_builder
    String::new()
}
fn generate_html4(arena: &Arena) -> String {
    node_to_html(0, arena, &HashMap::new())
}

fn generate_xhtml(arena: &Arena) -> String {
    node_to_html(0, arena, &HashMap::new())
}

fn generate_attributes_html(ele: &HtmlElement) -> String {
    let mut attribute_builder = String::new();
    for key in sort(ele.attributes().raw()) {
        if let Some(ref value) = ele.attributes().raw().get(&key) {
            attribute_builder.push_str(&format!(" {}=\'{}\'", key, value.join(" ")));
        }
    }
    attribute_builder
}

fn sort(map: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut v = vec![];
    for key in map.keys() {
        v.push(key.clone());
    }
    v.sort();
    v
}

fn node_to_html(id: usize, arena: &Arena, special_cases: &HashMap<String, Handler>) -> String {
    let mut html_builder = String::new();
    let node = arena.node_at(id);
    match &node.data {
        Html::Element(ref ele) => {
            if let Some(handler) = special_cases.get(ele.tag()) {
                html_builder.push_str(&handler(ele));
            } else {
            html_builder.push_str(&format!("<{}", ele.tag()));
            let attribute_html = generate_attributes_html(ele);
            html_builder.push_str(&attribute_html);
            if ele.body.is_empty() && common::is_void_tag(&ele.tag) {
                html_builder.push_str(" />");
            } else {
                html_builder.push('>');

                if !&ele.body.is_empty() {
                    html_builder.push_str(&ele.body);
                }
                for child_id in node.children() {
                    html_builder.push_str(&format!("{}", node_to_html(*child_id, arena, special_cases)));
                }
                match common::does_tag_close(&ele.tag) {
                    true => html_builder.push_str(&format!("</{}>", ele.tag())),
                    false => html_builder.push('>'),
                }
            }
            }
        }
        Html::Doctype(ref doctype) => {
            html_builder.push_str(&format!("{}", doctype_lookup(doctype)))
        }
        Html::Comment(ref comment) => {
            let mut comment = comment.to_string();
            if !comment.ends_with("\n") {
                comment.push(' ');
            }
            html_builder.push_str(&format!("<!--{}-->", comment))
        }
        Html::Css(ref css) => {
            html_builder.push_str(&format!("<style>{}</style>\n", css.text));
        }
        Html::Text(ref text) => html_builder.push_str(&format!("{}", text)),
        Html::SilentComment(_comment) => (),
    }
    if id == 0 {
        if let Some(sibling_id) = node.next_sibling() {
            html_builder.push_str(&format!("{}", node_to_html(sibling_id, arena, special_cases)));
        }
    }

    html_builder
}

fn doctype_lookup(doctype: &str) -> String {
    match doctype {
                    "strict" => 
                    "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Strict//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd\">".to_string(),
                    "frameset" =>
                    "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Frameset//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd\">".to_string(),
                    "5" =>
                    "<!DOCTYPE html>".to_string(),
                    "1.1" =>
                    "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.1//EN\" \"http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd\">".to_string(),
                    "basic" =>
                    "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML Basic 1.1//EN\" \"http://www.w3.org/TR/xhtml-basic/xhtml-basic11.dtd\">".to_string(),
                    "mobile" =>
                    "<!DOCTYPE html PUBLIC \"-//WAPFORUM//DTD XHTML Mobile 1.2//EN\" \"http://www.openmobilealliance.org/tech/DTD/xhtml-mobile12.dtd\">".to_string(),
                    _ => {
                        "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">".to_string()
                    }
                }
}
