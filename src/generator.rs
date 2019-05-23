use crate::parser::element::Element;
use crate::parser::{Arena, ArenaItem, Haml};
use std::collections::HashMap;

pub struct Generator<'a> {
    arena: &'a Arena,
}

fn arena_item_to_html(item: &ArenaItem) -> String {
    let mut html = String::new();
    let mut d : HashMap<String, String> = HashMap::new();
    if let Haml::Element(el) = &item.value {
        let name = el.name().unwrap();
        // let class_and_ids = el.class_and_ids.unwrap();
    }
    
    html
}

impl<'a> Generator<'a> {
    pub fn new(arena: &'a Arena) -> Generator {
        Generator {
            arena
        }
    }

    pub fn to_html(&self) -> String {
        let mut html = String::new();
        let root = self.arena.root();
        for child in root.children.iter() {
            if let Haml::Element(ref el) = &self.arena.item(*child).value {
                if let Some(name) = &el.name() {
                    html.push_str(&format!("<{}", name));
                    if let Some(class_and_ids) = &el.class_and_ids {
                        html.push_str(&format!("{}", class_and_ids));
                    }

                    for (k, v) in el.attributes().iter() {
                        html.push_str(&format!(" {}='{}'", k.trim(),v.join(" ").trim()));
                    }

                    html.push_str(">");

                    if let Some(text) = &el.inline_text {
                        html.push_str(&format!("{} ", text));
                    }
                    html.push_str(&format!("</{}>", name));
                }
            }
        }
        html
    }
}

// use super::HtmlFormat;
// use ast::{Arena, Html, HtmlElement, Node, ToHtml};
// use common;
// use std::collections::HashMap;
// use std::hash::Hash;

// type Handler = fn(&HtmlElement) -> String;

// #[derive(PartialEq, Eq, Hash)]
// pub enum SectionType {
//     Element(),
//     Attributes(),
//     Closing(),
// }

// enum GenerationType<'a> {
//     Arena(&'a Arena),
//     Element(&'a HtmlElement),
// }

// pub fn element_to_html(
//     ele: &HtmlElement,
//     special_cases: &HashMap<SectionType, HashMap<String, Handler>>,
// ) -> String {
//     let mut html_builder = String::new();
//     let mut handled = false;
//     if let Some(ref handlers) = special_cases.get(&SectionType::Element()) {
//         if let Some(special) = handlers.get(ele.tag()) {
//             handled = true;
//             html_builder.push_str(&special(ele));
//         }
//     }
//     if !handled {
//         html_builder.push_str(&format!("<{}", ele.tag()));
//         let mut attributes_handled = false;
//         let mut attributes_html = String::new();
//         if let Some(ref handlers) = special_cases.get(&SectionType::Attributes()) {
//             if let Some(special) = handlers.get(ele.tag()) {
//                 attributes_handled = true;
//                 attributes_html = special(ele);
//             }
//         }
//         if !attributes_handled {
//             attributes_html = generate_attributes_html(ele);
//         }
//         html_builder.push_str(&attributes_html);
//         let mut handled_closing = false;
//         if let Some(closing_handler) = special_cases.get(&SectionType::Closing()) {
//             if let Some(handler) = closing_handler.get(ele.tag()) {
//                 handled_closing = true;
//                 html_builder.push_str(&handler(ele));
//             }
//         }
//         if !handled_closing {
//             if ele.children.is_empty() && common::is_void_tag(&ele.tag) {
//                 html_builder.push_str(" />");
//             } else {
//                 html_builder.push('>');

//                 if !&ele.children.is_empty() {
//                     html_builder.push_str(&ele.body);
//                 }
//                 for child in &ele.children {
//                     match child {
//                         Html::Element(ref child) => {
//                             html_builder.push_str(&element_to_html(child, special_cases))
//                         }
//                         _ => (),
//                     }
//                 }
//                 match common::does_tag_close(&ele.tag) {
//                     true => html_builder.push_str(&format!("</{}>", ele.tag())),
//                     false => html_builder.push('>'),
//                 }
//             }
//         }
//     }
//     html_builder
// }
// pub fn to_html(arena: &Arena, format: HtmlFormat) -> String {
//     match format {
//         HtmlFormat::Html5() => generate_html5(GenerationType::Arena(arena)),
//         HtmlFormat::Html4() => generate_html4(GenerationType::Arena(arena)),
//         HtmlFormat::XHtml() => generate_xhtml(GenerationType::Arena(arena)),
//         _ => generate_html5(GenerationType::Arena(arena)),
//     }
// }

// fn generate_html5(generation_type: GenerationType) -> String {
//     let mut sections = HashMap::new();
//     let mut attribute_section_handler = HashMap::new();
//     attribute_section_handler.insert("input".to_string(), html5_input_attributes as Handler);
//     let mut closing_section_handler = HashMap::new();

//     closing_section_handler.insert("input".to_string(), html5_input_closing as Handler);
//     closing_section_handler.insert("p".to_string(), html5_p_closing as Handler);

//     sections.insert(SectionType::Attributes(), attribute_section_handler);
//     sections.insert(SectionType::Closing(), closing_section_handler);

//     match generation_type {
//         GenerationType::Arena(ref arena) => node_to_html(0, arena, &sections),
//         GenerationType::Element(ref element) => element_to_html(element, &sections),
//     }
// }

// fn html5_input_attributes(ele: &HtmlElement) -> String {
//     let mut attribute_builder = String::new();
//     for key in sort(ele.attributes().raw()) {
//         if let Some(ref value) = ele.attributes().raw().get(&key) {
//             let attribute = match &key[..] {
//                 "checked" => " checked".to_string(),
//                 _ => format!(" {}={}", key, value.join(" ")),
//             };
//             attribute_builder.push_str(&attribute);
//         }
//     }
//     attribute_builder
// }

// fn html5_p_closing(ele: &HtmlElement) -> String {
//     match ele.body.as_ref() {
//         "" => "></p>".to_string(),
//         _ => format!(">{}\n</p>", ele.body),
//     }
// }

// fn html5_input_closing(_ele: &HtmlElement) -> String {
//     ">".to_string()
// }

// fn xhtml_input_closing(_ele: &HtmlElement) -> String {
//     " />".to_string()
// }

// fn xhtml_input_attributes(ele: &HtmlElement) -> String {
//     let mut attribute_builder = String::new();
//     for key in sort(ele.attributes().raw()) {
//         if let Some(ref value) = ele.attributes().raw().get(&key) {
//             let attribute = match &key[..] {
//                 "checked" => format!(" checked='checked'"),
//                 _ => format!(" {}={}", key, value.join(" ")),
//             };
//             attribute_builder.push_str(&attribute);
//         }
//     }
//     attribute_builder
// }

// fn generate_html4(generation_type: GenerationType) -> String {
//     match generation_type {
//         GenerationType::Arena(arena) => node_to_html(0, arena, &HashMap::new()),
//         GenerationType::Element(element) => element_to_html(element, &HashMap::new()),
//     }
// }

// fn generate_xhtml(generation_type: GenerationType) -> String {
//     let mut sections = HashMap::new();
//     let mut attribute_section_handler = HashMap::new();
//     attribute_section_handler.insert("input".to_string(), xhtml_input_attributes as Handler);
//     sections.insert(SectionType::Attributes(), attribute_section_handler);

//     let mut closing_section_handler = HashMap::new();
//     closing_section_handler.insert("input".to_string(), xhtml_input_closing as Handler);
//     sections.insert(SectionType::Closing(), closing_section_handler);

//     match generation_type {
//         GenerationType::Arena(arena) => node_to_html(0, arena, &sections),
//         GenerationType::Element(element) => element_to_html(element, &sections),
//     }
// }

// fn generate_attributes_html(ele: &HtmlElement) -> String {
//     let mut attribute_builder = String::new();
//     for key in sort(ele.attributes().raw()) {
//         if let Some(ref value) = ele.attributes().raw().get(&key) {
//             attribute_builder.push_str(&format!(" {}='{}'", key, value.join(" ")));
//         }
//     }
//     attribute_builder
// }

// fn sort(map: &HashMap<String, Vec<String>>) -> Vec<String> {
//     let mut v = vec![];
//     for key in map.keys() {
//         v.push(key.clone());
//     }
//     v.sort();
//     v
// }

// fn node_to_html(
//     id: usize,
//     arena: &Arena,
//     special_cases: &HashMap<SectionType, HashMap<String, Handler>>,
// ) -> String {
//     let mut html_builder = String::new();
//     let node = arena.node_at(id);
//     match &node.data {
//         Html::Element(ref ele) => {
//             let mut handled = false;
//             if let Some(ref handlers) = special_cases.get(&SectionType::Element()) {
//                 if let Some(special) = handlers.get(ele.tag()) {
//                     handled = true;
//                     html_builder.push_str(&special(ele));
//                 }
//             }
//             if !handled {
//                 html_builder.push_str(&format!("<{}", ele.tag()));
//                 let mut attributes_handled = false;
//                 let mut attributes_html = String::new();
//                 if let Some(ref handlers) = special_cases.get(&SectionType::Attributes()) {
//                     if let Some(special) = handlers.get(ele.tag()) {
//                         attributes_handled = true;
//                         attributes_html = special(ele);
//                     }
//                 }
//                 if !attributes_handled {
//                     attributes_html = generate_attributes_html(ele);
//                 }
//                 html_builder.push_str(&attributes_html);
//                 let mut handled_closing = false;
//                 if let Some(closing_handler) = special_cases.get(&SectionType::Closing()) {
//                     if let Some(handler) = closing_handler.get(ele.tag()) {
//                         handled_closing = true;
//                         html_builder.push_str(&handler(ele));
//                     }
//                 }
//                 if !handled_closing {
//                     if ele.children.is_empty() && common::is_void_tag(&ele.tag) {
//                         html_builder.push_str(" />");
//                     } else {
//                         html_builder.push('>');

//                         if !&ele.children.is_empty() {
//                             html_builder.push_str(&ele.body);
//                         }
//                         for child_id in node.children() {
//                             html_builder.push_str(&format!(
//                                 "{}",
//                                 node_to_html(*child_id, arena, special_cases)
//                             ));
//                         }
//                         match common::does_tag_close(&ele.tag) {
//                             true => html_builder.push_str(&format!("</{}>", ele.tag())),
//                             false => html_builder.push('>'),
//                         }
//                     }
//                 }
//             }
//         }
//         Html::Doctype(ref doctype) => {
//             html_builder.push_str(&format!("{}", doctype_lookup(doctype)))
//         }
//         Html::Comment(ref comment) => {
//             let mut comment = comment.to_string();
//             if !comment.ends_with("\n") {
//                 comment.push(' ');
//             }
//             html_builder.push_str(&format!("<!--{}-->", comment))
//         }
//         Html::Css(ref css) => {
//             html_builder.push_str(&format!("<style>{}</style>\n", css.text));
//         }
//         Html::Text(ref text) => html_builder.push_str(&format!("{}", text)),
//         Html::SilentComment(_comment) => (),
//     }
//     if id == 0 {
//         if let Some(sibling_id) = node.next_sibling() {
//             html_builder.push_str(&format!(
//                 "{}",
//                 node_to_html(sibling_id, arena, special_cases)
//             ));
//         }
//     }
//     html_builder
// }

// fn doctype_lookup(doctype: &str) -> String {
//     match doctype {
//                     "strict" => 
//                     "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Strict//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd\">".to_string(),
//                     "frameset" =>
//                     "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Frameset//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd\">".to_string(),
//                     "5" =>
//                     "<!DOCTYPE html>".to_string(),
//                     "1.1" =>
//                     "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.1//EN\" \"http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd\">".to_string(),
//                     "basic" =>
//                     "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML Basic 1.1//EN\" \"http://www.w3.org/TR/xhtml-basic/xhtml-basic11.dtd\">".to_string(),
//                     "mobile" =>
//                     "<!DOCTYPE html PUBLIC \"-//WAPFORUM//DTD XHTML Mobile 1.2//EN\" \"http://www.openmobilealliance.org/tech/DTD/xhtml-mobile12.dtd\">".to_string(),
//                     _ => {
//                         "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">".to_string()
//                     }
//                 }
// }
