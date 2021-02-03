#![allow(dead_code)]

use std::env;
use std::fs::File;

mod json;
mod lexer;
mod parser;
mod tokens;

fn main() {
    let _json_file =
        File::open(env::args().nth(1).expect("No filename passed!")).expect("Unable to open file!");
    println!("Hello, world!");
}
