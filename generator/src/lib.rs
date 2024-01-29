use clap::Parser;
use nanorand::{rand::ChaCha20, Rng};

/// Generate a map
#[derive(Parser)]
pub struct MapGenerator {
    /// Width of the map
    x: u32,

    /// Height of the map
    y: u32,

    /// Density of walls
    density: u32,
}

impl MapGenerator {
    pub fn generate_map(self) -> String {
        let mut rand = ChaCha20::new();
        let mut string = String::new();

        // Height of the map
        string.push_str(&format!("{}\n", self.y));

        for _ in 0..self.y {
            for _ in 0..self.x {
                // Generate a random number between 0 and the height of the map
                let random_value = rand.generate_range(0..=self.y);

                // Decide to insert a wall or not
                if random_value * 2 < self.density {
                    string.push('o');
                } else {
                    string.push('.');
                }
            }
            string.push('\n');
        }

        string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_line() {
        let map = MapGenerator {
            x: 10,
            y: 10,
            density: 0,
        }
        .generate_map();
        let mut map_iter = map.lines();

        // First line contains the height of the map
        let first_line = map_iter.next().expect("Failed to get first line");
        assert_eq!(first_line, "10");
    }

    #[test]
    fn number_lines() {
        let map = MapGenerator {
            x: 10,
            y: 10,
            density: 0,
        }
        .generate_map();
        let mut map_iter = map.lines();

        // First line contains the height of the map
        let first_line = map_iter
            .next()
            .expect("Failed to get first line")
            .parse::<u32>()
            .unwrap();

        // Count number of lines
        let mut lines = 0;
        map_iter.for_each(|_line| lines += 1);

        assert_eq!(first_line, lines);
    }

    #[test]
    fn number_columns() {
        let map = MapGenerator {
            x: 10,
            y: 10,
            density: 0,
        }
        .generate_map();
        let mut map_iter = map.lines();

        // Get second line, skip first one
        map_iter.next().expect("Failed to get first line");
        let second_line = map_iter.next().expect("Failed to get second line");

        // Get number of characters
        let len = second_line.len();

        // Make sure the rest of the lines have the same length
        assert!(map_iter.all(|line| line.len() == len));
    }
}
