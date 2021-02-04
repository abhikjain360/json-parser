#![allow(dead_code)]

mod parser;
mod values;

use parser::Parser;
use values::Value;

fn main() {
    let map = Parser::new(
        r#"
{
    "name": "Mr. Json",
    "age": 19,
    "cars": ["bugatti", 3],
    "vers": 12.98,
    "oth": {
        "okay": true,
        "not_": null,
    },
}
"#,
    )
    .parse()
    .unwrap();

    for pair in map.iter() {
        println!("{}:\t{:?}", pair.0, pair.1);
    }
}
