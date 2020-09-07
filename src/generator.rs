use rand::Rng;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    x: u32,
    y: u32,
    density: u32,
}

fn main() {
    let opt = Opt::from_args();
    generate(opt.x, opt.y, opt.density);
}

/// Generates a map with a square and walls
///
/// # Arguments
///
/// * `x` - Width of the map
/// * `y` - Height of the map
/// * `density` - Density of walls
pub fn generate(x: u32, y: u32, density: u32) {
    let mut rand = rand::thread_rng();

    // print height of the map
    println!("{}", y);

    for _i in 0..y {
        for _j in 0..x {
            // gen_range is [low, high)
            if rand.gen_range(0, 101) <= density {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
