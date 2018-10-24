#[macro_use]
extern crate haml;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

use serde_json::{Error, Value};

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
    tests.run_test_by_name("a tag with '<' appended");
    Ok(())
}

/*
 * This is used for testing all tests that have previously been fixed. This is
 * a regression of sorts so we make sure new fixes don't break previously fixed tests
 */
#[test]
fn completed() -> Result<(), Error> {
    let json = include_str!("tests.json");
    let tests: Tests = serde_json::from_str(&json)?;
    // Comments
    tests.run_test_by_name("a nested markup comment nested markup comment");
    tests.run_test_by_name("an inline markup comment");

    // Text
    tests.run_test_by_name("inside a textarea tag");
    Ok(())
}
