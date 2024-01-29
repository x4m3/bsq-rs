use std::path::Path;

#[test]
fn main() {
    let base_path = format!("{}/../assets/example_files", env!("CARGO_MANIFEST_DIR"));
    let assets = Path::new(&base_path);

    let mut maps = assets.to_path_buf();
    maps.push("maps");
    let maps = maps.read_dir().unwrap();

    for path in maps {
        let map = path.unwrap().path();
        let filename = map.file_name().unwrap();

        // Read, parse and solve map
        let (file_reader, number_lines_on_board) = solver::open_read_file(&map).unwrap();
        let mut map = solver::Map::parse(file_reader, number_lines_on_board).unwrap();
        solver::solve(&mut map);
        let solved = map.to_string();

        // Read solved file
        let mut solved_map = assets.to_path_buf();
        solved_map.push("solved");
        solved_map.push(filename);
        let solved_map = std::fs::read_to_string(solved_map).unwrap();

        // Show if there are differences
        let diff = solved != solved_map;
        println!(
            "diff match {}: {:?}",
            if diff { "KO" } else { "OK" },
            filename
        );
    }
}
