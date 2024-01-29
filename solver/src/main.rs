use clap::Parser;
use solver::Error;
use solver::Map;

#[derive(Parser)]
struct Args {
    /// Path to a file containing a map
    path: std::path::PathBuf,
}

fn main() -> Result<(), Error> {
    // Get cli arguments
    let args = Args::parse();

    // Open and read file
    let (file_reader, number_lines_on_board) = solver::open_read_file(args.path)?;

    // Parse map
    let mut map = Map::parse(file_reader, number_lines_on_board)?;

    // Solve it!
    solver::solve(&mut map);

    // Print map with square
    println!("{}", map);

    Ok(())
}
