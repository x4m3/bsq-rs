extern crate clap;

use clap::{App, Arg};
use std:: {
    io,
    io::Read,
    fs,
    fs::File,
};

fn main() {
    let matches = App::new("bsq")
    .arg(Arg::with_name("path").required(true))
    .get_matches();

    let path = matches.value_of("path").unwrap();

    match check_path(path) {
        true => (),
        false => {
            eprintln!("error: no such file");
            ::std::process::exit(1);
        }
    }

    let file_data = read_file(path);
}

fn check_path(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn read_file(path: &str) -> Result<Vec<u8>, io::Error> {
    let mut data = Vec::new();

    File::open(path)?.read_to_end(&mut data)?;

    Ok(data)
}