#![feature(test)]

extern crate rjq;
extern crate test;

use std::io::Read;
use test::Bencher;

fn read_json(path: &str) -> String {
    let mut f = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    contents
}

fn bench<F, E>(b: &mut Bencher, contens: &str, f: F)
    where
        E: std::error::Error,
        F: Fn(&str) -> std::result::Result<(), E>
{
    b.iter(move || {
        match f(contens) {
            Ok(()) => Ok(()),
            Err(e) => Err(e.description().to_owned())
        }
    })
}

#[bench]
fn pretty_print_serde1(b: &mut Bencher) {
    let contents = read_json("./benches/data1.json");
    bench(b, contents.as_str(), rjq::rust_de::pretty::pretty_print);
}

#[bench]
fn pretty_print_serde2(b: &mut Bencher) {
    let contents = read_json("./benches/data2.json");
    bench(b, contents.as_str(), rjq::rust_de::pretty::pretty_print);
}

#[bench]
fn pretty_print_pest1(b: &mut Bencher) {
    let contents = read_json("./benches/data1.json");
    bench(b, contents.as_str(), rjq::pest_json::pretty::pretty_print);
}

#[bench]
fn pretty_print_pest2(b: &mut Bencher) {
    let contents = read_json("./benches/data2.json");
    bench(b, contents.as_str(), rjq::pest_json::pretty::pretty_print);
}