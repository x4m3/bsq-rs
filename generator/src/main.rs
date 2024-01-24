use clap::Parser;
use nanorand::{rand::ChaCha20, Rng};

/// Generate a map
#[derive(Parser)]
struct Args {
    /// Width of the map
    x: u32,

    /// Height of the map
    y: u32,

    /// Density of walls
    density: u32,
}

fn main() {
    let args = Args::parse();
    let mut rand = ChaCha20::new();

    // Print height of the map
    println!("{}", args.y);

    for _ in 0..args.y {
        for _ in 0..args.x {
            // Generate a random number between 0 and the height of the map
            let random_value = rand.generate_range(0..=args.y);

            // Decide to insert a wall or not
            if random_value * 2 < args.density {
                print!("o")
            } else {
                print!(".");
            }
        }
        println!();
    }
}
