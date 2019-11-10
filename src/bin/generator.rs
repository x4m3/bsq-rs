extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("generator")
    .arg(Arg::with_name("x").required(true))
    .arg(Arg::with_name("y").required(true))
    .arg(Arg::with_name("density").required(true))
    .get_matches();
   
    // get cli arguments and parse them to store as u32
    let x: u32 = matches.value_of("x").unwrap().parse().unwrap();
    let y: u32 = matches.value_of("y").unwrap().parse().unwrap();
    let density: u32 = matches.value_of("density").unwrap().parse().unwrap();

    generate(x, y, density);
}

fn generate(x: u32, y: u32, density: u32) {
}