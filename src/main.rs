
extern crate nix;
extern crate rusqlite;

pub mod converter;
pub mod db_interface;
pub mod file_parser;

use converter::generate_csv_from_fit;
use file_parser::parse_file;

fn main() {
    let input: Vec<String> = std::env::args().collect();

    generate_csv_from_fit(&input[1]);
    parse_file(&input[1], &input[2]);
}
