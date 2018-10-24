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
fn test_all() -> Result<(), Error> {
    let json = include_str!("tests.json");
    let tests: Tests = serde_json::from_str(&json)?;
    tests.run();
    Ok(())
}