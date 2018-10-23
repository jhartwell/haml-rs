#[macro_use]
extern crate haml;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;
use serde_json::{Value, Error};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tests {
    headers: HashMap<String, Test>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pub config: Config,
    pub haml: String,
    pub html: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub format: String,
}

#[test]
fn test_all() -> Result<(), Error> {
     let json = include_str!("tests.json");
let tests: Tests = serde_json::from_str(&json)?;
    println!("{:?}", tests);
    panic!("failed");
    // match test_headers(tests) {
    //     Ok(()) => assert!(true),
    //     e => panic!(format!("{:?}", e)),
    // }
    Ok(())
}


fn test_headers(json: Value) -> Result<(), Error> {
    let headers = &json["headers"];
    println!("{:?}", headers);
    test_headers_an_XHTML_XML_prolog(headers)?;
    test_headers_an_XHTML_default_transitional_doctype(headers)?;
    Ok(())
}

fn test_headers_an_XHTML_XML_prolog(json: &Value) -> Result<(), Error> {
    let json_string = serde_json::to_string(json)?;
    assert_eq!(true, false);
    Ok(())
}

fn test_headers_an_XHTML_default_transitional_doctype(json: &Value) -> Result<(), Error> {
    Ok(())
}

// use serde_derive;
// mod headers;

// #[test]
// fn test_all() {
//     let json = include_str!("tests.json");
//     let tests = json!(&json);
//     headers::test_headers()
// }
