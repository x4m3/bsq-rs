use std::path::PathBuf;
use std::{fs::OpenOptions, io::Read};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "map", parse(from_os_str))]
    path: PathBuf,
}

fn load_file(path: PathBuf) -> Option<String> {
    let mut contents = String::new();

    // open file in read-only
    let mut file = match OpenOptions::new().read(true).open(path) {
        Ok(file) => file,
        Err(_) => return None,
    };
    match file.read_to_string(&mut contents) {
        Ok(_) => Some(contents),
        Err(_) => None,
    }
}

fn main() {
    // get cli arguments
    let opt = Opt::from_args();

    // load file in string
    let file = match load_file(opt.path) {
        Some(file) => file,
        None => return,
    };

    // read first line and parse number of lines on map, can't parse -> error
    // read file, if at any moment any character != 'o' || '.' || '\n' -> error
}
