mod doctype;
pub mod element;

use crate::arena::Arena;
use crate::Format;


pub fn parse(haml: &str, format: Format) -> Arena {
    let mut arena = Arena::new();
    for (idx, ch) in haml.char_indices() {

    }
    arena
}
// mod doctype;
// pub mod element;

// use crate::arena::Arena;
// use crate::formatter::html4_formatter::Html4Formatter;
// use crate::formatter::html5_formatter::Html5Formatter;
// use crate::formatter::xhtml_formatter::XHtmlFormatter;
// use crate::formatter::xml_formatter::XmlFormatter;
// use crate::formatter::HtmlFormatter;
// use crate::regex::{
//     conditional_comment, prolog, sanitize, silent_comment, COMMENT_REGEX, TEXT_REGEX,
// };
// use crate::Format;
// use doctype::Doctype;
// use element::{Element, ElementType};
// use regex::Regex;
// use std::collections::HashMap;

// fn sanitize_html(line: &str) -> Option<String> {
//     let r = Regex::new(&sanitize()).unwrap();
//     match r.is_match(line) {
//         true => {
//             let caps = r.captures(line).unwrap();
//             match caps.name("text") {
//                 Some(m) => Some(
//                     m.as_str()
//                         .replace("&", "&amp;")
//                         .replace("<", "&lt;")
//                         .replace(">", "&gt;")
//                         .replace("'", "&apos;")
//                         .replace("\"", "&quot;")
//                         .to_owned(),
//                 ),
//                 None => None,
//             }
//         }
//         false => None,
//     }
// }

// fn text_from_string(line: &str) -> Option<String> {
//     let r = Regex::new(TEXT_REGEX).unwrap();
//     match r.captures(line) {
//         Some(m) => match m.name("text") {
//             Some(n) => Some(n.as_str().to_owned()),
//             None => None,
//         },
//         None => None,
//     }
// }

// fn comment(line: &str) -> Option<String> {
//     let r = Regex::new(COMMENT_REGEX).unwrap();
//     match r.is_match(line) {
//         true => {
//             let caps = r.captures(line).unwrap();
//             if let Some(c) = caps.name("comment") {
//                 Some(c.as_str().to_owned())
//             } else {
//                 None
//             }
//         }
//         false => None,
//     }
// }

// fn silent(line: &str) -> Option<Haml> {
//     let r = Regex::new(&silent_comment()).unwrap();
//     match r.captures(line) {
//         Some(m) => match m.name("ws") {
//             Some(ws) => Some(Haml::SilentComment(ws.as_str().len())),
//             None => Some(Haml::SilentComment(0)),
//         },
//         None => None,
//     }
// }

// fn conditional(line: &str) -> Option<Haml> {
//     let r = Regex::new(&conditional_comment()).unwrap();
//     let mut whitespace = 0;
//     let mut value = String::new();
//     match r.captures(line) {
//         Some(m) => {
//             match m.name("ws") {
//                 Some(ws) => whitespace = ws.as_str().len(),
//                 None => whitespace = 0,
//             }
//             match m.name("val") {
//                 Some(val) => value = val.as_str().to_string(),
//                 None => (),
//             }
//             Some(Haml::ConditionalComment(whitespace, value))
//         }
//         None => None,
//     }
// }

// #[derive(Clone, Debug, PartialEq)]
// pub enum Haml {
//     Root(),
//     Element(Element),
//     Text(String),
//     InnerText(String),
//     Comment(String),
//     Prolog(Option<String>),
//     SilentComment(usize),
//     ConditionalComment(usize, String),
// }

// pub struct Parser<'a> {
//     arena: Arena,
//     format: &'a Format,
// }

// impl<'a> Parser<'a> {
//     pub fn new(format: &'a Format) -> Parser {
//         Parser {
//             arena: Arena::new(),
//             format,
//         }
//     }

//     pub fn parse(&mut self, haml: &str) -> &Arena {
//         let mut previous_id = 0;
//         let mut first_line = true;
//         let prolog_regex = Regex::new(&prolog()).unwrap();
//         for line in haml.lines() {
//             println!("Hi: {}", line);
//                 // matches lines that start with &=
//             if let Some(sanitized_html) = sanitize_html(line) {
//                 self.arena.insert(Haml::Text(sanitized_html), previous_id);
//                 first_line = false;
//             } else if let Some(sc) = silent(line) {
//                 previous_id = self.arena.insert(sc, previous_id);
//                 first_line = false;
//             } else if let Some(cc) = conditional(line) {
//                 previous_id = self.arena.insert(cc, previous_id);
//                 first_line = false;
//             } else if prolog_regex.is_match(line) {
//                 first_line = false;
//                 let caps = prolog_regex.captures(line).unwrap();
//                 let value = match caps.name("type") {
//                     Some(m) => match m.as_str() {
//                         "" => None,
//                         val => Some(val.to_string()),
//                     },
//                     None => None,
//                 };
//                 self.arena.insert(Haml::Prolog(value), previous_id);
//             } else if let Some(el) = Element::from_string(line) {
//                 let ws = el.whitespace;
//                 let element = Haml::Element(el);
                
//                 if !first_line {
//                     let p_id = self.arena.from_whitespace(previous_id, ws);
//                     previous_id = self.arena.insert(element, p_id);
//                 } else {
//                     previous_id = self.arena.insert(element, 0);
//                     first_line = false;
//                 }
//             } else if let Some(comment) = comment(line) {
//                 previous_id = self.arena.insert(Haml::Comment(comment), previous_id);
//                 first_line = false;
//             } else if let Some(text_line) = text_from_string(line) {
//                 first_line = false;
//                 self.arena.insert(Haml::Text(text_line), previous_id);
//             }
//         }
//         &self.arena
//     }
// }

// // #[cfg(test)]
// // mod test {
// //     use super::*;

// //     #[test]
// //     fn parse_text() {
// //         let haml = r"\= test";
// //         let mut p = Parser::new();
// //         let e = p.parse(haml);
// //         let id = e.root().children[0];
// //         let item = e.item(id);
// //         match &item.value {
// //             Haml::Text(ref text) => assert_eq!("= test".to_owned(), *text),
// //             _ => panic!("failed"),
// //         }
// //     }

// //     #[test]
// //     fn parse_element_text() {
// //         let haml = "%hi\n\\value";
// //         let mut p = Parser::new();
// //         let e = p.parse(haml);
// //         let id = e.root().children[0];
// //         let item = e.item(id);
// //         if let Haml::Element(el) = &item.value {
// //             let mut it = item.children.iter();
// //             match it.next() {
// //                 Some(child_id) => {
// //                     let child = e.item(*child_id);
// //                     match &child.value {
// //                         Haml::Text(ref txt) => assert_eq!("value".to_owned(), *txt),
// //                         _ => panic!("Failed"),
// //                     }
// //                 },
// //                 None => panic!("Failed"),
// //             }
// //         }
// //     }

// //     #[test]
// //     fn parse_element() {
// //         let haml = "%hi\n  .box\n    #b\n  %span";
// //         let mut p = Parser::new();
// //         let e = p.parse(haml);
// //         let id = e.item(0).children[0];
// //         let item = e.item(id);
// //         let el = match &item.value {
// //             Haml::Element(el) => el,
// //             _ => panic!("failed"),
// //         };

// //         assert_eq!(Some("%hi".to_owned()), el.name);
// //         assert_eq!(ElementType::Other(), el.element_type);
// //         assert_eq!(0, el.whitespace);

// //         let mut it = item.children.iter();
// //         let b = it.next().unwrap();
// //         let bel = e.item(*b);
// //         let el2 = match &bel.value {
// //             Haml::Element(el) => el,
// //             _ => panic!("failed")
// //         };
// //         assert_eq!(Some(".box".to_owned()), el2.name);
// //         assert_eq!(ElementType::Div(), el2.element_type);
// //         assert_eq!(2, el2.whitespace);

// //         let mut it2 = bel.children.iter();
// //         let c = it2.next().unwrap();
// //         let cel = e.item(*c);
// //         let el3 = match &cel.value {
// //             Haml::Element(el) => el,
// //             _ => panic!("failed")
// //         };
// //         assert_eq!(Some("#b".to_owned()), el3.name);
// //         assert_eq!(ElementType::Div(), el3.element_type);
// //         assert_eq!(4, el3.whitespace);

// //         let mut d = it.next().unwrap();
// //         let del = e.item(*d);
// //         let el4 = match &del.value {
// //             Haml::Element(el) => el,
// //             _ => panic!("failed")
// //         };
// //         assert_eq!(Some("%span".to_owned()), el4.name);
// //         assert_eq!(ElementType::Other(), el4.element_type);
// //         assert_eq!(2, el4.whitespace);

// //     }
// // }
