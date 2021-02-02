#![allow(dead_code)]

use std::env;
use std::fs::File;

mod tokens;
mod lexer;
mod parser;

fn main() {
    let _json_file =
        File::open(env::args().nth(1).expect("No filename passed!")).expect("Unable to open file!");
    println!("Hello, world!");
}
