use clap::Parser;
use map::Map;
use std::io::BufRead;
use std::path::PathBuf;

mod map;

#[derive(Parser)]
struct Args {
    /// Path to a file containing a map
    path: PathBuf,
}

#[derive(Debug, thiserror::Error, PartialEq)]
enum Error {
    #[error("Failed to open file: {0}")]
    OpenFile(std::io::ErrorKind),

    #[error("Failed to read first line: {0}")]
    ReadFirstLine(std::io::ErrorKind),

    #[error("First line does not contain number of lines in board")]
    NumberLinesBoard,

    #[error("Board must contain at least one line")]
    NoLines,

    #[error("Failed to parse map: {0}")]
    Parse(#[from] map::ParseError),
}

fn main() -> Result<(), Error> {
    // Get cli arguments
    let args = Args::parse();

    // Open file in read-only mode, and attach a BufReader
    let file = std::fs::File::open(args.path).map_err(|e| Error::OpenFile(e.kind()))?;
    let mut file_reader = std::io::BufReader::new(file);

    // Read first line
    let mut first_line = String::new();
    if let Err(e) = file_reader.read_line(&mut first_line) {
        return Err(Error::ReadFirstLine(e.kind()));
    }

    // Remove '\n'
    let first_line = first_line.trim();

    // Parse string to a number
    let number_lines_on_board = first_line
        .parse::<u32>()
        .map_err(|_| Error::NumberLinesBoard)?;

    if number_lines_on_board == 0 {
        return Err(Error::NoLines);
    }

    // Parse map
    let mut map = Map::parse(file_reader, number_lines_on_board)?;

    // Find biggest square corner value
    map.compute_biggest_square();

    // Find first biggest square corner position
    // In case of multiple same big squares, locate and use the first one
    map.find_first_biggest_square_corner();

    // Create biggest square on the map from the corner
    map.fill_square();

    // Print map with square
    println!("{}", map);

    Ok(())
}
