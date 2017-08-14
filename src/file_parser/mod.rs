
use db_interface::create_table;
use db_interface::insert_values;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn parse_file(src_filename: &str, dst_filename: &str) {

    create_table(dst_filename);

    let src_filename = src_filename.replace(".FIT", ".csv");
    let text = read_from_file(&src_filename);
    let split_newline = text.split("\n");
    let vec_newline = split_newline.collect::<Vec<&str>>();

    for line in 1..vec_newline.len() - 1 {
        if vec_newline[line].to_string().starts_with("Data") {
            let split_comma = vec_newline[line].split(",");
            let vec_comma = split_comma.collect::<Vec<&str>>();

            if vec_comma[2] == "record" {
                process_values(vec_comma, dst_filename);
            }
        }
    }
}

fn read_from_file(src_filename: &str) -> String {

    let path = Path::new(src_filename);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!(),
    };

    let mut text = String::new();
    let _reader = match file.read_to_string(&mut text) {
        Ok(string) => string,
        Err(_) => panic!(),
    };
    text
}

fn process_values(vec: Vec<&str>, dst_filename: &str) {
    insert_values(trim(vec[4]),
                  trim(vec[7]),
                  trim(vec[10]),
                  trim(vec[13]),
                  trim(vec[16]),
                  trim(vec[19]),
                  trim(vec[22]),
                  trim(vec[25]),
                  trim(vec[28]),
                  dst_filename);
}

fn trim(value: &str) -> String {
    value.replace("\"", "")
}
