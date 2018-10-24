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

#[test]
fn all() -> Result<(), Error> {
    let json = include_str!("tests.json");
    let tests: Tests = serde_json::from_str(&json)?;
    tests.run();
    Ok(())
}

#[test]
fn single() -> Result<(),  Error> {
    let json = include_str!("tests.json");
    let tests: Tests = serde_json::from_str(&json)?;
    tests.run_test_by_name("a tag with '<' appended");
    Ok(())
}

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