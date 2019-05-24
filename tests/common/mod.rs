use haml;
// use haml::HtmlFormat;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

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
    // fn get_format(&self) -> HtmlFormat {
    //     let default_format = HtmlFormat::Html5();
    //     match self.config {
    //         Some(ref config) => match config.format {
    //             Some(ref format) => match format.as_ref() {
    //                 "xhtml" => HtmlFormat::XHtml(),
    //                 "html" => HtmlFormat::Html4(),
    //                 "html5" => HtmlFormat::Html5(),
    //                 _ => default_format,
    //             },
    //             _ => default_format,
    //         },
    //         _ => default_format,
    //     }
    // }

    pub fn run(&self, name: &str) {
        println!("Running test: {}", name);
        match self.optional {
            Some(true) => return,
            _ => {
                //let format = self.get_format();
                // let actual_html = haml::to_html(&self.haml, format);
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
