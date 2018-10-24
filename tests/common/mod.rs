use haml;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
pub type Tests = HashMap<String, HashMap<String, Test>>;

pub trait TestCollection {
    fn run(&self);
}

impl TestCollection for Tests {
    fn run(&self) {
        for (_, value) in self {
            for (_, test) in value {
                test.run();
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pub config: Option<Config>,
    pub haml: String,
    pub html: String,
    pub optional: Option<bool>,
}

impl Test {
    pub fn run(&self) {
        match self.optional {
            Some(true) => return,
            _ => {
                let actual_html = haml::to_html(&self.haml);
                assert_eq!(self.html, actual_html);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub format: Option<String>,
    pub escape_html: Option<String>,
}

#[cfg(target_os = "windows")]
pub fn newline() -> &'static str {
    "\r\n"
}

#[cfg(not(target_os = "windows"))]
pub fn newline() -> &'static str {
    "\n"
}
