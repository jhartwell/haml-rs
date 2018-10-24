use haml;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
pub type Tests = HashMap<String, HashMap<String, Test>>;

pub trait TestCollection {
    fn run(&self);
    fn run_test_by_name(&self, name: &str);
}

impl TestCollection for Tests {
    fn run(&self) {
        for (_, value) in self {
            for (name, test) in value {
                test.run(name);
            }
        }
    }

    fn run_test_by_name(&self, name: &str) {
        for (_, value) in self {
            for (test_name, test) in value {
                if name == test_name {
                    test.run(name);
                }
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
    pub fn run(&self, name: &str) {
        match self.optional {
            Some(true) => return,
            _ => {
                let config = match self.config {
                    Some(ref config) => match config.format {
                        Some(ref format) => format.to_string(),
                        _ => "".to_string(),
                    },
                    _ => "".to_string(),
                };
                println!("format: {}, section: {}, haml: {}", config, name, self.haml);
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
