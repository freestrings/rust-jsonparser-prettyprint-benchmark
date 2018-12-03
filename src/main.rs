extern crate rjq;

use std::env;
use std::error::Error;
use std::io::{self, Read};

fn main() -> std::result::Result<(), String> {
    let buffer = read_buffer().expect("입력에러");

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && &args[1] == "pest" {
        match rjq::pest_json::pretty::pretty_print(buffer.as_str()) {
            Ok(()) => Ok(()),
            Err(e) => Err(e.description().to_owned())
        }
    } else if args.len() == 2 && &args[1] == "serde" {
        match rjq::rust_de::pretty::pretty_print(buffer.as_str()) {
            Ok(()) => Ok(()),
            Err(e) => Err(e.description().to_owned())
        }
    } else {
        Ok(())
    }
}

fn read_buffer() -> std::result::Result<String, String> {
    let mut buffer = String::new();

    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e.description().to_owned())
    }
}