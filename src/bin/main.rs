extern crate haml;
#[macro_use]
extern crate clap;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use clap::App;

fn main() {
    let yml = load_yaml!("hamlrs.yml");
    let m = App::from_yaml(yml).get_matches();

    if let Some(matches) = m.subcommand_matches("ast") {
        match matches.value_of("input") {
            Some(input) => {

            },
            None => println!("Input file is required when printing AST.")
       }
    } else {
        if m.is_present("input") && m.is_present("output") {

        } else {
            println!("Input and output file required.");
        }
    }
    // let args: Vec<String> = env::args().collect();
    // if args.len() >= 3 {
    //     let input_file = &args[1];
    //     let output_file = &args[2];
    //     let haml = read_input(input_file);
    //     let html = haml::to_html(&haml);
    //     if let Err(err) = write_file(output_file, &html) {
    //         println!("Error writing output: {:?}", err);
    //     }
    // } else {
    //     println!("Incorrect usage. Please see below for correct usage.\n\thamlrs input output\n");
    // }
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
