extern crate haml;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use serde_json::Error;

mod common;
use common::{TestCollection, Tests};

/*
 * Run all non-optional tests in the json file
 */
#[test]
fn all() -> Result<(), Error> {
    let json = include_str!("tests.json");
    let tests: Tests = serde_json::from_str(&json)?;
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
    let json = include_str!("tests.json");
    let tests: Tests = serde_json::from_str(&json)?;
    tests.run_test_by_name("a tag with multiple CSS classes");
    Ok(())
}

#[test]
fn completed_comments() -> Result<(), Error> {
    let json = include_str!("tests.json");
    let tests: Tests = serde_json::from_str(&json)?;

    tests.run_test_by_name("a nested markup comment nested markup comment");
    tests.run_test_by_name("an inline markup comment");
    tests.run_test_by_name("a multiply nested silent comment with inconsistent indents");

    Ok(())
}

#[test]
fn completed_text() -> Result<(), Error> {
    let json = include_str!("tests.json");
    let tests: Tests = serde_json::from_str(&json)?;

    tests.run_test_by_name("inside a textarea tag");

    Ok(())
}

#[test]
fn completed_tags() -> Result<(), Error> {
    let json = include_str!("tests.json");
    let tests: Tests = serde_json::from_str(&json)?;

    tests.run_test_by_name("a self-closing tag (XHTML)");
    tests.run_test_by_name("a tag with multiple CSS classes");

    Ok(())
}

#[test]
fn completed_filters() -> Result<(), Error> {
    let json = include_str!("tests.json");
    let tests: Tests = serde_json::from_str(&json)?;

    tests.run_test_by_name("content in a 'css' filter (HTML)");

    Ok(())
}
