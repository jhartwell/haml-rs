extern crate haml;
mod common;

fn test(input_file: &str, expected_output_file: &str) {
    let haml = &common::read_file(input_file);
    let html = common::read_file(expected_output_file);
    let actual_html = haml::to_html(haml);
    assert_eq!(html, actual_html);
}

fn test_debug(input_file: &str) {
    let haml = &common::read_file(input_file);
    let tokens = haml::tokenize(haml);
    println!("{:?}", tokens);
    assert!(false)
}

#[test]
fn test_basic_haml() {
    test("tests/inputs/basic.haml", "tests/outputs/basic.html");
}

#[test]
fn test_custom_elements() {
    test("tests/inputs/custom.haml", "tests/outputs/custom.html");
}

#[test]
fn test_ruby_attributes() {
    test(
        "tests/inputs/ruby_attributes.haml",
        "tests/outputs/ruby_attributes.html",
    );
}
