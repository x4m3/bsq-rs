use clap::Parser;
use generator::MapGenerator;

fn main() {
    println!("{}", MapGenerator::parse().generate_map());
}
