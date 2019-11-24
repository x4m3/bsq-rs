extern crate clap;
extern crate rand;

use clap::{App, Arg};
use rand::Rng;

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

/// Generates a map with a square and walls
/// 
/// # Arguments
/// 
/// * `x` - Width of the map
/// * `y` - Height of the map
/// * `density` - Density of walls
pub fn generate(x: u32, y: u32, density: u32) {
    let mut i = 0;
    let mut j;
    let mut rand = rand::thread_rng();

    println!("{}", y);

    while i < y {
        j = 0;
        while j < x {
            // gen_range is [low, high)
            if rand.gen_range(0, 101) <= density {
                print!("o");
            } else {
                print!(".");
            }
            j += 1;
        }
        println!();
        i += 1;
    }
}