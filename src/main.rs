extern crate clap;

use clap::{App, Arg};
use std:: {
    io,
    io::Read,
    fs::File,
    path::Path,
};

fn main() {
    let matches = App::new("bsq")
    .arg(Arg::with_name("path").required(true))
    .get_matches();

    let path = matches.value_of("path").unwrap();

    match Path::new(path).is_file() {
        true => (),
        false => {
            eprintln!("error: could not open file {}", path);
            ::std::process::exit(1);
        }
    }

    let file_data = read_file(path);

}

fn read_file(path: &str) -> Result<Vec<u8>, io::Error> {
    let mut data = Vec::new();

    File::open(path)?.read_to_end(&mut data)?;

    Ok(data)
}