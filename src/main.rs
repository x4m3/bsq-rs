extern crate clap;

use clap::{App, Arg};
use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let matches = App::new("bsq")
    .arg(Arg::with_name("path").required(true))
    .get_matches();

    let path = matches.value_of("path").unwrap();

    let file_data = read_file(path);
    println!("{:?}", file_data);
}

fn read_file(path: &str) -> Result<Vec<u8>, io::Error> {
    let mut data = Vec::new();

    File::open(path)?.read_to_end(&mut data)?;

    Ok(data)
}