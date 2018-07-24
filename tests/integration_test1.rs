extern crate haml;
mod common;

use haml::Haml;

#[test]
fn test_it_out() {
    let haml = &common::read_file("tests/basic.haml");
    let html = common::read_file("tests/basic.html");
    let actual_html = Haml::to_html(haml);
    assert_eq!(html, actual_html);
}