use std::cmp::min;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "map", parse(from_os_str))]
    path: PathBuf,
}

type Map = Vec<Vec<i32>>;

fn open_file(path: &PathBuf) -> Option<BufReader<File>> {
    // Open file in read-only mode
    let file = match OpenOptions::new().read(true).open(path) {
        Ok(file) => file,
        Err(_) => return None,
    };
    Some(BufReader::new(file))
}

fn get_nb_lines_map(file: &mut BufReader<File>) -> Option<u32> {
    let mut first_line = String::new();
    file.read_line(&mut first_line).ok()?;

    // Remove '\n'
    Some(first_line.trim().parse::<u32>().ok()?)
}

fn parse_map(file: BufReader<File>, nb_lines_in_map: u32) -> Option<Map> {
    let mut map = Vec::new();
    let mut nb_lines_counter: u32 = 0;
    let mut length_last_line: Option<usize> = None;

    // Process line by line
    // While there are lines to read
    let mut iter = file.lines();
    while let Some(Ok(line)) = iter.next() {
        // Count number of lines seen so far
        nb_lines_counter += 1;

        // If len line != previous line, return error
        let length_current_line = line.len();
        if let Some(len) = length_last_line {
            if len != length_current_line {
                eprintln!("length of current line differs from length of last line");
                return None;
            }
        }
        length_last_line = Some(length_current_line);

        let mut array: Vec<i32> = vec![0; length_current_line];
        for (i, c) in line.chars().enumerate() {
            array[i] = match c {
                'o' => 0,
                '.' => 1,
                _ => {
                    eprintln!("invalid character in file");
                    return None;
                }
            };
        }
        map.push(array);
    }

    // If at the end, nb lines != nb on first line, return error
    if nb_lines_in_map != nb_lines_counter {
        None
    } else {
        Some(map)
    }
}

struct Point {
    x: usize,
    y: usize,
    value: u32,
}

fn compute_biggest_square_value(map: &mut Map) -> Option<u32> {
    // Clone original map to get reference, won't be needed after this function
    let orig_map = map.clone();

    let mut biggest_value = 0;

    // Iterate over each cell of the array
    for (x, line) in orig_map.iter().enumerate() {
        for (y, _cell) in line.iter().enumerate() {
            // Ignore first line and first column of each line
            // Compute only if value is not an obstacle
            if x != 0 && y != 0 && orig_map[x][y] > 0 {
                // Compute the minimum number of values surrounding the current value in square
                //
                // | x | x |
                // | x | _ |
                //
                // x -> values to compute
                // _ -> current value
                //
                map[x][y] = 1 + min(min(map[x][y - 1], map[x - 1][y]), map[x - 1][y - 1]);
            }

            // If current value is the biggest seen so far, keep it
            if map[x][y] > biggest_value {
                biggest_value = map[x][y];
            }
        }
    }

    if biggest_value == 0 {
        None
    } else {
        Some(biggest_value as u32)
    }
}

fn find_first_biggest_square(map: &Map, biggest_value: u32) -> Point {
    let mut biggest_square = Point {
        x: 0,
        y: 0,
        value: 0,
    };

    for (x, line) in map.iter().enumerate() {
        for (y, cell) in line.iter().enumerate() {
            if *cell as u32 == biggest_value {
                biggest_square.x = x;
                biggest_square.y = y;
                biggest_square.value = biggest_value;
                break;
            }
        }
    }
    biggest_square
}

fn make_square(map: &mut Map, biggest_corner: Point) {
    // Calculate location of the opposite corner of biggest square
    //
    //   v~~~~~~~~~~~~~~~~ opposite corner
    // | z | y | y | y |
    // | y | y | y | y |
    // | y | y | y | y |
    // | y | y | y | x |
    //               ^~~~~ biggest corner
    //
    let x = biggest_corner.x - (biggest_corner.value as usize - 1);
    let y = biggest_corner.y - (biggest_corner.value as usize - 1);

    for x in x..=biggest_corner.x {
        for y in y..=biggest_corner.y {
            map[x][y] = -1;
        }
    }
}

fn main() {
    // Get cli arguments
    let opt = Opt::from_args();

    // Load file in string
    let mut file = match open_file(&opt.path) {
        Some(file) => file,
        None => {
            eprintln!("could not open file {:?}", opt.path);
            return;
        }
    };

    // Read first line and parse number of lines on map
    let nb_lines_in_map = match get_nb_lines_map(&mut file) {
        Some(nb_lines) => {
            if nb_lines == 0 {
                eprintln!(
                    "there should be at least one line to process. try again with a valid file"
                );
                return;
            }
            nb_lines
        }
        None => {
            eprintln!("could not get number of lines in map. try again with a valid map file");
            return;
        }
    };

    // Parse map
    //
    // 'o' are obstacles, represented by 0
    // '.' are free space, represented by 1
    let mut map = match parse_map(file, nb_lines_in_map) {
        Some(map) => map,
        None => {
            eprintln!("invalid map. try again with a valid file");
            return;
        }
    };

    // Find biggest square corner value
    let biggest_square_corner_value = match compute_biggest_square_value(&mut map) {
        Some(biggest_square_corner_value) => biggest_square_corner_value,
        None => {
            eprintln!("found no squares");
            return;
        }
    };

    // Find first biggest square corner position
    // In case of multiple same big squares, locate and use the first one
    let biggest_square_corner = find_first_biggest_square(&map, biggest_square_corner_value);

    // Create biggest square on the map from the corner
    make_square(&mut map, biggest_square_corner);

    // Print map with square
    for line in &map {
        for cell in line {
            let c = match cell {
                -1 => 'x',
                0 => 'o',
                _ => '.',
            };
            print!("{}", c);
        }
        println!();
    }
}
