use haml;
// use haml::HtmlFormat;
use haml::Format;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Tests = HashMap<String, HashMap<String, Test>>;

pub trait TestCollection {
    fn run(&self);
    fn run_test_by_name(&self, name: &str);
    fn run_test_section(&self, name: &str);
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

    fn run_test_section(&self, name: &str) {
        if let Some(val) = self.get(name) {
            for (test_name, test) in val {
                test.run(test_name);
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
        println!("Running test: {}", name);
        println!("Input Haml:\n {}", self.haml);
        match self.optional {
            Some(true) => return,
            _ => {
                let mut format: Format = Format::Html5();
                if let Some(config) = &self.config {
                    if let Some(config_format) = &config.format {
                        match config_format.as_str() {
                            "xhtml" => format = Format::XHtml(),
                            "html4" => format = Format::Html4(),
                            "html5" => format = Format::Html5(),
                            _ => format = Format::Html5(),
                        }
                    }
                }
                println!("Format: {}", format);
                let actual_html = haml::to_html(&self.haml, &format);
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
