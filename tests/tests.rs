// extern crate haml;
// extern crate serde;
// extern crate serde_derive;
// extern crate serde_json;

// use serde_json::Error;

// mod common;
// use common::{TestCollection, Tests};

// fn load_json() -> Result<Tests, Error> {
//     let json = include_str!("tests.json");
//     let tests: Tests = serde_json::from_str(&json)?;
//     Ok(tests)
// }

// /*
//  * Run all non-optional tests in the json file
//  */
// #[test]
// fn all() -> Result<(), Error> {
//     let tests = load_json()?;
//     tests.run();
//     Ok(())
// }

// #[test]
// fn section() -> Result<(), Error> {
//     let tests = load_json()?;
//     tests.run_test_section("tags with HTML-style attributes");
//     tests.run_test_section("whitespace removal");
//     tests.run_test_section("conditional comments");
//     tests.run_test_section("whitespace preservation");
//     tests.run_test_section("boolean attributes");
//     tests.run_test_section("HTML escaping");
//     tests.run_test_section("Ruby-style interpolation");
//     tests.run_test_section("internal filters");

//     tests.run_test_section("markup comments");
//     tests.run_test_section("silent comments");
//     tests.run_test_section("tags with Ruby-style attributes");

//     tests.run_test_section("tags with nested content");
//     tests.run_test_section("tags with inline content");
//     tests.run_test_section("tags with unusual CSS identifiers");
//     tests.run_test_section("tags with unusual HTML characters");
//     tests.run_test_section("basic Haml tags and CSS");
//     tests.run_test_section("headers");
//     Ok(())
// }


// #[test]
// fn double() -> Result<(), Error> {
//     let tests = load_json()?;
//     tests.run_test_by_name("a tag with '>' appended and nested content");
//     tests.run_test_by_name("Inline content multiple simple tags");
//     Ok(())
// }

// /*
//  * This is used for testing one specific test from the JSON file at a time.
//  * Pass in the key for the test data to run_test_by_name and it will execute
//  * that given test
//  */
// #[test]
// fn single() -> Result<(), Error> {
//     let tests = load_json()?;
//     // tests.run_test_by_name("a self-closing tag (HTML4)");
//     tests.run_test_by_name("HTML-style attributes separated with newlines");
//     Ok(())
// }

// #[test]
// fn completed() -> Result<(), Error> {
//     let tests = load_json()?;
//     tests.run_test_by_name("HTML-style 'class' as an attribute");
//     tests.run_test_by_name("an HTML 4 frameset doctype");
//     tests.run_test_by_name("HTML-style tag with a CSS id and 'id' as an attribute");
//     tests.run_test_by_name("an HTML 5 XML prolog (silent)");
//     tests.run_test_by_name("an HTML 5 doctype");
//     tests.run_test_by_name("an XHTML 1.1 doctype");
//     tests.run_test_by_name("HTML-style multiple attributes");
//     tests.run_test_by_name("an XHTML default (transitional) doctype");
//     tests.run_test_by_name("HTML-style tag with an atomic attribute");
//     tests.run_test_by_name("boolean attribute with XHTML");
//     tests.run_test_by_name("a self-closing tag ('/' modifier + HTML5)");
//     tests.run_test_by_name("a class with underscores");
//     tests.run_test_by_name("inside a textarea tag");
//     tests.run_test_by_name("boolean attribute with HTML");
//     tests.run_test_by_name("a multiply nested silent comment");
//     tests.run_test_by_name("a nested markup comment nested markup comment");
//     tests.run_test_by_name("Inline content multiple simple tags");
//     tests.run_test_by_name("Inline content tag with CSS");
//     tests.run_test_by_name("Inline content simple tag");
//     tests.run_test_by_name("a class with dashes");
//     tests.run_test_by_name("a class with underscores");
//     tests.run_test_by_name("an all-numeric class");
//     tests.run_test_by_name("a tag with PascalCase");
//     tests.run_test_by_name("Ruby-style attributes separated with newlines");
//     tests.run_test_by_name("a tag with colons");
//     tests.run_test_by_name("inside a pre tag");
//     tests.run_test_by_name("a tag with underscores");
//     tests.run_test_by_name("an inline markup comment");
//     tests.run_test_by_name("a simple Haml tag");
//     tests.run_test_by_name("a tag with a CSS class");
//     tests.run_test_by_name("a tag with multiple CSS classes");
//     tests.run_test_by_name("a tag with a CSS id");
//     tests.run_test_by_name("a tag with multiple CSS id's");
//     tests.run_test_by_name("a tag with a class followed by an id");
//     tests.run_test_by_name("a tag with an id followed by a class");
//     tests.run_test_by_name("an implicit div with a CSS id");
//     tests.run_test_by_name("an implicit div with a CSS class");
//     tests.run_test_by_name("multiple simple Haml tags");
//     tests.run_test_by_name("a tag with dashes");
//     tests.run_test_by_name("a tag with camelCase");
//     tests.run_test_by_name("code following '&='");
//     tests.run_test_by_name("an XHTML 1.1 basic doctype");
//     Ok(())
// }
// // #[test]
// // fn completed_nested_content() -> Result<(), Error> {
// //     let tests = load_json()?;
// //     tests.run_test_by_name("Nested content tag with CSS");
// //     Ok(())
// // }
// // #[test]
// // fn completed_comments() -> Result<(), Error> {
// //     let tests = load_json()?;

// //     tests.run_test_by_name("a nested markup comment nested markup comment");
// //     tests.run_test_by_name("an inline markup comment");
// //     tests.run_test_by_name("a multiply nested silent comment with inconsistent indents");

// //     Ok(())
// // }

// // #[test]
// // fn completed_text() -> Result<(), Error> {
// //     let tests = load_json()?;

// //     tests.run_test_by_name("inside a textarea tag");

// //     Ok(())
// // }

// // #[test]
// // fn completed_tags() -> Result<(), Error> {
// //     let tests = load_json()?;

// //     tests.run_test_by_name("a self-closing tag (XHTML)");
// //     tests.run_test_by_name("a tag with multiple CSS classes");

// //     Ok(())
// // }

// // #[test]
// // fn completed_boolean_attributes() -> Result<(), Error> {
// //     let tests = load_json()?;

// //     tests.run_test_by_name("boolean attribute with HTML");
// //     tests.run_test_by_name("boolean attribute with XHTML");
// //     Ok(())
// // }

// // #[test]
// // fn completed_html_style_attributes() -> Result<(), Error> {
// //     let tests = load_json()?;

// //     tests.run_test_by_name("HTML-style multiple attributes");
// //     Ok(())
// // }

// // #[test]
// // fn completed_filters() -> Result<(), Error> {
// //     let tests = load_json()?;

// //     tests.run_test_by_name("content in a 'css' filter (HTML)");

// //     Ok(())
// // }
