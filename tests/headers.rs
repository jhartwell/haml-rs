extern crate haml;
#[macro_use]
extern crate serde_json;
mod common;

use serde_json::{Value, Error}; 

fn test_headers(json: Value) -> Result<(), Error> {
    let headers = &json["headers"];
    test_headers_an_XHTML_XML_prolog(headers)?;
    test_headers_an_XHTML_default_transitional_doctype(headers)?;
    Ok(())
}

fn test_headers_an_XHTML_XML_prolog(json: &Value) -> Result<(), Error> {
    Ok(())
}

fn test_headers_an_XHTML_default_transitional_doctype(json: &Value) -> Result<(), Error> {
    Ok(())
}