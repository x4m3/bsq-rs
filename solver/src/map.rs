use std::{fmt::Display, io::BufRead};

#[derive(Debug, Clone, PartialEq)]
pub enum PointKind {
    Obstacle,
    FreeSpace(u32),
    Square,
}

#[derive(Debug, Clone, PartialEq)]
struct Coordinates {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    board: Vec<Vec<PointKind>>,
    biggest_square: Option<PointKind>,
    biggest_square_corner_coords: Option<Coordinates>,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ParseError {
    #[error("Invalid line length")]
    LineLength,

    #[error("Invalid character")]
    Character,

    #[error("Unexpected number of lines")]
    NumberLines,
}

impl Map {
    pub fn parse(
        file: std::io::BufReader<std::fs::File>,
        number_lines_on_board: u32,
    ) -> Result<Self, ParseError> {
        let mut map = Vec::new();
        let mut number_seen_lines: u32 = 0;
        let mut length_previous_line: Option<usize> = None;

        // Process line by line
        // While there are lines to read
        let mut iter = file.lines();
        while let Some(Ok(line)) = iter.next() {
            let length_current_line = line.len();

            match length_previous_line {
                Some(len) => {
                    // Make sure the length of this line is the same as the previous one
                    if len != length_current_line {
                        return Err(ParseError::LineLength);
                    }
                }
                None => {
                    // Set initial value
                    length_previous_line = Some(length_current_line);
                }
            }

            let mut board_line = Vec::new();
            for character in line.chars() {
                let point = match character {
                    'o' => PointKind::Obstacle,
                    '.' => PointKind::FreeSpace(1),
                    _ => return Err(ParseError::Character),
                };
                board_line.push(point);
            }
            map.push(board_line);

            // Count number of lines seen so far
            number_seen_lines += 1;
        }

        if number_seen_lines != number_lines_on_board {
            return Err(ParseError::NumberLines);
        }

        Ok(Map {
            board: map,
            biggest_square: None,
            biggest_square_corner_coords: None,
        })
    }

    pub fn compute_biggest_square(&mut self) {
        // Keep a copy of the original board
        let orig = self.board.clone();

        let mut biggest_value = 0;

        let mut line_iter = orig.iter().enumerate();

        // Skip first line
        line_iter.next();

        // Iterate over each line
        for (x, line) in line_iter {
            let mut column_iter = line.iter().enumerate();

            // Skip first character
            column_iter.next();

            for (y, cell) in column_iter {
                if let PointKind::FreeSpace(_value) = cell {
                    // Compute the minimum number of values surrounding the current value in square
                    //
                    // | top left | top     |
                    // | left     | current |
                    //
                    let left = &self.board[x][y - 1];
                    let top = &self.board[x - 1][y];
                    let top_left = &self.board[x - 1][y - 1];

                    let (left, top, top_left) = match (left, top, top_left) {
                        (
                            PointKind::FreeSpace(left),
                            PointKind::FreeSpace(top),
                            PointKind::FreeSpace(top_left),
                        ) => (left, top, top_left),
                        (_, _, _) => continue,
                    };

                    let min = std::cmp::min(left, top);
                    let min = std::cmp::min(min, top_left);
                    let min = 1 + min;
                    self.board[x][y] = PointKind::FreeSpace(min);

                    if min > biggest_value {
                        biggest_value = min;
                    }
                }
            }
        }

        self.biggest_square = Some(PointKind::FreeSpace(biggest_value));
    }

    pub fn find_first_biggest_square_corner(&mut self) {
        for (x, line) in self.board.iter().enumerate() {
            for (y, cell) in line.iter().enumerate() {
                // We want to check if the cell is a free space and make sure we did find the biggest square
                if let (
                    PointKind::FreeSpace(cell_value),
                    Some(PointKind::FreeSpace(biggest_square)),
                ) = (cell, &self.biggest_square)
                {
                    if cell_value == biggest_square {
                        // Save the coordinates
                        self.biggest_square_corner_coords = Some(Coordinates { x, y });
                    }
                }
            }
        }
    }

    pub fn fill_square(&mut self) {
        let (start, end) = match (&self.biggest_square_corner_coords, &self.biggest_square) {
            (Some(end), Some(PointKind::FreeSpace(distance))) => {
                // Compute location of the opposite corner of biggest square
                //
                //   v~~~~~~~~~~~~~~~~ opposite corner (start)
                // | z | y | y | y |
                // | y | y | y | y |
                // | y | y | y | y |
                // | y | y | y | x |
                //               ^~~~~ biggest corner (end)
                //
                let start = Coordinates {
                    x: end.x - (*distance as usize - 1),
                    y: end.y - (*distance as usize - 1),
                };

                (start, end)
            }
            (_, _) => return,
        };

        for x in start.x..=end.x {
            for y in start.y..=end.y {
                if let PointKind::FreeSpace(_) = &self.board[x][y] {
                    self.board[x][y] = PointKind::Square;
                }
            }
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.board.iter().peekable();
        while let Some(line) = iter.next() {
            for cell in line {
                write!(
                    f,
                    "{}",
                    match cell {
                        PointKind::Obstacle => "o",
                        PointKind::FreeSpace(_) => ".",
                        PointKind::Square => "x",
                    }
                )?;
            }

            // Add '\n' if we are not at the last line
            if iter.peek().is_some() {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn invalid_number_lines() {
        // Create temporary file
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-map.txt");
        let mut file = std::fs::File::create(&file_path).unwrap();
        writeln!(file, "10").unwrap();
        writeln!(file).unwrap();
        writeln!(file).unwrap();
        writeln!(file).unwrap();

        let filebuff = super::super::open_read_file(&file_path).unwrap();

        let map = Map::parse(filebuff.0, filebuff.1);
        assert_eq!(map, Err(ParseError::NumberLines));
    }

    #[test]
    fn invalid_characters() {
        // Create temporary file
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-map.txt");
        let mut file = std::fs::File::create(&file_path).unwrap();
        writeln!(file, "10").unwrap();
        writeln!(file, "these characters are not valid").unwrap();
        writeln!(file, "these characters are not valid").unwrap();
        writeln!(file, "these characters are not valid").unwrap();

        let filebuff = super::super::open_read_file(&file_path).unwrap();

        let map = Map::parse(filebuff.0, filebuff.1);
        assert_eq!(map, Err(ParseError::Character));
    }

    #[test]
    fn invalid_line_length() {
        // Create temporary file
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-map.txt");
        let mut file = std::fs::File::create(&file_path).unwrap();
        writeln!(file, "10").unwrap();
        writeln!(file, "...").unwrap();
        writeln!(file, "oo").unwrap();
        writeln!(file, "...").unwrap();

        let filebuff = super::super::open_read_file(&file_path).unwrap();

        let map = Map::parse(filebuff.0, filebuff.1);
        assert_eq!(map, Err(ParseError::LineLength));
    }
}
