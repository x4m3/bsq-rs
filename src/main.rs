use clap::{App, Arg};
use std:: {
    io,
    process,
    io::Read,
    fs::File,
};

fn main() {
    let matches = App::new("bsq")
    .arg(Arg::with_name("path").required(true))
    .get_matches();

    let path = matches.value_of("path").unwrap();
    let file_data = match read_file(path) {
        Ok(file_data) => (file_data),
        Err(e) => {
            eprintln!("could not open {}: {}", path, e);
            process::exit(1);
        }
    };
    println!("{:?}", file_data);
    let number_lines = count_lines(&file_data);
    println!("there is {} lines in this file", number_lines);
    let mut lines = file_data.lines();
    println!("{:?}", lines.next());
}

fn count_lines(file: &String) -> usize {
    file.as_bytes().iter().filter(|&&c| c == b'\n').count()
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut data = String::new();

    File::open(path)?.read_to_string(&mut data)?;

    Ok(data)
}