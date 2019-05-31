extern crate haml;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_json::Error;

mod common;
use common::{TestCollection, Tests};

fn load_json() -> Result<Tests, Error> {
    let json = include_str!("tests.json");
    let tests: Tests = serde_json::from_str(&json)?;
    Ok(tests)
}

/*
 * Run all non-optional tests in the json file
 */
#[test]
fn all() -> Result<(), Error> {
    let tests = load_json()?;
    tests.run();
    Ok(())
}

/*
 * This is used for testing one specific test from the JSON file at a time.
 * Pass in the key for the test data to run_test_by_name and it will execute
 * that given test
 */
#[test]
fn single() -> Result<(), Error> {
    let tests = load_json()?;
    tests.run_test_by_name("boolean attribute with HTML");
    Ok(())
}

#[test]
fn completed() -> Result<(), Error> {
    let tests = load_json()?;
    tests.run_test_by_name("boolean attribute with HTML");
    tests.run_test_by_name("a nested markup comment nested markup comment");
    tests.run_test_by_name("Inline content multiple simple tags");
    tests.run_test_by_name("Inline content tag with CSS");
    tests.run_test_by_name("Inline content simple tag");
    tests.run_test_by_name("a class with dashes");
    tests.run_test_by_name("a class with underscores");
    tests.run_test_by_name("an all-numeric class");
    tests.run_test_by_name("a tag with PascalCase");
    tests.run_test_by_name("Ruby-style attributes separated with newlines");
    tests.run_test_by_name("a tag with colons");
    tests.run_test_by_name("inside a pre tag");
    tests.run_test_by_name("a tag with underscores");
    tests.run_test_by_name("an inline markup comment");
    tests.run_test_by_name("a simple Haml tag");
    tests.run_test_by_name("a tag with a CSS class");
    tests.run_test_by_name("a tag with multiple CSS classes");
    tests.run_test_by_name("a tag with a CSS id");
    tests.run_test_by_name("a tag with multiple CSS id's");
    tests.run_test_by_name("a tag with a class followed by an id");
    tests.run_test_by_name("a tag with an id followed by a class");
    tests.run_test_by_name("an implicit div with a CSS id");
    tests.run_test_by_name("an implicit div with a CSS class");
    tests.run_test_by_name("multiple simple Haml tags");
    tests.run_test_by_name("a tag with dashes");
    tests.run_test_by_name("a tag with camelCase");
    tests.run_test_by_name("code following '&='");
    tests.run_test_by_name("an XHTML 1.1 basic doctype");
    Ok(())
}
// #[test]
// fn completed_nested_content() -> Result<(), Error> {
//     let tests = load_json()?;
//     tests.run_test_by_name("Nested content tag with CSS");
//     Ok(())
// }
// #[test]
// fn completed_comments() -> Result<(), Error> {
//     let tests = load_json()?;

//     tests.run_test_by_name("a nested markup comment nested markup comment");
//     tests.run_test_by_name("an inline markup comment");
//     tests.run_test_by_name("a multiply nested silent comment with inconsistent indents");

//     Ok(())
// }

// #[test]
// fn completed_text() -> Result<(), Error> {
//     let tests = load_json()?;

//     tests.run_test_by_name("inside a textarea tag");

//     Ok(())
// }

// #[test]
// fn completed_tags() -> Result<(), Error> {
//     let tests = load_json()?;

//     tests.run_test_by_name("a self-closing tag (XHTML)");
//     tests.run_test_by_name("a tag with multiple CSS classes");

//     Ok(())
// }

// #[test]
// fn completed_boolean_attributes() -> Result<(), Error> {
//     let tests = load_json()?;

//     tests.run_test_by_name("boolean attribute with HTML");
//     tests.run_test_by_name("boolean attribute with XHTML");
//     Ok(())
// }

// #[test]
// fn completed_html_style_attributes() -> Result<(), Error> {
//     let tests = load_json()?;

//     tests.run_test_by_name("HTML-style multiple attributes");
//     Ok(())
// }

// #[test]
// fn completed_filters() -> Result<(), Error> {
//     let tests = load_json()?;

//     tests.run_test_by_name("content in a 'css' filter (HTML)");

//     Ok(())
// }
