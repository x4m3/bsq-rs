use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "map", parse(from_os_str))]
    path: PathBuf,
}

fn open_file(path: &PathBuf) -> Option<BufReader<File>> {
    // open file in read-only
    let file = match OpenOptions::new().read(true).open(path) {
        Ok(file) => file,
        Err(_) => return None,
    };
    Some(BufReader::new(file))
}

fn get_nb_lines_map(file: &mut BufReader<File>) -> Option<u32> {
    let mut first_line = String::new();
    file.read_line(&mut first_line).ok()?;

    // use trim to remove '\n'
    Some(first_line.trim().parse::<u32>().ok()?)
}

fn parse_map(file: BufReader<File>, nb_lines_in_map: u32) -> Option<Vec<Vec<u32>>> {
    let mut map = Vec::new();
    let mut nb_lines_counter: u32 = 0;
    let mut length_last_line: Option<usize> = None;

    // process line by line
    // while there is lines to read
    let mut iter = file.lines();
    while let Some(Ok(line)) = iter.next() {
        // count number of lines seen so far
        nb_lines_counter += 1;

        // if len line != previous line, return error
        let length_current_line = line.len();
        if let Some(len) = length_last_line {
            if len != length_current_line {
                eprintln!("length of current line differs from length of last line");
                return None;
            }
        }
        length_last_line = Some(length_current_line);

        let mut array: Vec<u32> = vec![0; length_current_line];
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

    // if at the end, nb lines != nb on first line, return error
    if nb_lines_in_map != nb_lines_counter {
        None
    } else {
        Some(map)
    }
}

fn main() {
    // get cli arguments
    let opt = Opt::from_args();

    // load file in string
    let mut file = match open_file(&opt.path) {
        Some(file) => file,
        None => {
            eprintln!("could not open file {:?}", opt.path);
            return;
        }
    };

    // read first line and parse number of lines on map
    let nb_lines_in_map = match get_nb_lines_map(&mut file) {
        Some(nb_lines) => nb_lines,
        None => {
            eprintln!("could not get number of lines in map. try again with a valid map file");
            return;
        }
    };
    if nb_lines_in_map == 0 {
        eprintln!("there should be at least one line to process. try again with a valid file");
        return;
    }
    let map = match parse_map(file, nb_lines_in_map) {
        Some(map) => map,
        None => {
            eprintln!("invalid map. try again with a valid file");
            return;
        }
    };

    for line in &map {
        println!("{:?}", line);
    }
}
