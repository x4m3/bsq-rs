extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::Read;

fn main() {
    let matches = App::new("bsq")
    .arg(Arg::with_name("path").required(true))
    .get_matches();

    let path = matches.value_of("path").unwrap();
    let file_data = read_file(path);
}

fn read_file(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();

    file.read_to_end(&mut data)?;

    return Ok(data);
}