extern crate clap;
extern crate haml;

use clap::{App, Arg, SubCommand};
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let m = App::new("Hamlrs")
        .version("0.2.1")
        .author("Jon Hartwell <jon@dontbreakthebuild.com>")
        .about("Convert Haml to HTML")
        .arg(Arg::with_name("INPUT").index(1))
        .arg(Arg::with_name("OUTPUT").index(2))
        .subcommand(
            SubCommand::with_name("ast")
                .about("Print the AST of the given Haml file")
                .version("0.2.1")
                .arg(Arg::with_name("INPUT").index(1)),
        )
        .get_matches();

    if let Some(matches) = m.subcommand_matches("ast") {
        match matches.value_of("INPUT") {
            Some(input) => {
                let contents = read_input(input);
                println!("{}", haml::to_ast(&contents));
            }
            None => println!("Input file is required when printing AST."),
        }
    } else {
        if m.is_present("INPUT") && m.is_present("OUTPUT") {
            let input_file = m.value_of("INPUT").unwrap();
            let output_file = m.value_of("OUTPUT").unwrap();
            let haml = read_input(input_file);
            let html = haml::to_html(&haml);
            if let Err(err) = write_file(output_file, &html) {
                println!("Error writing output: {:?}", err);
            }
        } else {
            println!("Input and output file required.");
        }
    }
}

fn read_input(file_name: &str) -> String {
    let mut input_fs = File::open(file_name).expect("Input file not found.");
    let mut contents = String::new();
    input_fs
        .read_to_string(&mut contents)
        .expect("Could not successfully load input file.");
    contents
}

fn write_file(file_name: &str, contents: &str) -> Result<usize, io::Error> {
    let mut output_fs = File::create(file_name).expect("Could not create output file");
    output_fs.write(contents.as_bytes())
}
