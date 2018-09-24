#[macro_use]
extern crate serde_json;
mod headers;

#[test]
fn test_all() -> () {
    let json = include_str!("tests.json");
    let tests = json!(&json);
    headers::test_headers()
}